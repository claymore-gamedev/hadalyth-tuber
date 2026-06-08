use godot::prelude::*;



#[derive(GodotClass)]
#[class(no_init)]
pub struct Broadcaster {
    #[var]
    user_id : GString,
    
    #[var]
    user_login : GString,
    
    #[var]
    user_name : GString
}

#[godot_api]
impl Broadcaster {
    #[func]
    pub fn create(
        user_id : GString,
        user_login : GString,
        user_name : GString,
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                user_id,
                user_login,
                user_name
            }
        )
    }
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct User {
    #[var]
    user_id : GString,
    
    #[var]
    user_login : GString,
    
    #[var]
    user_name : GString
}

#[godot_api]
impl User {
    #[func]
    pub fn create(
        user_id : GString,
        user_login : GString,
        user_name : GString,
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                user_id,
                user_login,
                user_name
            }
        )
    }
}


#[derive(GodotClass)]
#[class(no_init)]
pub struct Emote {
    #[var]
    id : GString,

    #[var]
    emote_set_id : GString,

    #[var]
    owner_id : GString,

    #[var]
    format : GString,
}

#[godot_api]
impl Emote {
    #[func]
    pub fn create(
        id : GString,
        emote_set_id : GString,
        owner_id : GString,
        format : GString
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                id,
                emote_set_id,
                owner_id,
                format
            }
        )
    }
}


#[derive(GodotClass)]
#[class(no_init)]
pub struct Fragment {
    #[var]
    text : GString,

    #[var]
    emote : Option<Gd<Emote>>,
}


#[godot_api]
impl Fragment {
    #[func]
    pub fn create(
        text : GString,
        emote : Option<Gd<Emote>>
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                text,
                emote
            }
        );
    }
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct Message {
    #[var]
    text : GString,

    #[var]
    fragments : Array<Gd<Fragment>>
}

#[godot_api]
impl Message {
    #[func]
    pub fn create(
        text : GString,
        fragments : Array<Gd<Fragment>>
    ) -> Gd<Self> {
        return Gd::from_object(
            Self {
                text, 
                fragments
            }
        )
    }
}

// ChannelChatMessageV1Payload { 
//     broadcaster_user_id: "1361712337", 
//     broadcaster_user_name: "CLAYMORE_DEV", 
//     broadcaster_user_login: "claymore_dev", 
//     chatter_user_id: "1361712337", 
//     chatter_user_name: "CLAYMORE_DEV", 
//     chatter_user_login: "claymore_dev", 
//     message_id: "4d9a4a6c-12bc-4d18-b5b6-105b17e16126", 
//     message: Message { 
//         text: "asdf SeemsGood Kappa asdf asdf asdf", 
//         fragments: [
//             Text { 
//                 text: "asdf " 
//             }, 
//             Emote { 
//                 text: "SeemsGood", 
//                 emote: Emote { id: "64138", emote_set_id: "0", owner_id: "0", format: [Static] } 
//             }, 
//             Text { 
//                 text: " " 
//             }, 
//             Emote { 
//                 text: "Kappa", 
//                 emote: Emote { id: "25", emote_set_id: "0", owner_id: "0", format: [Static] } 
//             }, 
//             Text { 
//                 text: " asdf asdf asdf" 
//             }
//         ] 
//     }, 
//     message_type: Text, 
//     badges: [Badge { set_id: "broadcaster", id: "1", info: "" }, Badge { set_id: "subscriber", id: "3006", info: "8" }, Badge { set_id: "premium", id: "1", info: "" }], cheer: None, color: "#000000", reply: None, channel_points_custom_reward_id: None, channel_points_animation_id: None, source_broadcaster_user_id: None, source_broadcaster_user_name: None, source_broadcaster_user_login: None, source_message_id: None, source_badges: [], is_source_only: None }
