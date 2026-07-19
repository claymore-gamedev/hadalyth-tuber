use godot::prelude::*;

use crate::custom_config::hadalyth_twitch_client_id::HadalythTwitchClientId;
use crate::custom_config::hadalyth_twitch_eventsubs::HadalythTwitchEventSubs;
use crate::custom_config::hadalyth_twitch_scopes::HadalythTwitchScopes;
use crate::custom_events::socket_event::SocketEvent;
use crate::custom_events::twitch_event::TwitchEvent;

use crate::custom_resources::badge::Badge;
use crate::custom_resources::bits_custom_power_up::BitsCustomPowerUp;
use crate::custom_resources::charity::Charity;
use crate::custom_resources::currency::Currency;
use crate::custom_resources::message::Message;
use crate::custom_resources::reply::Reply;
use crate::custom_resources::source::Source;
use crate::custom_resources::timestamp::Timestamp;
use crate::custom_resources::user::User;

use crate::custom_enums::automod_message_status::AutomodMessageStatus;

use crate::hadalyth_twitch_async::*;

const REFRESH_TOKEN_PATH: &str = "user://refresh_token.cfg";
const REFRESH_CFG_SECTION_KEY: &str = "REFRESH_TOKEN";
const REFRESH_CFG_REFRESH_TOKEN_KEY: &str = "refresh_token";


#[derive(GodotClass)]
#[class(base=Node)]
pub struct HadalythTwitch {
    client_id: Option<Gd<HadalythTwitchClientId>>,
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
        // Try to start a tokio runtime for later async tasks
        let runtime = tokio::runtime::Runtime::new();
        let Ok(runtime) = runtime else {
            godot_error!("TwitchError: tokio runtime failed to initialize");
            return;
        };
        self.runtime = Some(runtime);

        let twitch: twitch_api::helix::HelixClient<'static, reqwest::Client> =
            twitch_api::helix::HelixClient::default();
        self.twitch = Some(twitch);
    }

    fn process(&mut self, _delta: f64) {
        // Process all events sent from other threads
        while let Ok(event) = self.rx.try_recv() {
            match event {
                TwitchEvent::Debug(message) => {
                    godot_print!("TwitchEvent::Debug: {}", message);
                }

                TwitchEvent::RefreshTokenStatus(token) => {
                    godot_print!("TwitchEvent::RefreshTokenStatus");
                    self.token = token;
                    let status = self.token.is_some();
                    self.signals().refresh_token_status().emit(status);
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

                TwitchEvent::RefreshTokenUpdate(refresh_token) => match refresh_token {
                    Some(refresh_token) => {
                        let mut refresh_cfg = godot::classes::ConfigFile::new_gd();
                        refresh_cfg.set_value(
                            REFRESH_CFG_SECTION_KEY,
                            REFRESH_CFG_REFRESH_TOKEN_KEY,
                            &refresh_token.as_str().to_godot().to_variant(),
                        );
                        refresh_cfg.save(REFRESH_TOKEN_PATH);
                    }
                    None => {}
                },

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

    // MESSAGE TYPE CONSTS

    #[constant]
    pub const MESSAGE_TYPE_TEST: i64 = 0;
    #[constant]
    pub const MESSAGE_TYPE_CHANNEL_POINTS_HIGHLIGHTED: i64 = 1;
    #[constant]
    pub const MESSAGE_TYPE_CHANNEL_POINTS_SUB_ONLY: i64 = 2;
    #[constant]
    pub const MESSAGE_TYPE_USER_INTRO: i64 = 3;
    #[constant]
    pub const MESSAGE_TYPE_POWER_UPS_GIGANTIFIED_EMOTE: i64 = 4;
    #[constant]
    pub const MESSAGE_TYPE_POWER_UPS_MESSAGE_EFFECT: i64 = 5;

    // START OF EVENTSUB SIGNALS

    #[signal]
    pub fn recv_automod_message_hold_v2(
        broadcaster: Option<Gd<User>>,
        user: Option<Gd<User>>,
        message: Option<Gd<Message>>,
        message_id: String,
        held_at: String,
        reason: String,
    );
    #[signal]
    pub fn recv_automod_message_update_v2(
        broadcaster: Option<Gd<User>>,
        moderator: Option<Gd<User>>,
        user: Option<Gd<User>>,
        message: Option<Gd<Message>>,
        message_id: String,
        held_at: String,
        reason: String,
        status: String,
    );
    #[signal]
    pub fn recv_automod_settings_update_v1(
        broadcaster: Option<Gd<User>>,
        moderator: Option<Gd<User>>,
        aggression: i64,
        bullying: i64,
        disability: i64,
        misogyny: i64,
        race_ethnicity_or_religion: i64,
        sex_based_terms: i64,
        sexuality_sex_or_gender: i64,
        swearing: i64,
        overall_level: i64,
    );
    #[signal]
    pub fn recv_automod_terms_update_v1(
        broadcaster: Option<Gd<User>>,
        moderator: Option<Gd<User>>,
        from_automod: bool,
        action: String,
        terms: PackedStringArray,
    );
    #[signal]
    pub fn recv_channel_ad_break_begin_v1(
        broadcaster: Option<Gd<User>>,
        requester: Option<Gd<User>>,
        started_at: Option<Gd<Timestamp>>,
        duration_seconds: i64,
        is_automatic: bool,
    );
    #[signal]
    pub fn recv_channel_bits_use_v1(
        broadcaster: Option<Gd<User>>,
        user: Option<Gd<User>>,
        message: Option<Gd<Message>>,
        bits: i64,
        bits_type: i64,
        bits_custom_power_up: Option<Gd<BitsCustomPowerUp>>,
    );
    #[signal]
    pub fn recv_channel_chat_clear_v1(broadcaster: Option<Gd<User>>);
    #[signal]
    pub fn recv_channel_chat_clear_user_messages_v1(
        broadcaster: Option<Gd<User>>,
        user: Option<Gd<User>>,
    );
    #[signal]
    pub fn recv_channel_chat_message_v1(
        broadcaster: Option<Gd<User>>,
        chatter: Option<Gd<User>>,
        message: Option<Gd<Message>>,
        message_id: String,
        message_type: i64,
        badges: Array<Gd<Badge>>,
        channel_points_animation_id: String,
        channel_points_custom_reward_id: String,
        cheer: i64,
        color: String,
        is_source_only: bool,
        reply: Option<Gd<Reply>>,
        source: Option<Gd<Source>>,
    );
    #[signal]
    pub fn recv_channel_chat_message_delete_v1(
        broadcaster: Option<Gd<User>>,
        user: Option<Gd<User>>,
        message_id: String,
    );
    #[signal]
    pub fn recv_channel_chat_notification_v1();
    #[signal]
    pub fn recv_channel_chat_user_message_hold_v1(
        broadcaster: Option<Gd<User>>,
        user: Option<Gd<User>>,
        message_id: String,
        message: Option<Gd<Message>>    
    );
    #[signal]
    pub fn recv_channel_chat_user_message_update_v1(
        broadcaster : Option<Gd<User>>,
        user : Option<Gd<User>>,
        message_id : String,
        message : Option<Gd<Message>>,
        status : AutomodMessageStatus,
    );
    #[signal]
    pub fn recv_channel_chat_settings_update_v1(
        broadcaster : Option<Gd<User>>,
        emote_mode : bool,
        follower_mode : bool,
        follower_mode_duration_minutes : i64,
        slow_mode : bool,
        slow_mode_wait_time_seconds : i64,
        subscriber_mode : bool,
        unique_chat_mode : bool
    );
    #[signal]
    pub fn recv_channel_charity_campaign_donate_v1(
        broadcaster : Option<Gd<User>>,
        user : Option<Gd<User>>,
        charity : Option<Gd<Charity>>,
        amount : Option<Gd<Currency>>,
        campaign_id : String,
        donation_id : String
    );
    #[signal]
    pub fn recv_channel_charity_campaign_progress_v1(
        broadcaster : Option<Gd<User>>,
        charity : Option<Gd<Charity>>,
        current_amount : Option<Gd<Currency>>,
        target_amount : Option<Gd<Currency>>,
        campaign_id : String
    );
    #[signal]
    pub fn recv_channel_charity_campaign_start_v1(
        broadcaster : Option<Gd<User>>,
        charity : Option<Gd<Charity>>,
        current_amount : Option<Gd<Currency>>,
        target_amount : Option<Gd<Currency>>,
        started_at : Option<Gd<Timestamp>>,
        campaign_id : String
    );
    #[signal]
    pub fn recv_channel_charity_campaign_stop_v1(
        broadcaster : Option<Gd<User>>,
        charity : Option<Gd<Charity>>,
        current_amount : Option<Gd<Currency>>,
        target_amount : Option<Gd<Currency>>,
        stopped_at : Option<Gd<Timestamp>>,
        campaign_id : String
    );
    #[signal]
    pub fn recv_channel_update_v2(
        &broadcaster : Option<Gd<User>>,
        category_id : String,
        category_name : String,
        content_classification_labels : PackedStringArray,
        language : String,
        title : String
    );
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
    pub fn refresh_token_status(status: bool);
    #[signal]
    pub fn device_user_token_status(status: bool);
    #[signal]
    pub fn twitch_socket_status(status: bool);
}


// Init functions
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
    fn set_client_id(&mut self, client_id: Option<Gd<HadalythTwitchClientId>>) {
        self.client_id = client_id;
    }

    #[func]
    pub fn _init_refresh_user_token(&mut self) {
        let Some(ref runtime) = self.runtime else {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::RefreshTokenStatus(None));
            godot_print!("Tokio runtime not initialized");
            return;
        };

        let Some(ref twitch) = self.twitch else {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::RefreshTokenStatus(None));
            godot_print!("Failed to initialize helix client");
            return;
        };

        let Some(ref mut client_id) = self.client_id else {
            godot_print!("Client ID not set");
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::RefreshTokenStatus(None));
            return;
        };
        let client_id = client_id.bind_mut().get_twitch_api_client_id();

        // Try load refresh token
        if !godot::classes::FileAccess::file_exists(REFRESH_TOKEN_PATH) {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::RefreshTokenStatus(None));
            return;
        };
        let mut refresh_token_cfg = godot::classes::ConfigFile::new_gd();
        refresh_token_cfg.load(REFRESH_TOKEN_PATH);

        let refresh_token =
            refresh_token_cfg.get_value(REFRESH_CFG_SECTION_KEY, REFRESH_CFG_REFRESH_TOKEN_KEY);
        let refresh_token = String::try_from_variant(&refresh_token);
        let Ok(refresh_token) = refresh_token else {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::RefreshTokenStatus(None));
            return;
        };

        // Clone parameters
        let tx = self.tx.clone();
        let http_client = twitch.clone_client();
        let refresh_token = twitch_api::twitch_oauth2::RefreshToken::new(refresh_token);

        // Spawn device user token auth
        runtime.spawn(init_refresh_user_token_async(
            tx,
            http_client,
            client_id,
            refresh_token,
        ));
    }

    #[func]
    pub fn _init_device_user_token(&mut self) {
        let Some(ref runtime) = self.runtime else {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::DeviceUserTokenStatus(None));
            godot_print!("Tokio runtime not initialized");
            return;
        };

        let Some(ref twitch) = self.twitch else {
            let tx = self.tx.clone();
            let _ = tx.send(TwitchEvent::DeviceUserTokenStatus(None));
            godot_print!("Failed to initialize helix client");
            return;
        };

        let Some(ref mut client_id) = self.client_id else {
            godot_print!("Client ID not set");
            return;
        };
        let client_id = client_id.bind_mut().get_twitch_api_client_id();

        let Some(ref mut scopes) = self.scopes else {
            godot_print!("Scopes not set");
            return;
        };
        let scopes = scopes.bind_mut().get_twitch_api_scopes();

        // Clone parameters
        let tx = self.tx.clone();
        let http_client = twitch.clone_client();
        let token_builder =
            twitch_api::twitch_oauth2::DeviceUserTokenBuilder::new(client_id, scopes);

        // Spawn device user token auth
        runtime.spawn(init_device_user_token_async(tx, http_client, token_builder));
    }

    #[func]
    pub fn _init_twitch_socket(&mut self) {
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
    pub fn _init_subscribe_eventsubs(&mut self) {
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
    fn init_twitch(&mut self) {
        let this = self.to_gd();
        godot::task::spawn(async move {
            init_twitch_async(this).await;
        });
    }

    #[func]
    fn _kill_helix_client(&mut self) {
        self.twitch = None;
    }

    #[func]
    fn _kill_twitch_socket(&mut self) {
        let Some(ref socket_tx) = self.socket_tx else {
            return;
        };
        let _ = socket_tx.send(SocketEvent::SocketClose);
    }

    #[func]
    fn kill_twitch(&mut self) {
        self._kill_helix_client();
        self._kill_twitch_socket();
    }
}

// CDN helper functions
#[godot_api(secondary)]
impl HadalythTwitch {

    // Nothing here yet

    // MVP probably something like:
    // - Emote downloading/caching/serving
    // - Profile downloading/caching/serving
    // - Badge downloading/caching/serving
    // - 

    // I think I should make a godot class that listens for a signal containing the
    // Texture2D that was requested and updates itself based on that.
    // Or I could just hold a reference to the returned texture in this singleton and manage it from here?
    // I could hold a reference to the texture in a hashset until I have the resource figured out
    // Once the resource is figured out, just update the texture and remove it from the hashset
    // Or I could have a hashmap of vectors of textures to each emote, badge, and profile?
    // I like that approach I think.  I would need a placeholder for each that I can load immediately.
    // That's not that bad.

    // Okay.  I am going to lazily populate a hashmap of Texture2D
    // If I can't immediately load a texture from disk because it doesnt exist or it's older than X days, I will
    // make a request to download the new texture from twitch's server.
    // Once that is resolved I will push the texture into the control node asynchronously.

}

// Helix endpoint functions
#[godot_api(secondary)]
impl HadalythTwitch {

    // Nothing here yet

}