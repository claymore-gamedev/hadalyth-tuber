use godot::prelude::*;

use crate::custom_config::hadalyth_twitch_eventsubs::HadalythTwitchEventSubs;
use crate::custom_config::hadalyth_twitch_scopes::HadalythTwitchScopes;
use crate::custom_events::socket_event::SocketEvent;
use crate::custom_events::twitch_event::TwitchEvent;

use crate::custom_resources::bits_custom_power_up::BitsCustomPowerUp;
use crate::custom_resources::broadcaster::Broadcaster;
use crate::custom_resources::message::Message;
use crate::custom_resources::moderator::Moderator;
use crate::custom_resources::user::User;

use crate::hadalyth_twitch_async::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HadalythTwitch {
    client_id: Option<String>,
    scopes: Option<Gd<HadalythTwitchScopes>>,
    eventsubs: Option<Gd<HadalythTwitchEventSubs>>,

    runtime: Option<tokio::runtime::Runtime>,

    session_id: Option<String>,

    tx: std::sync::mpsc::Sender<TwitchEvent>,
    rx: std::sync::mpsc::Receiver<TwitchEvent>,

    socket_tx: Option<std::sync::mpsc::Sender<SocketEvent>>,

    twitch: Option<twitch_api::helix::HelixClient<'static, reqwest::Client>>,
    token: Option<twitch_api::twitch_oauth2::UserToken>,

    base: Base<Node>,
}

#[godot_api]
impl INode for HadalythTwitch {
    fn init(base: Base<Node>) -> Self {
        let client_id = None;
        let scopes = None;
        let eventsubs = None;

        let runtime = None;

        let session_id = None;

        let (tx, rx) = std::sync::mpsc::channel::<TwitchEvent>();

        let socket_tx = None;

        let twitch: Option<twitch_api::helix::HelixClient<'static, reqwest::Client>> = None;
        let token: Option<twitch_api::twitch_oauth2::UserToken> = None;

        return Self {
            client_id,
            scopes,
            eventsubs,

            runtime,

            session_id,

            tx,
            rx,

            socket_tx,

            twitch,
            token,

            base,
        };
    }

    fn ready(&mut self) {
        // Load the client id from a text file next to the executable
        let client_id = std::fs::read_to_string("twitch_id.txt");
        let Ok(client_id) = client_id else {
            godot_error!("TwitchError: no twitch_id.txt");
            return;
        };
        let client_id = client_id.trim().to_string();
        godot_print!("TwitchInfo: loaded client_id {}", client_id);
        self.client_id = Some(client_id);

        // Try to start a tokio runtime for later async tasks
        let runtime = tokio::runtime::Runtime::new();
        let Ok(runtime) = runtime else {
            godot_error!("TwitchError: tokio runtime failed to initialize");
            return;
        };
        self.runtime = Some(runtime);
    }

    fn process(&mut self, _delta: f64) {
        // Process all events sent from other threads
        while let Ok(event) = self.rx.try_recv() {
            match event {
                TwitchEvent::Debug(message) => {
                    godot_print!("TwitchEvent::Debug: {}", message);
                }

                TwitchEvent::DeviceUserTokenRequest(verification_uri, expires_in) => {
                    godot_print!(
                        "TwitchEvent::DeviceUserTokenRequest: {}, {}",
                        verification_uri,
                        expires_in
                    );
                    godot::classes::Os::singleton().shell_open(&verification_uri);
                }

                TwitchEvent::DeviceUserTokenStatus(token) => {
                    godot_print!("TwitchEvent::DeviceUserTokenStatus");
                    self.token = token;
                    let status = self.token.is_some();
                    self.signals().device_user_token_status().emit(status);
                }

                TwitchEvent::TwitchSocketStatus(session_id) => {
                    godot_print!("TwitchEvent::TwitchSocketStatus: {:?}", session_id);
                    self.session_id = session_id;
                    let status = self.session_id.is_some();
                    self.signals().twitch_socket_status().emit(status);
                }

                TwitchEvent::TwitchEventsubReceived(event) => {
                    godot_print!("TwitchEvent::TwitchEventsubReceived");
                    self._parse_twitch_eventsub(event);
                }
            }
        }
    }
}

#[godot_api]
impl HadalythTwitch {
    // BITS TYPE CONSTS

    #[constant]
    pub const BITS_TYPE_INVALID: i64 = -1;
    #[constant]
    pub const BITS_TYPE_CHEER: i64 = 0;
    #[constant]
    pub const BITS_TYPE_POWER_UP: i64 = 1;
    #[constant]
    pub const BITS_TYPE_CUSTOM_POWER_UP: i64 = 2;

    // START OF EVENTSUB SIGNALS

    #[signal]
    pub fn recv_automod_message_hold_v2(
        broadcaster: Option<Gd<Broadcaster>>,
        held_at: String,
        message: Option<Gd<Message>>,
        message_id: String,
        reason: String,
        user: Option<Gd<User>>,
    );
    #[signal]
    pub fn recv_automod_message_update_v2(
        broadcaster: Option<Gd<Broadcaster>>,
        held_at: String,
        message: Option<Gd<Message>>,
        message_id: String,
        moderator: Option<Gd<Moderator>>,
        reason: String,
        status: String,
        user: Option<Gd<User>>,
    );
    #[signal]
    pub fn recv_automod_settings_update_v1();
    #[signal]
    pub fn recv_automod_terms_update_v1();
    #[signal]
    pub fn recv_channel_ad_break_begin_v1();
    #[signal]
    pub fn recv_channel_bits_use_v1(
        bits: i64,
        broadcaster: Option<Gd<Broadcaster>>,
        bits_custom_power_up: Option<Gd<BitsCustomPowerUp>>,
        message: Option<Gd<Message>>,
        bits_type: i64,
        user: Option<Gd<User>>,
    );
    #[signal]
    pub fn recv_channel_chat_clear_v1();
    #[signal]
    pub fn recv_channel_chat_clear_user_messages_v1();
    #[signal]
    pub fn recv_channel_chat_message_v1();
    #[signal]
    pub fn recv_channel_chat_message_delete_v1();
    #[signal]
    pub fn recv_channel_chat_notification_v1();
    #[signal]
    pub fn recv_channel_chat_user_message_hold_v1();
    #[signal]
    pub fn recv_channel_chat_user_message_update_v1();
    #[signal]
    pub fn recv_channel_chat_settings_update_v1();
    #[signal]
    pub fn recv_channel_charity_campaign_donate_v1();
    #[signal]
    pub fn recv_channel_charity_campaign_progress_v1();
    #[signal]
    pub fn recv_channel_charity_campaign_start_v1();
    #[signal]
    pub fn recv_channel_charity_campaign_stop_v1();
    #[signal]
    pub fn recv_channel_update_v2();
    #[signal]
    pub fn recv_channel_follow_v2();
    #[signal]
    pub fn recv_channel_subscribe_v1();
    #[signal]
    pub fn recv_channel_cheer_v1();
    #[signal]
    pub fn recv_channel_ban_v1();
    #[signal]
    pub fn recv_channel_unban_v1();
    #[signal]
    pub fn recv_channel_unban_request_create_v1();
    #[signal]
    pub fn recv_channel_unban_request_resolve_v1();
    #[signal]
    pub fn recv_channel_vip_add_v1();
    #[signal]
    pub fn recv_channel_vip_remove_v1();
    #[signal]
    pub fn recv_channel_warning_acknowledge_v1();
    #[signal]
    pub fn recv_channel_warning_send_v1();
    #[signal]
    pub fn recv_channel_points_automatic_reward_redemption_add_v1();
    #[signal]
    pub fn recv_channel_points_custom_reward_add_v1();
    #[signal]
    pub fn recv_channel_points_custom_reward_update_v1();
    #[signal]
    pub fn recv_channel_points_custom_reward_remove_v1();
    #[signal]
    pub fn recv_channel_points_custom_reward_redemption_add_v1();
    #[signal]
    pub fn recv_channel_points_custom_reward_redemption_update_v1();
    #[signal]
    pub fn recv_channel_poll_begin_v1();
    #[signal]
    pub fn recv_channel_poll_progress_v1();
    #[signal]
    pub fn recv_channel_poll_end_v1();
    #[signal]
    pub fn recv_channel_prediction_begin_v1();
    #[signal]
    pub fn recv_channel_prediction_progress_v1();
    #[signal]
    pub fn recv_channel_prediction_lock_v1();
    #[signal]
    pub fn recv_channel_prediction_end_v1();
    #[signal]
    pub fn recv_channel_raid_v1();
    #[signal]
    pub fn recv_channel_shared_chat_begin_v1();
    #[signal]
    pub fn recv_channel_shared_chat_end_v1();
    #[signal]
    pub fn recv_channel_shared_chat_update_v1();
    #[signal]
    pub fn recv_channel_shield_mode_begin_v1();
    #[signal]
    pub fn recv_channel_shield_mode_end_v1();
    #[signal]
    pub fn recv_channel_shoutout_create_v1();
    #[signal]
    pub fn recv_channel_shoutout_receive_v1();
    #[signal]
    pub fn recv_channel_suspicious_user_message_v1();
    #[signal]
    pub fn recv_channel_suspicious_user_update_v1();
    #[signal]
    pub fn recv_channel_goal_begin_v1();
    #[signal]
    pub fn recv_channel_goal_progress_v1();
    #[signal]
    pub fn recv_channel_goal_end_v1();
    #[signal]
    pub fn recv_channel_hype_train_begin_v1();
    #[signal]
    pub fn recv_channel_hype_train_progress_v1();
    #[signal]
    pub fn recv_channel_hype_train_end_v1();
    #[signal]
    pub fn recv_channel_moderate_v2();
    #[signal]
    pub fn recv_channel_moderator_add_v1();
    #[signal]
    pub fn recv_channel_moderator_remove_v1();
    #[signal]
    pub fn recv_stream_online_v1();
    #[signal]
    pub fn recv_stream_offline_v1();
    #[signal]
    pub fn recv_user_update_v1();
    #[signal]
    pub fn recv_user_whisper_message_v1();
    #[signal]
    pub fn recv_channel_subscription_end_v1();
    #[signal]
    pub fn recv_channel_subscription_gift_v1();
    #[signal]
    pub fn recv_channel_subscription_message_v1();

    // END OF EVENT SUB SIGNALS

    #[signal]
    fn device_user_token_status(status: bool);
    #[signal]
    fn twitch_socket_status(status: bool);
}

#[godot_api(secondary)]
impl HadalythTwitch {
    #[func]
    fn set_scopes(&mut self, scopes: Option<Gd<HadalythTwitchScopes>>) {
        self.scopes = scopes;
    }

    #[func]
    fn set_eventsubs(&mut self, eventsubs: Option<Gd<HadalythTwitchEventSubs>>) {
        self.eventsubs = eventsubs;
    }

    #[func]
    fn init_device_user_token(&mut self) {
        let Some(ref runtime) = self.runtime else {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::DeviceUserTokenStatus(None));
            godot_print!("Tokio runtime not initialized");
            return;
        };

        let twitch: twitch_api::helix::HelixClient<'static, reqwest::Client> =
            twitch_api::helix::HelixClient::default();
        self.twitch = Some(twitch);
        let Some(ref twitch) = self.twitch else {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::DeviceUserTokenStatus(None));
            godot_print!("Failed to initialize helix client");
            return;
        };

        let Some(ref client_id) = self.client_id else {
            godot_print!("Failed to initialize client id");
            return;
        };
        let client_id = twitch_api::twitch_oauth2::ClientId::new(client_id.clone());
        godot_print!("{}", client_id);

        let Some(ref mut scopes) = self.scopes else {
            godot_print!("Scopes not set");
            return;
        };
        let scopes = scopes.bind_mut().get_twitch_api_scopes();

        // let scopes = vec![
        //     twitch_api::twitch_oauth2::scopes::Scope::BitsRead,
        //     twitch_api::twitch_oauth2::scopes::Scope::ModeratorReadFollowers,
        //     twitch_api::twitch_oauth2::scopes::Scope::ChannelReadAds,
        //     twitch_api::twitch_oauth2::scopes::Scope::UserReadChat,
        //     twitch_api::twitch_oauth2::scopes::Scope::ChannelReadSubscriptions,
        //     twitch_api::twitch_oauth2::scopes::Scope::ChannelReadRedemptions,
        //     twitch_api::twitch_oauth2::scopes::Scope::ChannelManageRedemptions,
        //     twitch_api::twitch_oauth2::scopes::Scope::ChannelReadHypeTrain,
        //     twitch_api::twitch_oauth2::scopes::Scope::ModeratorReadShoutouts,
        //     twitch_api::twitch_oauth2::scopes::Scope::ModeratorManageShoutouts,
        // ];

        let tx = self.tx.clone();
        let http_client = twitch.clone_client();
        let token_builder =
            twitch_api::twitch_oauth2::DeviceUserTokenBuilder::new(client_id, scopes);

        runtime.spawn(init_device_user_token_async(tx, http_client, token_builder));
    }

    #[func]
    fn init_twitch_socket(&mut self) {
        let Some(ref runtime) = self.runtime else {
            godot_print!("Tokio runtime not initialized");
            return;
        };
        let Some(ref twitch) = self.twitch else {
            godot_print!("Helix client not initialized");
            return;
        };
        let Some(ref token) = self.token else {
            godot_print!("User token not initialized");
            return;
        };

        // If there is a current tx channel,
        // send a close signal before you open a new one
        if self.socket_tx.is_some() {
            let _ = self
                .socket_tx
                .as_ref()
                .unwrap()
                .send(SocketEvent::SocketClose);
            self.socket_tx = None;
        }

        // Open a new channel
        let (socket_tx, socket_rx) = std::sync::mpsc::channel::<SocketEvent>();
        self.socket_tx = Some(socket_tx);

        // Clone paramters
        let tx = self.tx.clone();
        let http_client = twitch.clone_client();
        let token = token.clone();

        // Spawn socket
        runtime.spawn(init_twitch_socket_async(tx, socket_rx, http_client, token));
    }

    #[func]
    fn init_subscribe_eventsubs(&mut self) {
        let Some(ref runtime) = self.runtime else {
            godot_print!("Tokio runtime not initialized");
            return;
        };
        let Some(ref twitch) = self.twitch else {
            godot_print!("Helix client not initialized");
            return;
        };
        let Some(ref token) = self.token else {
            godot_print!("User token not initialized");
            return;
        };
        let Some(ref session_id) = self.session_id else {
            godot_print!("Socket not initiliazed");
            return;
        };

        let Some(ref eventsubs) = self.eventsubs else {
            godot_error!("Eventsubs not set");
            return;
        };

        let tx = self.tx.clone();
        let twitch = twitch.clone();
        let token = token.clone();
        let session_id = session_id.clone();
        let eventsubs = eventsubs.bind().get_twitch_api_eventsubs();

        runtime.spawn(init_subscribe_eventsubs_async(
            tx, twitch, token, session_id, eventsubs,
        ));
    }

    #[func]
    fn kill_helix_client(&mut self) {
        self.twitch = None;
    }

    #[func]
    fn kill_twitch_socket(&mut self) {
        let Some(ref socket_tx) = self.socket_tx else {
            return;
        };
        let _ = socket_tx.send(SocketEvent::SocketClose);
    }
}
