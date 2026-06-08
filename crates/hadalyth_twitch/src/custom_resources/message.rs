use godot::prelude::*;

use crate::custom_resources::fragment::Fragment;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Message {
    #[var]
    text: GString,

    #[var]
    fragments: Array<Gd<Fragment>>,
}

#[godot_api]
impl Message {
    #[func]
    pub fn create(text: GString, fragments: Array<Gd<Fragment>>) -> Gd<Self> {
        return Gd::from_object(Self { text, fragments });
    }
}
