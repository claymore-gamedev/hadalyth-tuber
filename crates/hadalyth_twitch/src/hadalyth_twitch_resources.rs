use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HadalythTwitchScopes {
    base : Base<Resource>,

    #[export_group(name="Analytics")]
    #[export] analytics_read_extensions : bool,         
    #[export] analytics_read_games : bool,
    
    #[export_group(name="Bits")]              
    #[export] bits_read : bool, 

    #[export_group(name="Channel")]                        
    #[export] channel_bot : bool,                       
    #[export] channel_edit_commercial : bool,           
    #[export] channel_manage_ads : bool,                
    #[export] channel_manage_broadcast : bool,          
    #[export] channel_manage_clips : bool,              
    #[export] channel_manage_extensions : bool,         
    #[export] channel_manage_guest_star : bool,         
    #[export] channel_manage_moderators : bool,         
    #[export] channel_manage_polls : bool,              
    #[export] channel_manage_predictions : bool,        
    #[export] channel_manage_raids : bool,              
    #[export] channel_manage_redemptions : bool,        
    #[export] channel_manage_schedule : bool,           
    #[export] channel_manage_videos : bool,             
    #[export] channel_manage_vips : bool,               
    #[export] channel_moderate : bool,                  
    #[export] channel_read_ads : bool,                  
    #[export] channel_read_charity : bool,              
    #[export] channel_read_editors : bool,              
    #[export] channel_read_goals : bool,                
    #[export] channel_read_guest_star : bool,           
    #[export] channel_read_hype_train : bool,           
    #[export] channel_read_polls : bool,                
    #[export] channel_read_predictions : bool,          
    #[export] channel_read_redemptions : bool,          
    #[export] channel_read_stream_key : bool,           
    #[export] channel_read_subscriptions : bool,        
    #[export] channel_read_vips : bool,                 
    
    #[export_group(name="Chat")]
    #[export] chat_edit : bool,                         
    #[export] chat_read : bool,                         

    #[export_group(name="Clips")]
    #[export] clips_edit : bool,                        
    #[export] editor_manage_clips : bool,               

    #[export_group(name="Moderation")]
    #[export] moderation_read : bool,                   

    #[export_group(name="Moderator")]
    #[export] moderator_manage_announcements : bool,    
    #[export] moderator_manage_automod : bool,          
    #[export] moderator_manage_automod_settings : bool, 
    #[export] moderator_manage_banned_users : bool,     
    #[export] moderator_manage_blocked_terms : bool,    
    #[export] moderator_manage_chat_messages : bool,    
    #[export] moderator_manage_chat_settings : bool,    
    #[export] moderator_manage_guest_star : bool,       
    #[export] moderator_manage_shield_mode : bool,      
    #[export] moderator_manage_shoutouts : bool,        
    #[export] moderator_manage_suspicious_users : bool, 
    #[export] moderator_manage_unban_requests : bool,   
    #[export] moderator_manage_warnings : bool,         
    #[export] moderator_read_automod_settings : bool,   
    #[export] moderator_read_banned_users : bool,       
    #[export] moderator_read_blocked_terms : bool,      
    #[export] moderator_read_chat_messages : bool,      
    #[export] moderator_read_chat_settings : bool,      
    #[export] moderator_read_chatters : bool,           
    #[export] moderator_read_followers : bool,          
    #[export] moderator_read_guest_star : bool,         
    #[export] moderator_read_moderators : bool,         
    #[export] moderator_read_shield_mode : bool,        
    #[export] moderator_read_shoutouts : bool,          
    #[export] moderator_read_suspicious_users : bool,   
    #[export] moderator_read_unban_requests : bool,     
    #[export] moderator_read_vips : bool,               
    #[export] moderator_read_warnings : bool,       

    #[export_group(name="User")]    
    #[export] user_bot : bool,                          
    #[export] user_edit : bool,                         
    #[export] user_edit_broadcast : bool,               
    #[export] user_manage_blocked_users : bool,         
    #[export] user_manage_chat_color : bool,            
    #[export] user_manage_whispers : bool,              
    #[export] user_read_blocked_users : bool,           
    #[export] user_read_broadcast : bool,               
    #[export] user_read_chat : bool,                    
    #[export] user_read_email : bool,                   
    #[export] user_read_emotes : bool,                  
    #[export] user_read_follows : bool,                 
    #[export] user_read_moderated_channels : bool,      
    #[export] user_read_subscriptions : bool,           
    #[export] user_read_whispers : bool,                
    #[export] user_write_chat : bool,

    #[export_group(name="Whispers")]                
    #[export] whispers_read : bool,                     
}

impl HadalythTwitchScopes {
    pub fn get_twitch_api_scopes(&mut self) -> Vec<twitch_api::twitch_oauth2::Scope> {
        let mut scopes = vec![];

        if self.analytics_read_extensions { scopes.push(twitch_api::twitch_oauth2::Scope::AnalyticsReadExtensions); }
        if self.analytics_read_games { scopes.push(twitch_api::twitch_oauth2::Scope::AnalyticsReadGames); }
        if self.bits_read { scopes.push(twitch_api::twitch_oauth2::Scope::BitsRead); }
        if self.channel_bot { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelBot); }
        if self.channel_edit_commercial { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelEditCommercial); }
        if self.channel_manage_ads { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageAds); }
        if self.channel_manage_broadcast { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageBroadcast); }
        if self.channel_manage_clips { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageClips); }
        if self.channel_manage_extensions { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageExtensions); }
        if self.channel_manage_guest_star { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageGuestStar); }
        if self.channel_manage_moderators { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageModerators); }
        if self.channel_manage_polls { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManagePolls); }
        if self.channel_manage_predictions { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManagePredictions); }
        if self.channel_manage_raids { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageRaids); }
        if self.channel_manage_redemptions { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageRedemptions); }
        if self.channel_manage_schedule { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageSchedule); }
        if self.channel_manage_videos { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageVideos); }
        if self.channel_manage_vips { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelManageVips); }
        if self.channel_moderate { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelModerate); }
        if self.channel_read_ads { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadAds); }
        if self.channel_read_charity { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadCharity); }
        if self.channel_read_editors { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadEditors); }
        if self.channel_read_goals { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadGoals); }
        if self.channel_read_guest_star { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadGuestStar); }
        if self.channel_read_hype_train { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadHypeTrain); }
        if self.channel_read_polls { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadPolls); }
        if self.channel_read_predictions { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadPredictions); }
        if self.channel_read_redemptions { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadRedemptions); }
        if self.channel_read_stream_key { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadStreamKey); }
        if self.channel_read_subscriptions { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadSubscriptions); }
        if self.channel_read_vips { scopes.push(twitch_api::twitch_oauth2::Scope::ChannelReadVips); }
        if self.chat_edit { scopes.push(twitch_api::twitch_oauth2::Scope::ChatEdit); }
        if self.chat_read { scopes.push(twitch_api::twitch_oauth2::Scope::ChatRead); }
        if self.clips_edit { scopes.push(twitch_api::twitch_oauth2::Scope::ClipsEdit); }
        if self.editor_manage_clips { scopes.push(twitch_api::twitch_oauth2::Scope::EditorManageClips); }
        if self.moderation_read { scopes.push(twitch_api::twitch_oauth2::Scope::ModerationRead); }
        if self.moderator_manage_announcements { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageAnnouncements); }
        if self.moderator_manage_automod { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageAutoMod); }
        if self.moderator_manage_automod_settings { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageAutomodSettings); }
        if self.moderator_manage_banned_users { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageBannedUsers); }
        if self.moderator_manage_blocked_terms { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageBlockedTerms); }
        if self.moderator_manage_chat_messages { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageChatMessages); }
        if self.moderator_manage_chat_settings { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageChatSettings); }
        if self.moderator_manage_guest_star { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageGuestStar); }
        if self.moderator_manage_shield_mode { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageShieldMode); }
        if self.moderator_manage_shoutouts { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageShoutouts); }
        if self.moderator_manage_suspicious_users { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageSuspiciousUsers); }
        if self.moderator_manage_unban_requests { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageUnbanRequests); }
        if self.moderator_manage_warnings { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorManageWarnings); }
        if self.moderator_read_automod_settings { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadAutomodSettings); }
        if self.moderator_read_banned_users { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadBannedUsers); }
        if self.moderator_read_blocked_terms { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadBlockedTerms); }
        if self.moderator_read_chat_messages { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadChatMessages); }
        if self.moderator_read_chat_settings { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadChatSettings); }
        if self.moderator_read_chatters { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadChatters); }
        if self.moderator_read_followers { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadFollowers); }
        if self.moderator_read_guest_star { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadGuestStar); }
        if self.moderator_read_moderators { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadModerators); }
        if self.moderator_read_shield_mode { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadShieldMode); }
        if self.moderator_read_shoutouts { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadShoutouts); }
        if self.moderator_read_suspicious_users { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadSuspiciousUsers); }
        if self.moderator_read_unban_requests { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadUnbanRequests); }
        if self.moderator_read_vips { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadVips); }
        if self.moderator_read_warnings { scopes.push(twitch_api::twitch_oauth2::Scope::ModeratorReadWarnings); }
        if self.user_bot { scopes.push(twitch_api::twitch_oauth2::Scope::UserBot); }
        if self.user_edit { scopes.push(twitch_api::twitch_oauth2::Scope::UserEdit); }
        if self.user_edit_broadcast { scopes.push(twitch_api::twitch_oauth2::Scope::UserEditBroadcast); }
        if self.user_manage_blocked_users { scopes.push(twitch_api::twitch_oauth2::Scope::UserManageBlockedUsers); }
        if self.user_manage_chat_color { scopes.push(twitch_api::twitch_oauth2::Scope::UserManageChatColor); }
        if self.user_manage_whispers { scopes.push(twitch_api::twitch_oauth2::Scope::UserManageWhispers); }
        if self.user_read_blocked_users { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadBlockedUsers); }
        if self.user_read_broadcast { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadBroadcast); }
        if self.user_read_chat { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadChat); }
        if self.user_read_email { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadEmail); }
        if self.user_read_emotes { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadEmotes); }
        if self.user_read_follows { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadFollows); }
        if self.user_read_moderated_channels { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadModeratedChannels); }
        if self.user_read_subscriptions { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadSubscriptions); }
        if self.user_read_whispers { scopes.push(twitch_api::twitch_oauth2::Scope::UserReadWhispers); }
        if self.user_write_chat { scopes.push(twitch_api::twitch_oauth2::Scope::UserWriteChat); }
        if self.whispers_read { scopes.push(twitch_api::twitch_oauth2::Scope::WhispersRead); }

        return scopes;
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HadalythTwitchEventSubs {
    base : Base<Resource>,

    #[export_group(name="Automod")]
    // #[export] pub automod_message_hold_v1 : bool,
    #[export] pub automod_message_hold_v2 : bool,
    // #[export] pub automod_message_update_v1 : bool,
    #[export] pub automod_message_update_v2 : bool,
    #[export] pub automod_settings_update_v1 : bool,
    #[export] pub automod_terms_update_v1 : bool,

    #[export_group(name="Channel")]
    #[export] pub channel_ad_break_begin_v1 : bool,
    #[export] pub channel_bits_use_v1 : bool,
    #[export] pub channel_chat_clear_v1 : bool,
    #[export] pub channel_chat_clear_user_messages_v1 : bool,
    #[export] pub channel_chat_message_v1 : bool,
    #[export] pub channel_chat_message_delete_v1 : bool,
    #[export] pub channel_chat_notification_v1 : bool,
    #[export] pub channel_chat_user_message_hold_v1 : bool,
    #[export] pub channel_chat_user_message_update_v1 : bool,
    #[export] pub channel_chat_settings_update_v1 : bool,
    #[export] pub channel_charity_campaign_donate_v1 : bool,
    #[export] pub channel_charity_campaign_progress_v1 : bool,
    #[export] pub channel_charity_campaign_start_v1 : bool,
    #[export] pub channel_charity_campaign_stop_v1 : bool,
    #[export] pub channel_update_v2 : bool,
    #[export] pub channel_follow_v2 : bool,
    #[export] pub channel_subscribe_v1 : bool,
    #[export] pub channel_cheer_v1 : bool,
    #[export] pub channel_ban_v1 : bool,
    #[export] pub channel_unban_v1 : bool,
    #[export] pub channel_unban_request_create_v1 : bool,
    #[export] pub channel_unban_request_resolve_v1 : bool,
    #[export] pub channel_vip_add_v1 : bool,
    #[export] pub channel_vip_remove_v1 : bool,
    #[export] pub channel_warning_acknowledge_v1 : bool,
    #[export] pub channel_warning_send_v1 : bool,
    #[export] pub channel_points_automatic_reward_redemption_add_v1 : bool,
    #[export] pub channel_points_custom_reward_add_v1 : bool,
    #[export] pub channel_points_custom_reward_update_v1 : bool,
    #[export] pub channel_points_custom_reward_remove_v1 : bool,
    #[export] pub channel_points_custom_reward_redemption_add_v1 : bool,
    #[export] pub channel_points_custom_reward_redemption_update_v1 : bool,
    #[export] pub channel_poll_begin_v1 : bool,
    #[export] pub channel_poll_progress_v1 : bool,
    #[export] pub channel_poll_end_v1 : bool,
    #[export] pub channel_prediction_begin_v1 : bool,
    #[export] pub channel_prediction_progress_v1 : bool,
    #[export] pub channel_prediction_lock_v1 : bool,
    #[export] pub channel_prediction_end_v1 : bool,
    #[export] pub channel_raid_v1 : bool,
    #[export] pub channel_shared_chat_begin_v1 : bool,
    #[export] pub channel_shared_chat_end_v1 : bool,
    #[export] pub channel_shared_chat_update_v1 : bool,
    #[export] pub channel_shield_mode_begin_v1 : bool,
    #[export] pub channel_shield_mode_end_v1 : bool,
    #[export] pub channel_shoutout_create_v1 : bool,
    #[export] pub channel_shoutout_receive_v1 : bool,
    #[export] pub channel_suspicious_user_message_v1 : bool,
    #[export] pub channel_suspicious_user_update_v1 : bool,
    #[export] pub channel_goal_begin_v1 : bool,
    #[export] pub channel_goal_progress_v1 : bool,
    #[export] pub channel_goal_end_v1 : bool,
    #[export] pub channel_hype_train_begin_v1 : bool,
    #[export] pub channel_hype_train_progress_v1 : bool,
    #[export] pub channel_hype_train_end_v1 : bool,
    // #[export] pub channel_moderate_v1 : bool,
    #[export] pub channel_moderate_v2 : bool,
    #[export] pub channel_moderator_add_v1 : bool,
    #[export] pub channel_moderator_remove_v1 : bool,
    #[export] pub channel_subscription_end_v1 : bool,
    #[export] pub channel_subscription_gift_v1 : bool,
    #[export] pub channel_subscription_message_v1 : bool,

    // #[export] pub conduit_shard_disabled_v1 : bool,
    // #[export] pub extension_bits_transaction_create_v1 : bool,
    
    #[export_group(name="Stream")]
    #[export] pub stream_online_v1 : bool,
    #[export] pub stream_offline_v1 : bool,
    
    #[export_group(name="User")]
    #[export] pub user_update_v1 : bool,
    // #[export] pub user_authorization_grant_v1 : bool,
    // #[export] pub user_authorization_revoke_v1 : bool,
    #[export] pub user_whisper_message_v1 : bool,
    }
pub struct TwitchApiEventSubs {
    pub automod_message_hold_v2 : bool,
    pub automod_message_update_v2 : bool,
    pub automod_settings_update_v1 : bool,
    pub automod_terms_update_v1 : bool,
    pub channel_ad_break_begin_v1 : bool,
    pub channel_bits_use_v1 : bool,
    pub channel_chat_clear_v1 : bool,
    pub channel_chat_clear_user_messages_v1 : bool,
    pub channel_chat_message_v1 : bool,
    pub channel_chat_message_delete_v1 : bool,
    pub channel_chat_notification_v1 : bool,
    pub channel_chat_user_message_hold_v1 : bool,
    pub channel_chat_user_message_update_v1 : bool,
    pub channel_chat_settings_update_v1 : bool,
    pub channel_charity_campaign_donate_v1 : bool,
    pub channel_charity_campaign_progress_v1 : bool,
    pub channel_charity_campaign_start_v1 : bool,
    pub channel_charity_campaign_stop_v1 : bool,
    pub channel_update_v2 : bool,
    pub channel_follow_v2 : bool,
    pub channel_subscribe_v1 : bool,
    pub channel_cheer_v1 : bool,
    pub channel_ban_v1 : bool,
    pub channel_unban_v1 : bool,
    pub channel_unban_request_create_v1 : bool,
    pub channel_unban_request_resolve_v1 : bool,
    pub channel_vip_add_v1 : bool,
    pub channel_vip_remove_v1 : bool,
    pub channel_warning_acknowledge_v1 : bool,
    pub channel_warning_send_v1 : bool,
    pub channel_points_automatic_reward_redemption_add_v1 : bool,
    pub channel_points_custom_reward_add_v1 : bool,
    pub channel_points_custom_reward_update_v1 : bool,
    pub channel_points_custom_reward_remove_v1 : bool,
    pub channel_points_custom_reward_redemption_add_v1 : bool,
    pub channel_points_custom_reward_redemption_update_v1 : bool,
    pub channel_poll_begin_v1 : bool,
    pub channel_poll_progress_v1 : bool,
    pub channel_poll_end_v1 : bool,
    pub channel_prediction_begin_v1 : bool,
    pub channel_prediction_progress_v1 : bool,
    pub channel_prediction_lock_v1 : bool,
    pub channel_prediction_end_v1 : bool,
    pub channel_raid_v1 : bool,
    pub channel_shared_chat_begin_v1 : bool,
    pub channel_shared_chat_end_v1 : bool,
    pub channel_shared_chat_update_v1 : bool,
    pub channel_shield_mode_begin_v1 : bool,
    pub channel_shield_mode_end_v1 : bool,
    pub channel_shoutout_create_v1 : bool,
    pub channel_shoutout_receive_v1 : bool,
    pub channel_suspicious_user_message_v1 : bool,
    pub channel_suspicious_user_update_v1 : bool,
    pub channel_goal_begin_v1 : bool,
    pub channel_goal_progress_v1 : bool,
    pub channel_goal_end_v1 : bool,
    pub channel_hype_train_begin_v1 : bool,
    pub channel_hype_train_progress_v1 : bool,
    pub channel_hype_train_end_v1 : bool,
    pub channel_moderate_v2 : bool,
    pub channel_moderator_add_v1 : bool,
    pub channel_moderator_remove_v1 : bool,
    pub stream_online_v1 : bool,
    pub stream_offline_v1 : bool,
    pub user_update_v1 : bool,
    pub user_whisper_message_v1 : bool,
    pub channel_subscription_end_v1 : bool,
    pub channel_subscription_gift_v1 : bool,
    pub channel_subscription_message_v1 : bool
}
impl HadalythTwitchEventSubs {
    pub fn get_twitch_api_eventsubs(&self) -> TwitchApiEventSubs {
        TwitchApiEventSubs {
            automod_message_hold_v2 : self.automod_message_hold_v2,
            automod_message_update_v2 : self.automod_message_update_v2,
            automod_settings_update_v1 : self.automod_settings_update_v1,
            automod_terms_update_v1 : self.automod_terms_update_v1,
            channel_ad_break_begin_v1 : self.channel_ad_break_begin_v1,
            channel_bits_use_v1 : self.channel_bits_use_v1,
            channel_chat_clear_v1 : self.channel_chat_clear_v1,
            channel_chat_clear_user_messages_v1 : self.channel_chat_clear_user_messages_v1,
            channel_chat_message_v1 : self.channel_chat_message_v1,
            channel_chat_message_delete_v1 : self.channel_chat_message_delete_v1,
            channel_chat_notification_v1 : self.channel_chat_notification_v1,
            channel_chat_user_message_hold_v1 : self.channel_chat_user_message_hold_v1,
            channel_chat_user_message_update_v1 : self.channel_chat_user_message_update_v1,
            channel_chat_settings_update_v1 : self.channel_chat_settings_update_v1,
            channel_charity_campaign_donate_v1 : self.channel_charity_campaign_donate_v1,
            channel_charity_campaign_progress_v1 : self.channel_charity_campaign_progress_v1,
            channel_charity_campaign_start_v1 : self.channel_charity_campaign_start_v1,
            channel_charity_campaign_stop_v1 : self.channel_charity_campaign_stop_v1,
            channel_update_v2 : self.channel_update_v2,
            channel_follow_v2 : self.channel_follow_v2,
            channel_subscribe_v1 : self.channel_subscribe_v1,
            channel_cheer_v1 : self.channel_cheer_v1,
            channel_ban_v1 : self.channel_ban_v1,
            channel_unban_v1 : self.channel_unban_v1,
            channel_unban_request_create_v1 : self.channel_unban_request_create_v1,
            channel_unban_request_resolve_v1 : self.channel_unban_request_resolve_v1,
            channel_vip_add_v1 : self.channel_vip_add_v1,
            channel_vip_remove_v1 : self.channel_vip_remove_v1,
            channel_warning_acknowledge_v1 : self.channel_warning_acknowledge_v1,
            channel_warning_send_v1 : self.channel_warning_send_v1,
            channel_points_automatic_reward_redemption_add_v1 : self.channel_points_automatic_reward_redemption_add_v1,
            channel_points_custom_reward_add_v1 : self.channel_points_custom_reward_add_v1,
            channel_points_custom_reward_update_v1 : self.channel_points_custom_reward_update_v1,
            channel_points_custom_reward_remove_v1 : self.channel_points_custom_reward_remove_v1,
            channel_points_custom_reward_redemption_add_v1 : self.channel_points_custom_reward_redemption_add_v1,
            channel_points_custom_reward_redemption_update_v1 : self.channel_points_custom_reward_redemption_update_v1,
            channel_poll_begin_v1 : self.channel_poll_begin_v1,
            channel_poll_progress_v1 : self.channel_poll_progress_v1,
            channel_poll_end_v1 : self.channel_poll_end_v1,
            channel_prediction_begin_v1 : self.channel_prediction_begin_v1,
            channel_prediction_progress_v1 : self.channel_prediction_progress_v1,
            channel_prediction_lock_v1 : self.channel_prediction_lock_v1,
            channel_prediction_end_v1 : self.channel_prediction_end_v1,
            channel_raid_v1 : self.channel_raid_v1,
            channel_shared_chat_begin_v1 : self.channel_shared_chat_begin_v1,
            channel_shared_chat_end_v1 : self.channel_shared_chat_end_v1,
            channel_shared_chat_update_v1 : self.channel_shared_chat_update_v1,
            channel_shield_mode_begin_v1 : self.channel_shield_mode_begin_v1,
            channel_shield_mode_end_v1 : self.channel_shield_mode_end_v1,
            channel_shoutout_create_v1 : self.channel_shoutout_create_v1,
            channel_shoutout_receive_v1 : self.channel_shoutout_receive_v1,
            channel_suspicious_user_message_v1 : self.channel_suspicious_user_message_v1,
            channel_suspicious_user_update_v1 : self.channel_suspicious_user_update_v1,
            channel_goal_begin_v1 : self.channel_goal_begin_v1,
            channel_goal_progress_v1 : self.channel_goal_progress_v1,
            channel_goal_end_v1 : self.channel_goal_end_v1,
            channel_hype_train_begin_v1 : self.channel_hype_train_begin_v1,
            channel_hype_train_progress_v1 : self.channel_hype_train_progress_v1,
            channel_hype_train_end_v1 : self.channel_hype_train_end_v1,
            channel_moderate_v2 : self.channel_moderate_v2,
            channel_moderator_add_v1 : self.channel_moderator_add_v1,
            channel_moderator_remove_v1 : self.channel_moderator_remove_v1,
            stream_online_v1 : self.stream_online_v1,
            stream_offline_v1 : self.stream_offline_v1,
            user_update_v1 : self.user_update_v1,
            user_whisper_message_v1 : self.user_whisper_message_v1,
            channel_subscription_end_v1 : self.channel_subscription_end_v1,
            channel_subscription_gift_v1 : self.channel_subscription_gift_v1,
            channel_subscription_message_v1 : self.channel_subscription_message_v1
        }
    }
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct Broadcaster {
    #[var]
    user_id : GString,
    
    #[var]
    user_login : GString,
    
    #[var]
    user_name : GString
}

#[godot_api]
impl Broadcaster {
    #[func]
    pub fn create(
        user_id : GString,
        user_login : GString,
        user_name : GString,
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                user_id,
                user_login,
                user_name
            }
        )
    }
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct User {
    #[var]
    user_id : GString,
    
    #[var]
    user_login : GString,
    
    #[var]
    user_name : GString
}

#[godot_api]
impl User {
    #[func]
    pub fn create(
        user_id : GString,
        user_login : GString,
        user_name : GString,
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                user_id,
                user_login,
                user_name
            }
        )
    }
}


#[derive(GodotClass)]
#[class(no_init)]
pub struct Emote {
    #[var]
    id : GString,

    #[var]
    emote_set_id : GString,

    #[var]
    owner_id : GString,

    #[var]
    format : GString,
}

#[godot_api]
impl Emote {
    #[func]
    pub fn create(
        id : GString,
        emote_set_id : GString,
        owner_id : GString,
        format : GString
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                id,
                emote_set_id,
                owner_id,
                format
            }
        )
    }
}


#[derive(GodotClass)]
#[class(no_init)]
pub struct Fragment {
    #[var]
    text : GString,

    #[var]
    emote : Option<Gd<Emote>>,
}


#[godot_api]
impl Fragment {
    #[func]
    pub fn create(
        text : GString,
        emote : Option<Gd<Emote>>
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                text,
                emote
            }
        );
    }
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct Message {
    #[var]
    text : GString,

    #[var]
    fragments : Array<Gd<Fragment>>
}

#[godot_api]
impl Message {
    #[func]
    pub fn create(
        text : GString,
        fragments : Array<Gd<Fragment>>
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                text, 
                fragments
            }
        )
    }
}

// ChannelChatMessageV1Payload { 
//     broadcaster_user_id: "1361712337", 
//     broadcaster_user_name: "CLAYMORE_DEV", 
//     broadcaster_user_login: "claymore_dev", 
//     chatter_user_id: "1361712337", 
//     chatter_user_name: "CLAYMORE_DEV", 
//     chatter_user_login: "claymore_dev", 
//     message_id: "4d9a4a6c-12bc-4d18-b5b6-105b17e16126", 
//     message: Message { 
//         text: "asdf SeemsGood Kappa asdf asdf asdf", 
//         fragments: [
//             Text { 
//                 text: "asdf " 
//             }, 
//             Emote { 
//                 text: "SeemsGood", 
//                 emote: Emote { id: "64138", emote_set_id: "0", owner_id: "0", format: [Static] } 
//             }, 
//             Text { 
//                 text: " " 
//             }, 
//             Emote { 
//                 text: "Kappa", 
//                 emote: Emote { id: "25", emote_set_id: "0", owner_id: "0", format: [Static] } 
//             }, 
//             Text { 
//                 text: " asdf asdf asdf" 
//             }
//         ] 
//     }, 
//     message_type: Text, 
//     badges: [Badge { set_id: "broadcaster", id: "1", info: "" }, Badge { set_id: "subscriber", id: "3006", info: "8" }, Badge { set_id: "premium", id: "1", info: "" }], cheer: None, color: "#000000", reply: None, channel_points_custom_reward_id: None, channel_points_animation_id: None, source_broadcaster_user_id: None, source_broadcaster_user_name: None, source_broadcaster_user_login: None, source_message_id: None, source_badges: [], is_source_only: None }
