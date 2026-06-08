use godot::prelude::*;

use super::hadalyth_twitch::HadalythTwitch;

#[godot_api(secondary)]
impl HadalythTwitch {

    pub fn _parse_twitch_eventsub(&mut self, event : twitch_api::eventsub::Event) {
        match event {
            twitch_api::eventsub::Event::AutomodMessageHoldV2(payload) => {
                godot_print!("EventSub AutomodMessageHoldV2");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }

            twitch_api::eventsub::Event::AutomodMessageUpdateV2(payload) => {
                godot_print!("EventSub AutomodMessageUpdateV2");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::AutomodSettingsUpdateV1(payload) => {
                godot_print!("EventSub AutomodSettingsUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::AutomodTermsUpdateV1(payload) => {
                godot_print!("EventSub AutomodTermsUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelAdBreakBeginV1(payload) => {
                godot_print!("EventSub ChannelAdBreakBeginV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelBitsUseV1(payload) => {
                godot_print!("EventSub ChannelBitsUseV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatClearV1(payload) => {
                godot_print!("EventSub ChannelChatClearV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatClearUserMessagesV1(payload) => {
                godot_print!("EventSub ChannelChatClearUserMessagesV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatMessageV1(payload) => {
                godot_print!("EventSub ChannelChatMessageV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatMessageDeleteV1(payload) => {
                godot_print!("EventSub ChannelChatMessageDeleteV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatNotificationV1(payload) => {
                godot_print!("EventSub ChannelChatNotificationV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatUserMessageHoldV1(payload) => {
                godot_print!("EventSub ChannelChatUserMessageHoldV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatUserMessageUpdateV1(payload) => {
                godot_print!("EventSub ChannelChatUserMessageUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelChatSettingsUpdateV1(payload) => {
                godot_print!("EventSub ChannelChatSettingsUpdateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelCharityCampaignDonateV1(payload) => {
                godot_print!("EventSub ChannelCharityCampaignDonateV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelCharityCampaignProgressV1(payload) => {
                godot_print!("EventSub ChannelCharityCampaignProgressV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
                    twitch_api::eventsub::Message::Revocation() => {}
                    twitch_api::eventsub::Message::Notification(payload) => {
                        godot_print!("\t{:?}", payload);
                    }
                    _ => {}
                }
            }
            
            twitch_api::eventsub::Event::ChannelCharityCampaignStartV1(payload) => {
                godot_print!("EventSub ChannelCharityCampaignStartV1");
                match payload.message {
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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
                    twitch_api::eventsub::Message::VerificationRequest(_) => {unreachable!()}
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