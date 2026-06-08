use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HadalythTwitchClientId {
    base: Base<Resource>,

    #[export]
    pub client_id : GString,

}

impl HadalythTwitchClientId {
    pub fn get_twitch_api_client_id(&mut self) -> twitch_oauth2::ClientId {
        return twitch_api::twitch_oauth2::ClientId::new(self.client_id.to_string());
    }
}