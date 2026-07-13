use godot::prelude::*;

use crate::custom_resources::badge::Badge;
use crate::custom_resources::bits_custom_power_up::BitsCustomPowerUp;
use crate::custom_resources::charity::Charity;
use crate::custom_resources::currency::Currency;
use crate::custom_resources::message::Message;
use crate::custom_resources::reply::Reply;
use crate::custom_resources::source::Source;
use crate::custom_resources::timestamp::Timestamp;
use crate::custom_resources::user::User;

use crate::custom_traits::to_godot_message::ToGodotMessage;

use super::hadalyth_twitch::HadalythTwitch;

#[godot_api(secondary)]
impl HadalythTwitch {
    pub fn _parse_twitch_eventsub(&mut self, event: twitch_api::eventsub::Event) {
        match event {
            twitch_api::eventsub::Event::AutomodMessageHoldV2(payload) => {
                godot_print!("EventSub AutomodMessageHoldV2");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let held_at = payload.held_at.to_string();

                        let message = payload.message.to_godot_message();

                        let message_id = payload.message_id.to_string();

                        let reason = format!("{:?}", payload.reason);

                        let user = User::create(
                            payload.user_id.to_string().to_godot(),
                            payload.user_login.to_string().to_godot(),
                            payload.user_name.to_string().to_godot(),
                        );

                        self.signals().recv_automod_message_hold_v2().emit(
                            &broadcaster,
                            &user,
                            &message,
                            message_id,
                            held_at,
                            reason,
                        )
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::AutomodMessageUpdateV2(payload) => {
                godot_print!("EventSub AutomodMessageUpdateV2");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster: Gd<User> = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let held_at = payload.held_at.to_string();

                        let message = payload.message.to_godot_message();

                        let message_id = payload.message_id.to_string();

                        let moderator = User::create(
                            payload.moderator_user_id.to_string().to_godot(),
                            payload.moderator_user_login.to_string().to_godot(),
                            payload.moderator_user_name.to_string().to_godot(),
                        );

                        let reason = format!("{:?}", payload.reason);

                        let status = format!("{:?}", payload.status);

                        let user = User::create(
                            payload.user_id.to_string().to_godot(),
                            payload.user_login.to_string().to_godot(),
                            payload.user_name.to_string().to_godot(),
                        );

                        self.signals().recv_automod_message_update_v2().emit(
                            &broadcaster,
                            &moderator,
                            &user,
                            &message,
                            message_id,
                            held_at,
                            reason,
                            status,
                        );
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::AutomodSettingsUpdateV1(payload) => {
                godot_print!("EventSub AutomodSettingsUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let moderator = User::create(
                            payload.moderator_user_id.to_string().to_godot(),
                            payload.moderator_user_login.to_string().to_godot(),
                            payload.moderator_user_name.to_string().to_godot(),
                        );

                        let aggression = payload.aggression;
                        let bullying = payload.bullying;
                        let disability = payload.disability;
                        let misogyny = payload.misogyny;
                        let race_ethnicity_or_religion = payload.race_ethnicity_or_religion;
                        let sex_based_terms = payload.sex_based_terms;
                        let sexuality_sex_or_gender = payload.sexuality_sex_or_gender;
                        let swearing = payload.swearing;

                        let overall_level = match payload.overall_level {
                            Some(overall_level) => overall_level,
                            None => 0,
                        };

                        self.signals().recv_automod_settings_update_v1().emit(
                            &broadcaster,
                            &moderator,
                            aggression as i64,
                            bullying as i64,
                            disability as i64,
                            misogyny as i64,
                            race_ethnicity_or_religion as i64,
                            sex_based_terms as i64,
                            sexuality_sex_or_gender as i64,
                            swearing as i64,
                            overall_level as i64,
                        )
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::AutomodTermsUpdateV1(payload) => {
                godot_print!("EventSub AutomodTermsUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let action = payload.action;
                        let action = format!("{:?}", action);

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let moderator = User::create(
                            payload.moderator_user_id.to_string().to_godot(),
                            payload.moderator_user_login.to_string().to_godot(),
                            payload.moderator_user_name.to_string().to_godot(),
                        );

                        let from_automod = payload.from_automod;

                        let terms: Vec<GString> =
                            payload.terms.iter().map(|x| x.to_godot()).collect();
                        let terms = PackedStringArray::from(terms);

                        self.signals().recv_automod_terms_update_v1().emit(
                            &broadcaster,
                            &moderator,
                            from_automod,
                            action,
                            &terms,
                        )
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelAdBreakBeginV1(payload) => {
                godot_print!("EventSub ChannelAdBreakBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let requester = User::create(
                            payload.requester_user_id.to_string().to_godot(),
                            payload.requester_user_login.to_string().to_godot(),
                            payload.requester_user_name.to_string().to_godot(),
                        );

                        let started_at = payload.started_at;
                        let started_at = Timestamp::create(
                            started_at.year().into(),
                            started_at.month().into(),
                            started_at.day().into(),
                            started_at.hour().into(),
                            started_at.minute().into(),
                            started_at.second().into(),
                        );

                        let duration_seconds = payload.duration_seconds;

                        let is_automatic = payload.is_automatic;

                        self.signals().recv_channel_ad_break_begin_v1().emit(
                            &broadcaster,
                            &requester,
                            &started_at,
                            duration_seconds as i64,
                            is_automatic,
                        )
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelBitsUseV1(payload) => {
                godot_print!("EventSub ChannelBitsUseV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let bits = payload.bits as i64;

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let bits_custom_power_up = match payload.custom_power_up {
                            Some(bits_custom_power_up) => Some(BitsCustomPowerUp::create(
                                bits_custom_power_up.title.to_godot(),
                                bits_custom_power_up.reward_id.to_string().to_godot(),
                            )),
                            None => None,
                        };

                        let message: Option<Gd<Message>> = match payload.message {
                            Some(message) => Some(message.to_godot_message()),
                            None => None,
                        };

                        let bits_type = match payload.type_ {
                            twitch_api::eventsub::channel::bits::BitsType::Cheer => {
                                HadalythTwitch::BITS_TYPE_CHEER
                            }
                            twitch_api::eventsub::channel::bits::BitsType::PowerUp => {
                                HadalythTwitch::BITS_TYPE_POWER_UP
                            }
                            twitch_api::eventsub::channel::bits::BitsType::CustomPowerUp => {
                                HadalythTwitch::BITS_TYPE_CUSTOM_POWER_UP
                            }
                            _ => HadalythTwitch::BITS_TYPE_INVALID,
                        };

                        let user = User::create(
                            payload.user_id.to_string().to_godot(),
                            payload.user_login.to_string().to_godot(),
                            payload.user_name.to_string().to_godot(),
                        );

                        self.signals().recv_channel_bits_use_v1().emit(
                            &broadcaster,
                            &user,
                            message.as_ref(),
                            bits,
                            bits_type,
                            bits_custom_power_up.as_ref(),
                        )
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatClearV1(payload) => {
                godot_print!("EventSub ChannelChatClearV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        self.signals()
                            .recv_channel_chat_clear_v1()
                            .emit(&broadcaster);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatClearUserMessagesV1(payload) => {
                godot_print!("EventSub ChannelChatClearUserMessagesV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let user = User::create(
                            payload.target_user_id.to_string().to_godot(),
                            payload.target_user_login.to_string().to_godot(),
                            payload.target_user_name.to_string().to_godot(),
                        );

                        self.signals()
                            .recv_channel_chat_clear_user_messages_v1()
                            .emit(&broadcaster, &user);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatMessageV1(payload) => {
                godot_print!("EventSub ChannelChatMessageV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let twitch_api_badges = payload.badges;
                        let mut badges: Array<Gd<Badge>> = array![];
                        for twitch_api_badge in twitch_api_badges.iter() {
                            let badge = Badge::create(
                                twitch_api_badge.set_id.to_string().to_godot(),
                                twitch_api_badge.id.to_string().to_godot(),
                                twitch_api_badge.info.to_string().to_godot(),
                            );
                            badges.push(&badge);
                        }

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let channel_points_animation_id = payload.channel_points_animation_id;
                        let channel_points_animation_id = match channel_points_animation_id {
                            Some(channel_points_animation_id) => channel_points_animation_id,
                            None => "".to_string(),
                        };

                        let channel_points_custom_reward_id =
                            payload.channel_points_custom_reward_id;
                        let channel_points_custom_reward_id = match channel_points_custom_reward_id
                        {
                            Some(channel_points_custom_reward_id) => {
                                channel_points_custom_reward_id.to_string()
                            }
                            None => "".to_string(),
                        };

                        let chatter = User::create(
                            payload.chatter_user_id.to_string().to_godot(),
                            payload.chatter_user_login.to_string().to_godot(),
                            payload.chatter_user_name.to_string().to_godot(),
                        );

                        let cheer = payload.cheer;
                        let cheer = match cheer {
                            Some(cheer) => cheer.bits,
                            None => 0,
                        } as i64;

                        let color = payload.color.to_string();

                        let is_source_only = payload.is_source_only;
                        let is_source_only = match is_source_only {
                            Some(is_source_only) => is_source_only,
                            None => false,
                        };

                        let message = payload.message.to_godot_message();

                        let message_id = payload.message_id.to_string();

                        let message_type = payload.message_type as i64;

                        let reply = payload.reply;
                        let reply = match reply {
                            Some(reply) => Some(Reply::create(
                                reply.parent_message_id.to_string().to_godot(),
                                reply.parent_message_body.to_string().to_godot(),
                                reply.parent_user_id.to_string().to_godot(),
                                reply.parent_user_name.to_string().to_godot(),
                                reply.parent_user_login.to_string().to_godot(),
                                reply.thread_message_id.to_string().to_godot(),
                                reply.thread_user_id.to_string().to_godot(),
                                reply.thread_user_name.to_string().to_godot(),
                                reply.thread_user_login.to_string().to_godot(),
                            )),
                            None => None,
                        };

                        let twitch_api_badges = payload.source_badges;
                        let mut source_badges: Array<Gd<Badge>> = array![];
                        for twitch_api_badge in twitch_api_badges.iter() {
                            let badge = Badge::create(
                                twitch_api_badge.set_id.to_string().to_godot(),
                                twitch_api_badge.id.to_string().to_godot(),
                                twitch_api_badge.info.to_string().to_godot(),
                            );
                            source_badges.push(&badge);
                        }

                        let source_broadcaster = match (
                            payload.source_broadcaster_user_id,
                            payload.source_broadcaster_user_login,
                            payload.source_broadcaster_user_name,
                        ) {
                            (
                                Some(source_broadcaster_user_id),
                                Some(source_broadcaster_user_login),
                                Some(source_broadcaster_user_name),
                            ) => Some(User::create(
                                source_broadcaster_user_id.to_string().to_godot(),
                                source_broadcaster_user_login.to_string().to_godot(),
                                source_broadcaster_user_name.to_string().to_godot(),
                            )),
                            _ => None,
                        };

                        let source_message_id = payload.source_message_id;
                        let source_message_id = match source_message_id {
                            Some(source_message_id) => source_message_id.to_string(),
                            None => "".to_string(),
                        };

                        let source = match source_broadcaster {
                            Some(source_broadcaster) => Some(Source::create(
                                source_badges,
                                Some(source_broadcaster),
                                source_message_id.to_godot(),
                            )),
                            None => None,
                        };

                        self.signals().recv_channel_chat_message_v1().emit(
                            &broadcaster,
                            &chatter,
                            &message,
                            message_id,
                            message_type,
                            &badges,
                            channel_points_animation_id,
                            channel_points_custom_reward_id,
                            cheer,
                            color,
                            is_source_only,
                            reply.as_ref(),
                            source.as_ref(),
                        )
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatMessageDeleteV1(payload) => {
                godot_print!("EventSub ChannelChatMessageDeleteV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_user_id.to_string().to_godot(),
                            payload.broadcaster_user_login.to_string().to_godot(),
                            payload.broadcaster_user_name.to_string().to_godot(),
                        );

                        let user = User::create(
                            payload.target_user_id.to_string().to_godot(),
                            payload.target_user_login.to_string().to_godot(),
                            payload.target_user_name.to_string().to_godot(),
                        );

                        let message_id = payload.message_id.to_string();

                        self.signals().recv_channel_chat_message_delete_v1().emit(
                            &broadcaster,
                            &user,
                            message_id,
                        )
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatNotificationV1(payload) => {
                godot_print!("EventSub ChannelChatNotificationV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                        
                        godot_error!("Event not implemented yet");

                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatUserMessageHoldV1(payload) => {
                godot_print!("EventSub ChannelChatUserMessageHoldV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        godot_error!("Event not implemented yet");

                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatUserMessageUpdateV1(payload) => {
                godot_print!("EventSub ChannelChatUserMessageUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        godot_error!("Event not implemented yet");

                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelChatSettingsUpdateV1(payload) => {
                godot_print!("EventSub ChannelChatSettingsUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        godot_error!("Event not implemented yet");

                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelCharityCampaignDonateV1(payload) => {
                godot_print!("EventSub ChannelCharityCampaignDonateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_id.to_string().to_godot(),
                            payload.broadcaster_login.to_string().to_godot(),
                            payload.broadcaster_name.to_string().to_godot(),
                        );
                        
                        let user = User::create(
                            payload.user_id.to_string().to_godot(),
                            payload.user_login.to_string().to_godot(),
                            payload.user_name.to_string().to_godot(),
                        );
                        
                        let charity = Charity::create(
                            payload.charity_description.to_godot(),
                            payload.charity_logo.to_godot(),
                            payload.charity_name.to_godot(),
                            payload.charity_website.to_godot()
                        );

                        let amount = payload.amount;
                        let amount = Currency::create(
                            amount.value, 
                            amount.decimal_places, 
                            amount.currency.to_godot()
                        );

                        let campaign_id = payload.campaign_id.as_str().to_string();
                        let donation_id = payload.id.as_str().to_string();                      
                        
                        self.signals().recv_channel_charity_campaign_donate_v1().emit(
                            &broadcaster,
                            &user,
                            &charity,
                            &amount,
                            campaign_id,
                            donation_id
                        );
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelCharityCampaignProgressV1(payload) => {
                godot_print!("EventSub ChannelCharityCampaignProgressV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);

                        let broadcaster = User::create(
                            payload.broadcaster_id.to_string().to_godot(),
                            payload.broadcaster_login.to_string().to_godot(),
                            payload.broadcaster_name.to_string().to_godot(),
                        );
                        
                        let charity = Charity::create(
                            payload.charity_description.to_godot(),
                            payload.charity_logo.to_godot(),
                            payload.charity_name.to_godot(),
                            payload.charity_website.to_godot()
                        );

                        let current_amount = payload.current_amount;
                        let current_amount = Currency::create(
                            current_amount.value, 
                            current_amount.decimal_places, 
                            current_amount.currency.to_godot()
                        );

                        let target_amount = payload.target_amount;
                        let target_amount = Currency::create(
                            target_amount.value, 
                            target_amount.decimal_places, 
                            target_amount.currency.to_godot()
                        );

                        let campaign_id = payload.id.as_str().to_string();
                        
                        self.signals().recv_channel_charity_campaign_progress_v1().emit(
                            &broadcaster,
                            &charity,
                            &current_amount,
                            &target_amount,
                            campaign_id
                        );
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelCharityCampaignStartV1(payload) => {
                godot_print!("EventSub ChannelCharityCampaignStartV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelCharityCampaignStopV1(payload) => {
                godot_print!("EventSub ChannelCharityCampaignStopV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelUpdateV2(payload) => {
                godot_print!("EventSub ChannelUpdateV2");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelFollowV2(payload) => {
                godot_print!("EventSub ChannelFollowV2");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSubscribeV1(payload) => {
                godot_print!("EventSub ChannelSubscribeV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelCheerV1(payload) => {
                godot_print!("EventSub ChannelCheerV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelBanV1(payload) => {
                godot_print!("EventSub ChannelBanV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelUnbanV1(payload) => {
                godot_print!("EventSub ChannelUnbanV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelUnbanRequestCreateV1(payload) => {
                godot_print!("EventSub ChannelUnbanRequestCreateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelUnbanRequestResolveV1(payload) => {
                godot_print!("EventSub ChannelUnbanRequestResolveV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelVipAddV1(payload) => {
                godot_print!("EventSub ChannelVipAddV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelVipRemoveV1(payload) => {
                godot_print!("EventSub ChannelVipRemoveV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelWarningAcknowledgeV1(payload) => {
                godot_print!("EventSub ChannelWarningAcknowledgeV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelWarningSendV1(payload) => {
                godot_print!("EventSub ChannelWarningSendV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPointsAutomaticRewardRedemptionAddV1(payload) => {
                godot_print!("EventSub ChannelPointsAutomaticRewardRedemptionAddV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPointsCustomRewardAddV1(payload) => {
                godot_print!("EventSub ChannelPointsCustomRewardAddV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPointsCustomRewardUpdateV1(payload) => {
                godot_print!("EventSub ChannelPointsCustomRewardUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPointsCustomRewardRemoveV1(payload) => {
                godot_print!("EventSub ChannelPointsCustomRewardRemoveV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPointsCustomRewardRedemptionAddV1(payload) => {
                godot_print!("EventSub ChannelPointsCustomRewardRedemptionAddV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPointsCustomRewardRedemptionUpdateV1(payload) => {
                godot_print!("EventSub ChannelPointsCustomRewardRedemptionUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPollBeginV1(payload) => {
                godot_print!("EventSub ChannelPollBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPollProgressV1(payload) => {
                godot_print!("EventSub ChannelPollProgressV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPollEndV1(payload) => {
                godot_print!("EventSub ChannelPollEndV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPredictionBeginV1(payload) => {
                godot_print!("EventSub ChannelPredictionBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPredictionProgressV1(payload) => {
                godot_print!("EventSub ChannelPredictionProgressV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPredictionLockV1(payload) => {
                godot_print!("EventSub ChannelPredictionLockV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelPredictionEndV1(payload) => {
                godot_print!("EventSub ChannelPredictionEndV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelRaidV1(payload) => {
                godot_print!("EventSub ChannelRaidV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSharedChatBeginV1(payload) => {
                godot_print!("EventSub ChannelSharedChatBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSharedChatEndV1(payload) => {
                godot_print!("EventSub ChannelSharedChatEndV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSharedChatUpdateV1(payload) => {
                godot_print!("EventSub ChannelSharedChatUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelShieldModeBeginV1(payload) => {
                godot_print!("EventSub ChannelShieldModeBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelShieldModeEndV1(payload) => {
                godot_print!("EventSub ChannelShieldModeEndV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelShoutoutCreateV1(payload) => {
                godot_print!("EventSub ChannelShoutoutCreateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelShoutoutReceiveV1(payload) => {
                godot_print!("EventSub ChannelShoutoutReceiveV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSuspiciousUserMessageV1(payload) => {
                godot_print!("EventSub ChannelSuspiciousUserMessageV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSuspiciousUserUpdateV1(payload) => {
                godot_print!("EventSub ChannelSuspiciousUserUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelGoalBeginV1(payload) => {
                godot_print!("EventSub ChannelGoalBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelGoalProgressV1(payload) => {
                godot_print!("EventSub ChannelGoalProgressV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelGoalEndV1(payload) => {
                godot_print!("EventSub ChannelGoalEndV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelHypeTrainBeginV1(payload) => {
                godot_print!("EventSub ChannelHypeTrainBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelHypeTrainProgressV1(payload) => {
                godot_print!("EventSub ChannelHypeTrainProgressV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelHypeTrainEndV1(payload) => {
                godot_print!("EventSub ChannelHypeTrainEndV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelModerateV2(payload) => {
                godot_print!("EventSub ChannelModerateV2");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelModeratorAddV1(payload) => {
                godot_print!("EventSub ChannelModeratorAddV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelModeratorRemoveV1(payload) => {
                godot_print!("EventSub ChannelModeratorRemoveV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::StreamOnlineV1(payload) => {
                godot_print!("EventSub StreamOnlineV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::StreamOfflineV1(payload) => {
                godot_print!("EventSub StreamOfflineV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::UserUpdateV1(payload) => {
                godot_print!("EventSub UserUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::UserWhisperMessageV1(payload) => {
                godot_print!("EventSub UserWhisperMessageV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSubscriptionEndV1(payload) => {
                godot_print!("EventSub ChannelSubscriptionEndV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSubscriptionGiftV1(payload) => {
                godot_print!("EventSub ChannelSubscriptionGiftV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::ChannelSubscriptionMessageV1(payload) => {
                godot_print!("EventSub ChannelSubscriptionMessageV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {
                        unreachable!()
                    }
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            _ => {
                godot_print!("EventSub not implemented");
            }
        }
    }
}
