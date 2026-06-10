use futures_util::stream::StreamExt;
use godot::prelude::*;
use twitch_oauth2::TwitchToken;

use crate::custom_config::hadalyth_twitch_eventsubs::TwitchApiEventSubs;
use crate::custom_events::socket_event::SocketEvent;
use crate::custom_events::twitch_event::TwitchEvent;
use crate::hadalyth_twitch::HadalythTwitch;

const TWITCH_EVENTSUB_WEBSOCKET_URL: &str = "wss://eventsub.wss.twitch.tv/ws";

// Handles calling and waiting for all the methods needed to init
pub async fn init_twitch_async(mut this: Gd<HadalythTwitch>) {
    this.call_deferred("_init_device_user_token", &[]);
    this.signals().device_user_token_status().to_future().await;

    this.call_deferred("_init_twitch_socket", &[]);
    this.signals().twitch_socket_status().to_future().await;

    this.call_deferred("_init_subscribe_eventsubs", &[]);
}

// Initializes the device code flow token authorization process
pub async fn init_device_user_token_async(
    tx: std::sync::mpsc::Sender<TwitchEvent>,
    http_client: reqwest::Client,
    mut token_builder: twitch_api::twitch_oauth2::DeviceUserTokenBuilder,
) {
    // Send the initial request

    let device_code_response = token_builder.start(&http_client).await;
    let Ok(device_code_response) = device_code_response else {
        let _ = tx.send(TwitchEvent::DeviceUserTokenStatus(None));
        let error = format!(
            "device_token_exchange_response bad {}",
            device_code_response.unwrap_err()
        );
        let _ = tx.send(TwitchEvent::Debug(error));
        return;
    };

    // Emit the parameters to the main thread and open the device default web browser

    let verification_uri = &device_code_response.verification_uri;
    let expires_in = device_code_response.expires_in;
    let _ = tx.send(TwitchEvent::DeviceUserTokenRequest(
        verification_uri.clone(),
        expires_in,
    ));

    // Wait for the response to be processed

    let device_token_exchange_response = token_builder
        .wait_for_code(&http_client, tokio::time::sleep)
        .await;
    let Ok(device_token_exchange_response) = device_token_exchange_response else {
        let _ = tx.send(TwitchEvent::DeviceUserTokenStatus(None));
        let _ = tx.send(TwitchEvent::Debug(
            "device token exchange response bad".to_string(),
        ));
        return;
    };

    let _ = tx.send(TwitchEvent::DeviceUserTokenStatus(Some(
        device_token_exchange_response,
    )));
}

// Initialize and maintain the socket
pub async fn init_twitch_socket_async(
    tx: std::sync::mpsc::Sender<TwitchEvent>,
    socket_rx: std::sync::mpsc::Receiver<SocketEvent>,
    http_client: reqwest::Client,
    mut token: twitch_api::twitch_oauth2::UserToken,
) {
    // Make a hashset of message ids that have already been received

    let mut message_ids = std::collections::HashSet::<String>::new();

    // Connect the initial socket

    let socket = tokio_tungstenite::connect_async(TWITCH_EVENTSUB_WEBSOCKET_URL).await;
    let Ok(socket) = socket else {
        let _ = tx.send(TwitchEvent::TwitchSocketStatus(None));
        return;
    };

    let mut socket_read = socket.0.split().1;

    // Pump the socket_read for any messages
    // and handle them immediately
    while let Some(message) = socket_read.next().await {
        while let Ok(socket_event) = socket_rx.try_recv() {
            match socket_event {
                SocketEvent::SocketClose => {
                    return;
                }
            }
        }

        // Error, assume socket is corrupted
        let Ok(message) = message else {
            let _ = tx.send(TwitchEvent::TwitchSocketStatus(None));
            let _ = tx.send(TwitchEvent::Debug("Invalid message received".to_string()));
            return;
        };

        match message {
            tokio_tungstenite::tungstenite::Message::Close(_) => {
                let _ = tx.send(TwitchEvent::TwitchSocketStatus(None));
                let _ = tx.send(TwitchEvent::Debug("Close frame received".to_string()));
                return;
            }
            tokio_tungstenite::tungstenite::Message::Frame(_) => {

                // Ignore
            }
            tokio_tungstenite::tungstenite::Message::Ping(_)
            | tokio_tungstenite::tungstenite::Message::Pong(_) => {
                // Refresh the token whenever a ping or a pong happens
                // https://github.com/twitch-rs/twitch_api/blob/main/examples/eventsub_websocket/src/websocket.rs

                let _ = token.refresh_token(&http_client).await;
            }
            tokio_tungstenite::tungstenite::Message::Binary(_) => {

                // Ignore
            }
            tokio_tungstenite::tungstenite::Message::Text(payload) => {
                // Actual payload from twitch, parse into an enum and execute
                let event_data = twitch_api::eventsub::event::Event::parse_websocket(&payload);
                let Ok(event_data) = event_data else {
                    // Assume there can be single corrupted packets without the stream completely failing
                    // Continue to the next message
                    continue;
                };

                // Check to see if we have already received the message
                let message_id = event_data.message_id();
                if message_ids.contains(&message_id.to_string()) {
                    continue;
                }
                message_ids.insert(message_id.to_string());

                // Match the event data
                match event_data {
                    twitch_api::eventsub::EventsubWebsocketData::Welcome {
                        metadata: _,
                        payload,
                    } => {
                        // Process welcome and emit the session id back into the main thread so event subs can be issued
                        let _ = tx.send(TwitchEvent::TwitchSocketStatus(Some(
                            payload.session.id.to_string(),
                        )));
                    }
                    twitch_api::eventsub::EventsubWebsocketData::Reconnect {
                        metadata: _,
                        payload,
                    } => {
                        // Get the reconnect url
                        // If it can not be retrieved, then close the socket
                        let reconnect_url = payload.session.reconnect_url;
                        let Some(reconnect_url) = reconnect_url else {
                            let _ = tx.send(TwitchEvent::TwitchSocketStatus(None));
                            let _ = tx.send(TwitchEvent::Debug(
                                "Reconnect with no valid reconnect url".to_string(),
                            ));
                            return;
                        };

                        // Replace the socket with a new one
                        let socket =
                            tokio_tungstenite::connect_async(reconnect_url.to_string()).await;
                        let Ok(socket) = socket else {
                            let _ = tx.send(TwitchEvent::TwitchSocketStatus(None));
                            let _ = tx.send(TwitchEvent::Debug("Reconnect failed".to_string()));
                            return;
                        };
                        socket_read = socket.0.split().1;
                        continue;
                    }
                    twitch_api::eventsub::EventsubWebsocketData::Keepalive {
                        metadata: _,
                        payload: _,
                    } => {

                        // TODO : Reset a timer somewhere
                    }
                    twitch_api::eventsub::EventsubWebsocketData::Revocation {
                        metadata: _,
                        payload: _,
                    } => {
                        // Socket was revoked for some reason
                        // Exit out and attempt to restart in godot

                        let _ = tx.send(TwitchEvent::TwitchSocketStatus(None));
                        let _ = tx.send(TwitchEvent::Debug("Revocation received".to_string()));
                        return;
                    }
                    twitch_api::eventsub::EventsubWebsocketData::Notification {
                        metadata: _,
                        payload,
                    } => {
                        // This is an actual eventsub notification
                        // Send it back to the main thread for parsing and routing
                        let _ = tx.send(TwitchEvent::TwitchEventsubReceived(payload));
                    }
                    _ => {

                        // Ignore
                    }
                }
            }
        }
    }
}

// Initialize the eventsubs with the session id of a valid socket
pub async fn init_subscribe_eventsubs_async(
    tx: std::sync::mpsc::Sender<TwitchEvent>,
    twitch: twitch_api::helix::HelixClient<'static, reqwest::Client>,
    token: twitch_api::twitch_oauth2::UserToken,
    session_id: String,
    eventsubs: TwitchApiEventSubs,
) {
    if eventsubs.automod_message_hold_v2 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::automod::AutomodMessageHoldV2::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.automod_message_update_v2 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::automod::AutomodMessageUpdateV2::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.automod_settings_update_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::automod::AutomodSettingsUpdateV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.automod_terms_update_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::automod::AutomodTermsUpdateV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_ad_break_begin_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelAdBreakBeginV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_bits_use_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelBitsUseV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_chat_clear_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatClearV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_chat_clear_user_messages_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatClearUserMessagesV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_chat_message_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatMessageV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_chat_message_delete_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatMessageDeleteV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_chat_notification_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatNotificationV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_chat_user_message_hold_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatUserMessageHoldV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_chat_user_message_update_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatUserMessageUpdateV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_chat_settings_update_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelChatSettingsUpdateV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_charity_campaign_donate_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelCharityCampaignDonateV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_charity_campaign_progress_v1 {
        // channel.bits.use
        let result = twitch.create_eventsub_subscription(
            twitch_api::eventsub::channel::ChannelCharityCampaignProgressV1::broadcaster_user_id(
                token.user_id.clone()
            ),
            twitch_api::eventsub::Transport::websocket(&session_id),
            &token.clone()
        ).await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_charity_campaign_start_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelCharityCampaignStartV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_charity_campaign_stop_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelCharityCampaignStopV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_update_v2 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelUpdateV2::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_follow_v2 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelFollowV2::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_subscribe_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSubscribeV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_cheer_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelCheerV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_ban_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelBanV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_unban_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelUnbanV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_unban_request_create_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelUnbanRequestCreateV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_unban_request_resolve_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelUnbanRequestResolveV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_vip_add_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelVipAddV1::new(token.user_id.clone()),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_vip_remove_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelVipRemoveV1::new(token.user_id.clone()),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_warning_acknowledge_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelWarningAcknowledgeV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_warning_send_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelWarningSendV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_points_automatic_reward_redemption_add_v1 {
        // channel.bits.use
        let result = twitch.create_eventsub_subscription(
            twitch_api::eventsub::channel::ChannelPointsAutomaticRewardRedemptionAddV1::broadcaster_user_id(
                token.user_id.clone()
            ),
            twitch_api::eventsub::Transport::websocket(&session_id),
            &token.clone()
        ).await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_points_custom_reward_add_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPointsCustomRewardAddV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_points_custom_reward_update_v1 {
        // channel.bits.use
        let result = twitch.create_eventsub_subscription(
            twitch_api::eventsub::channel::ChannelPointsCustomRewardUpdateV1::broadcaster_user_id(
                token.user_id.clone()
            ),
            twitch_api::eventsub::Transport::websocket(&session_id),
            &token.clone()
        ).await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_points_custom_reward_remove_v1 {
        // channel.bits.use
        let result = twitch.create_eventsub_subscription(
            twitch_api::eventsub::channel::ChannelPointsCustomRewardRemoveV1::broadcaster_user_id(
                token.user_id.clone()
            ),
            twitch_api::eventsub::Transport::websocket(&session_id),
            &token.clone()
        ).await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_points_custom_reward_redemption_add_v1 {
        // channel.bits.use
        let result = twitch.create_eventsub_subscription(
            twitch_api::eventsub::channel::ChannelPointsCustomRewardRedemptionAddV1::broadcaster_user_id(
                token.user_id.clone()
            ),
            twitch_api::eventsub::Transport::websocket(&session_id),
            &token.clone()
        ).await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_points_custom_reward_redemption_update_v1 {
        // channel.bits.use
        let result = twitch.create_eventsub_subscription(
            twitch_api::eventsub::channel::ChannelPointsCustomRewardRedemptionUpdateV1::broadcaster_user_id(
                token.user_id.clone()
            ),
            twitch_api::eventsub::Transport::websocket(&session_id),
            &token.clone()
        ).await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_poll_begin_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPollBeginV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_poll_progress_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPollProgressV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_poll_end_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPollEndV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_prediction_begin_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPredictionBeginV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_prediction_progress_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPredictionProgressV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_prediction_lock_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPredictionLockV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_prediction_end_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelPredictionEndV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_raid_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelRaidV1::to_broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));

        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelRaidV1::from_broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_shared_chat_begin_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSharedChatBeginV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_shared_chat_end_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSharedChatEndV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_shared_chat_update_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSharedChatUpdateV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_shield_mode_begin_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelShieldModeBeginV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_shield_mode_end_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelShieldModeEndV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_shoutout_create_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelShoutoutCreateV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_shoutout_receive_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelShoutoutReceiveV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_suspicious_user_message_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSuspiciousUserMessageV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_suspicious_user_update_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSuspiciousUserUpdateV1::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_goal_begin_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelGoalBeginV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_goal_progress_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelGoalProgressV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_goal_end_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelGoalEndV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_hype_train_begin_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelHypeTrainBeginV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_hype_train_progress_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelHypeTrainProgressV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_hype_train_end_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelHypeTrainEndV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_moderate_v2 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelModerateV2::new(
                    token.user_id.clone(),
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_moderator_add_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelModeratorAddV1::new(token.user_id.clone()),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
    if eventsubs.channel_moderator_remove_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelModeratorRemoveV1::new(token.user_id.clone()),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.stream_online_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::stream::StreamOnlineV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.stream_offline_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::stream::StreamOfflineV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.user_update_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::user::UserUpdateV1::new(token.user_id.clone()),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.user_whisper_message_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::user::UserWhisperMessageV1::new(token.user_id.clone()),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_subscription_end_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSubscriptionEndV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_subscription_gift_v1 {
        // channel.bits.use
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSubscriptionGiftV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };

    if eventsubs.channel_subscription_message_v1 {
        let result = twitch
            .create_eventsub_subscription(
                twitch_api::eventsub::channel::ChannelSubscriptionMessageV1::broadcaster_user_id(
                    token.user_id.clone(),
                ),
                twitch_api::eventsub::Transport::websocket(&session_id),
                &token.clone(),
            )
            .await;
        let _ = tx.send(TwitchEvent::Debug(format!("{:?}", result)));
    };
}
