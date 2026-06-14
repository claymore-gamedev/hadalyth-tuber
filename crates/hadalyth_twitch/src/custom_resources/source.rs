use crate::custom_resources::{badge::Badge, broadcaster::Broadcaster};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Source {
    #[var]
    pub source_badges: Array<Gd<Badge>>,
    #[var]
    pub source_broadcaster: Option<Gd<Broadcaster>>,
    #[var]
    pub source_message_id: GString,
}

#[godot_api]
impl Source {
    #[func]
    pub fn create(
        source_badges: Array<Gd<Badge>>,
        source_broadcaster: Option<Gd<Broadcaster>>,
        source_message_id: GString,
    ) -> Gd<Self> {
        Gd::from_object(Self {
            source_badges,
            source_broadcaster,
            source_message_id,
        })
    }
}
