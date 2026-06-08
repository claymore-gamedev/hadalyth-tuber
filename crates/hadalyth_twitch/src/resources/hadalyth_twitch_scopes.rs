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