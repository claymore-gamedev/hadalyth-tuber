use godot::prelude::*;

use crate::custom_resources::user::User;


#[derive(GodotClass)]
#[class(no_init)]
pub struct Mention {
    #[var]
    user : Option<Gd<User>>,
}

impl Mention{
    pub fn create(
        user : Option<Gd<User>>
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                user
            }
        );
    }
}