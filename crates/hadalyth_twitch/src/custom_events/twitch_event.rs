// Events to be processed in the main thread
pub enum TwitchEvent {
    Debug(String),

    DeviceUserTokenRequest(String, u64),
    DeviceUserTokenStatus(Option<twitch_api::twitch_oauth2::UserToken>),

    TwitchSocketStatus(Option<String>),

    TwitchEventsubReceived(twitch_api::eventsub::Event),
}
