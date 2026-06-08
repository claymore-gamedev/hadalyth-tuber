use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HadalythTwitchEventSubs {
    base: Base<Resource>,

    #[export_group(name = "Automod")]
    // #[export] pub automod_message_hold_v1 : bool,
    #[export]
    pub automod_message_hold_v2: bool,
    // #[export] pub automod_message_update_v1 : bool,
    #[export]
    pub automod_message_update_v2: bool,
    #[export]
    pub automod_settings_update_v1: bool,
    #[export]
    pub automod_terms_update_v1: bool,

    #[export_group(name = "Channel")]
    #[export]
    pub channel_ad_break_begin_v1: bool,
    #[export]
    pub channel_bits_use_v1: bool,
    #[export]
    pub channel_chat_clear_v1: bool,
    #[export]
    pub channel_chat_clear_user_messages_v1: bool,
    #[export]
    pub channel_chat_message_v1: bool,
    #[export]
    pub channel_chat_message_delete_v1: bool,
    #[export]
    pub channel_chat_notification_v1: bool,
    #[export]
    pub channel_chat_user_message_hold_v1: bool,
    #[export]
    pub channel_chat_user_message_update_v1: bool,
    #[export]
    pub channel_chat_settings_update_v1: bool,
    #[export]
    pub channel_charity_campaign_donate_v1: bool,
    #[export]
    pub channel_charity_campaign_progress_v1: bool,
    #[export]
    pub channel_charity_campaign_start_v1: bool,
    #[export]
    pub channel_charity_campaign_stop_v1: bool,
    #[export]
    pub channel_update_v2: bool,
    #[export]
    pub channel_follow_v2: bool,
    #[export]
    pub channel_subscribe_v1: bool,
    #[export]
    pub channel_cheer_v1: bool,
    #[export]
    pub channel_ban_v1: bool,
    #[export]
    pub channel_unban_v1: bool,
    #[export]
    pub channel_unban_request_create_v1: bool,
    #[export]
    pub channel_unban_request_resolve_v1: bool,
    #[export]
    pub channel_vip_add_v1: bool,
    #[export]
    pub channel_vip_remove_v1: bool,
    #[export]
    pub channel_warning_acknowledge_v1: bool,
    #[export]
    pub channel_warning_send_v1: bool,
    #[export]
    pub channel_points_automatic_reward_redemption_add_v1: bool,
    #[export]
    pub channel_points_custom_reward_add_v1: bool,
    #[export]
    pub channel_points_custom_reward_update_v1: bool,
    #[export]
    pub channel_points_custom_reward_remove_v1: bool,
    #[export]
    pub channel_points_custom_reward_redemption_add_v1: bool,
    #[export]
    pub channel_points_custom_reward_redemption_update_v1: bool,
    #[export]
    pub channel_poll_begin_v1: bool,
    #[export]
    pub channel_poll_progress_v1: bool,
    #[export]
    pub channel_poll_end_v1: bool,
    #[export]
    pub channel_prediction_begin_v1: bool,
    #[export]
    pub channel_prediction_progress_v1: bool,
    #[export]
    pub channel_prediction_lock_v1: bool,
    #[export]
    pub channel_prediction_end_v1: bool,
    #[export]
    pub channel_raid_v1: bool,
    #[export]
    pub channel_shared_chat_begin_v1: bool,
    #[export]
    pub channel_shared_chat_end_v1: bool,
    #[export]
    pub channel_shared_chat_update_v1: bool,
    #[export]
    pub channel_shield_mode_begin_v1: bool,
    #[export]
    pub channel_shield_mode_end_v1: bool,
    #[export]
    pub channel_shoutout_create_v1: bool,
    #[export]
    pub channel_shoutout_receive_v1: bool,
    #[export]
    pub channel_suspicious_user_message_v1: bool,
    #[export]
    pub channel_suspicious_user_update_v1: bool,
    #[export]
    pub channel_goal_begin_v1: bool,
    #[export]
    pub channel_goal_progress_v1: bool,
    #[export]
    pub channel_goal_end_v1: bool,
    #[export]
    pub channel_hype_train_begin_v1: bool,
    #[export]
    pub channel_hype_train_progress_v1: bool,
    #[export]
    pub channel_hype_train_end_v1: bool,
    // #[export] pub channel_moderate_v1 : bool,
    #[export]
    pub channel_moderate_v2: bool,
    #[export]
    pub channel_moderator_add_v1: bool,
    #[export]
    pub channel_moderator_remove_v1: bool,
    #[export]
    pub channel_subscription_end_v1: bool,
    #[export]
    pub channel_subscription_gift_v1: bool,
    #[export]
    pub channel_subscription_message_v1: bool,

    // #[export] pub conduit_shard_disabled_v1 : bool,
    // #[export] pub extension_bits_transaction_create_v1 : bool,
    #[export_group(name = "Stream")]
    #[export]
    pub stream_online_v1: bool,
    #[export]
    pub stream_offline_v1: bool,

    #[export_group(name = "User")]
    #[export]
    pub user_update_v1: bool,
    // #[export] pub user_authorization_grant_v1 : bool,
    // #[export] pub user_authorization_revoke_v1 : bool,
    #[export]
    pub user_whisper_message_v1: bool,
}
pub struct TwitchApiEventSubs {
    pub automod_message_hold_v2: bool,
    pub automod_message_update_v2: bool,
    pub automod_settings_update_v1: bool,
    pub automod_terms_update_v1: bool,
    pub channel_ad_break_begin_v1: bool,
    pub channel_bits_use_v1: bool,
    pub channel_chat_clear_v1: bool,
    pub channel_chat_clear_user_messages_v1: bool,
    pub channel_chat_message_v1: bool,
    pub channel_chat_message_delete_v1: bool,
    pub channel_chat_notification_v1: bool,
    pub channel_chat_user_message_hold_v1: bool,
    pub channel_chat_user_message_update_v1: bool,
    pub channel_chat_settings_update_v1: bool,
    pub channel_charity_campaign_donate_v1: bool,
    pub channel_charity_campaign_progress_v1: bool,
    pub channel_charity_campaign_start_v1: bool,
    pub channel_charity_campaign_stop_v1: bool,
    pub channel_update_v2: bool,
    pub channel_follow_v2: bool,
    pub channel_subscribe_v1: bool,
    pub channel_cheer_v1: bool,
    pub channel_ban_v1: bool,
    pub channel_unban_v1: bool,
    pub channel_unban_request_create_v1: bool,
    pub channel_unban_request_resolve_v1: bool,
    pub channel_vip_add_v1: bool,
    pub channel_vip_remove_v1: bool,
    pub channel_warning_acknowledge_v1: bool,
    pub channel_warning_send_v1: bool,
    pub channel_points_automatic_reward_redemption_add_v1: bool,
    pub channel_points_custom_reward_add_v1: bool,
    pub channel_points_custom_reward_update_v1: bool,
    pub channel_points_custom_reward_remove_v1: bool,
    pub channel_points_custom_reward_redemption_add_v1: bool,
    pub channel_points_custom_reward_redemption_update_v1: bool,
    pub channel_poll_begin_v1: bool,
    pub channel_poll_progress_v1: bool,
    pub channel_poll_end_v1: bool,
    pub channel_prediction_begin_v1: bool,
    pub channel_prediction_progress_v1: bool,
    pub channel_prediction_lock_v1: bool,
    pub channel_prediction_end_v1: bool,
    pub channel_raid_v1: bool,
    pub channel_shared_chat_begin_v1: bool,
    pub channel_shared_chat_end_v1: bool,
    pub channel_shared_chat_update_v1: bool,
    pub channel_shield_mode_begin_v1: bool,
    pub channel_shield_mode_end_v1: bool,
    pub channel_shoutout_create_v1: bool,
    pub channel_shoutout_receive_v1: bool,
    pub channel_suspicious_user_message_v1: bool,
    pub channel_suspicious_user_update_v1: bool,
    pub channel_goal_begin_v1: bool,
    pub channel_goal_progress_v1: bool,
    pub channel_goal_end_v1: bool,
    pub channel_hype_train_begin_v1: bool,
    pub channel_hype_train_progress_v1: bool,
    pub channel_hype_train_end_v1: bool,
    pub channel_moderate_v2: bool,
    pub channel_moderator_add_v1: bool,
    pub channel_moderator_remove_v1: bool,
    pub stream_online_v1: bool,
    pub stream_offline_v1: bool,
    pub user_update_v1: bool,
    pub user_whisper_message_v1: bool,
    pub channel_subscription_end_v1: bool,
    pub channel_subscription_gift_v1: bool,
    pub channel_subscription_message_v1: bool,
}
impl HadalythTwitchEventSubs {
    pub fn get_twitch_api_eventsubs(&self) -> TwitchApiEventSubs {
        TwitchApiEventSubs {
            automod_message_hold_v2: self.automod_message_hold_v2,
            automod_message_update_v2: self.automod_message_update_v2,
            automod_settings_update_v1: self.automod_settings_update_v1,
            automod_terms_update_v1: self.automod_terms_update_v1,
            channel_ad_break_begin_v1: self.channel_ad_break_begin_v1,
            channel_bits_use_v1: self.channel_bits_use_v1,
            channel_chat_clear_v1: self.channel_chat_clear_v1,
            channel_chat_clear_user_messages_v1: self.channel_chat_clear_user_messages_v1,
            channel_chat_message_v1: self.channel_chat_message_v1,
            channel_chat_message_delete_v1: self.channel_chat_message_delete_v1,
            channel_chat_notification_v1: self.channel_chat_notification_v1,
            channel_chat_user_message_hold_v1: self.channel_chat_user_message_hold_v1,
            channel_chat_user_message_update_v1: self.channel_chat_user_message_update_v1,
            channel_chat_settings_update_v1: self.channel_chat_settings_update_v1,
            channel_charity_campaign_donate_v1: self.channel_charity_campaign_donate_v1,
            channel_charity_campaign_progress_v1: self.channel_charity_campaign_progress_v1,
            channel_charity_campaign_start_v1: self.channel_charity_campaign_start_v1,
            channel_charity_campaign_stop_v1: self.channel_charity_campaign_stop_v1,
            channel_update_v2: self.channel_update_v2,
            channel_follow_v2: self.channel_follow_v2,
            channel_subscribe_v1: self.channel_subscribe_v1,
            channel_cheer_v1: self.channel_cheer_v1,
            channel_ban_v1: self.channel_ban_v1,
            channel_unban_v1: self.channel_unban_v1,
            channel_unban_request_create_v1: self.channel_unban_request_create_v1,
            channel_unban_request_resolve_v1: self.channel_unban_request_resolve_v1,
            channel_vip_add_v1: self.channel_vip_add_v1,
            channel_vip_remove_v1: self.channel_vip_remove_v1,
            channel_warning_acknowledge_v1: self.channel_warning_acknowledge_v1,
            channel_warning_send_v1: self.channel_warning_send_v1,
            channel_points_automatic_reward_redemption_add_v1: self
                .channel_points_automatic_reward_redemption_add_v1,
            channel_points_custom_reward_add_v1: self.channel_points_custom_reward_add_v1,
            channel_points_custom_reward_update_v1: self.channel_points_custom_reward_update_v1,
            channel_points_custom_reward_remove_v1: self.channel_points_custom_reward_remove_v1,
            channel_points_custom_reward_redemption_add_v1: self
                .channel_points_custom_reward_redemption_add_v1,
            channel_points_custom_reward_redemption_update_v1: self
                .channel_points_custom_reward_redemption_update_v1,
            channel_poll_begin_v1: self.channel_poll_begin_v1,
            channel_poll_progress_v1: self.channel_poll_progress_v1,
            channel_poll_end_v1: self.channel_poll_end_v1,
            channel_prediction_begin_v1: self.channel_prediction_begin_v1,
            channel_prediction_progress_v1: self.channel_prediction_progress_v1,
            channel_prediction_lock_v1: self.channel_prediction_lock_v1,
            channel_prediction_end_v1: self.channel_prediction_end_v1,
            channel_raid_v1: self.channel_raid_v1,
            channel_shared_chat_begin_v1: self.channel_shared_chat_begin_v1,
            channel_shared_chat_end_v1: self.channel_shared_chat_end_v1,
            channel_shared_chat_update_v1: self.channel_shared_chat_update_v1,
            channel_shield_mode_begin_v1: self.channel_shield_mode_begin_v1,
            channel_shield_mode_end_v1: self.channel_shield_mode_end_v1,
            channel_shoutout_create_v1: self.channel_shoutout_create_v1,
            channel_shoutout_receive_v1: self.channel_shoutout_receive_v1,
            channel_suspicious_user_message_v1: self.channel_suspicious_user_message_v1,
            channel_suspicious_user_update_v1: self.channel_suspicious_user_update_v1,
            channel_goal_begin_v1: self.channel_goal_begin_v1,
            channel_goal_progress_v1: self.channel_goal_progress_v1,
            channel_goal_end_v1: self.channel_goal_end_v1,
            channel_hype_train_begin_v1: self.channel_hype_train_begin_v1,
            channel_hype_train_progress_v1: self.channel_hype_train_progress_v1,
            channel_hype_train_end_v1: self.channel_hype_train_end_v1,
            channel_moderate_v2: self.channel_moderate_v2,
            channel_moderator_add_v1: self.channel_moderator_add_v1,
            channel_moderator_remove_v1: self.channel_moderator_remove_v1,
            stream_online_v1: self.stream_online_v1,
            stream_offline_v1: self.stream_offline_v1,
            user_update_v1: self.user_update_v1,
            user_whisper_message_v1: self.user_whisper_message_v1,
            channel_subscription_end_v1: self.channel_subscription_end_v1,
            channel_subscription_gift_v1: self.channel_subscription_gift_v1,
            channel_subscription_message_v1: self.channel_subscription_message_v1,
        }
    }
}
