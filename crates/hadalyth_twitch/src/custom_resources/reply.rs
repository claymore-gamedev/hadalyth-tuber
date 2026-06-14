use crate::custom_resources::user::User;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Reply {
    #[var]
    pub parent_message_id: GString,
    #[var]
    pub parent_message_body: GString,
    #[var]
    pub parent_user: Gd<User>,
    #[var]
    pub thread_message_id: GString,
    #[var]
    pub thread_user: Gd<User>,
}

#[godot_api]
impl Reply {
    #[func]
    pub fn create(
        parent_message_id: GString,
        parent_message_body: GString,
        parent_user_id: GString,
        parent_user_name: GString,
        parent_user_login: GString,
        thread_message_id: GString,
        thread_user_id: GString,
        thread_user_name: GString,
        thread_user_login: GString,
    ) -> Gd<Self> {
        let parent_user = User::create(parent_user_id, parent_user_login, parent_user_name);

        let thread_user = User::create(thread_user_id, thread_user_login, thread_user_name);

        Gd::from_object(Self {
            parent_message_id,
            parent_message_body,
            parent_user,
            thread_message_id,
            thread_user,
        })
    }
}
