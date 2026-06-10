use godot::prelude::*;

use crate::custom_resources::cheermote::Cheermote;
use crate::custom_resources::emote::Emote;
use crate::custom_resources::mention::Mention;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Fragment {
    #[var]
    text: GString,

    #[var]
    cheermote: Option<Gd<Cheermote>>,

    #[var]
    emote: Option<Gd<Emote>>,

    #[var]
    mention: Option<Gd<Mention>>,
}

#[godot_api]
impl Fragment {
    #[func]
    pub fn create(
        text: GString,
        cheermote: Option<Gd<Cheermote>>,
        emote: Option<Gd<Emote>>,
        mention: Option<Gd<Mention>>,
    ) -> Gd<Self> {
        return Gd::from_object(Self {
            text,
            cheermote,
            emote,
            mention,
        });
    }
}
