// Events to be processed in the main thread
pub enum TwitchEvent {
    Debug(String),

    RefreshTokenStatus(Option<twitch_api::twitch_oauth2::UserToken>),
    DeviceUserTokenRequest(String, u64),
    DeviceUserTokenStatus(Option<twitch_api::twitch_oauth2::UserToken>),

    RefreshTokenUpdate(Option<String>),

    TwitchSocketStatus(Option<String>),

    TwitchEventsubReceived(twitch_api::eventsub::Event),
}
