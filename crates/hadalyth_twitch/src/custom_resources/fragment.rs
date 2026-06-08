use godot::prelude::*;

use crate::custom_resources::emote::Emote;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Fragment {
    #[var]
    text: GString,

    #[var]
    emote: Option<Gd<Emote>>,
}

#[godot_api]
impl Fragment {
    #[func]
    pub fn create(text: GString, emote: Option<Gd<Emote>>) -> Gd<Self> {
        return Gd::from_object(Self { text, emote });
    }
}
