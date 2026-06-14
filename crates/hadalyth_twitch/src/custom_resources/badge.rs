use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Badge {
    #[var]
    set_id: GString,

    #[var]
    id: GString,

    #[var]
    info: GString,
}

#[godot_api]
impl Badge {
    #[func]
    pub fn create(set_id: GString, id: GString, info: GString) -> Gd<Self> {
        Gd::from_object(Self { set_id, id, info })
    }
}
