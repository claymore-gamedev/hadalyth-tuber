use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Chatter {
    #[var]
    user_id: GString,

    #[var]
    user_login: GString,

    #[var]
    user_name: GString,
}

#[godot_api]
impl Chatter {
    #[func]
    pub fn create(user_id: GString, user_login: GString, user_name: GString) -> Gd<Self> {
        return Gd::from_object(Self {
            user_id,
            user_login,
            user_name,
        });
    }
}
