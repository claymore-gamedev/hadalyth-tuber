use godot::prelude::*;

use twitch_api::eventsub::automod::message::AutomodMessage;

use crate::custom_resources::cheermote::Cheermote;
use crate::custom_resources::emote::Emote;
use crate::custom_resources::fragment::Fragment;
use crate::custom_resources::mention::Mention;
use crate::custom_resources::message::Message;
use crate::custom_resources::user::User;

pub trait ToGodotMessage {
    fn to_godot_message(&self) -> Gd<Message>;
}

impl ToGodotMessage for twitch_api::common::chat::Message {
    fn to_godot_message(&self) -> Gd<Message> {
        let message_text = self.text.to_godot();
        let mut message_fragments: Array<Gd<Fragment>> = array![];
        for fragment in self.fragments.as_slice() {
            let fragment = match fragment {
                twitch_api::common::chat::Fragment::Cheermote { text, cheermote } => {
                    let cheermote = Cheermote::create(
                        cheermote.prefix.to_godot(),
                        cheermote.bits as i64,
                        cheermote.tier as i64
                    );
                    Fragment::create(
                        text.to_godot(),
                        Some(cheermote),
                        None,
                        None
                    )
                }
                twitch_api::common::chat::Fragment::Emote {text, emote} => {

                    let format = emote.format.iter().map(|x|x.to_string().to_godot()).collect();

                    let emote = Emote::create(
                        emote.id.to_string().to_godot(),
                        emote.emote_set_id.to_string().to_godot(),
                        emote.owner_id.to_string().to_godot(),
                        format
                    );
                    Fragment::create(
                        text.to_godot(),
                        None,
                        Some(emote),
                        None
                    )
                }
                twitch_api::common::chat::Fragment::Mention {text, mention} => {
                    
                    let user = User::create(
                        mention.user_id.to_string().to_godot(), 
                        mention.user_login.to_string().to_godot(), 
                        mention.user_name.to_string().to_godot()
                    );
                    let mention = Mention::create(
                        Some(user)
                    );
                    Fragment::create(
                        text.to_godot(),
                        None,
                        None,
                        Some(mention)
                    )
                }
                twitch_api::common::chat::Fragment::Text{ text } => {
                    Fragment::create(
                        text.to_godot(),
                        None,
                        None,
                        None
                    )
                }
                _ => {
                    // Note : This should be unreachable
                    godot_error!("Invalid message fragment?");
                    godot_error!("{:?}", fragment);
                    continue;
                }
            };
            message_fragments.push(&fragment);
        }
        let message = Message::create(message_text, message_fragments);
        return message;
    }
}

impl ToGodotMessage for AutomodMessage {
    fn to_godot_message(&self) -> Gd<Message> {
        let message_text = self.text.to_godot();
        let mut message_fragments: Array<Gd<Fragment>> = array![];
        for fragment in self.fragments.as_slice() {
            let fragment = match fragment {
                twitch_api::eventsub::automod::message::AutomodMessageFragment::Cheermote { text, cheermote } => {
                    let cheermote = Cheermote::create(
                        cheermote.prefix.to_godot(),
                        cheermote.bits as i64,
                        cheermote.tier as i64
                    );
                    Fragment::create(
                        text.to_godot(),
                        Some(cheermote),
                        None,
                        None
                    )
                }
                twitch_api::eventsub::automod::message::AutomodMessageFragment::Emote {text, emote} => {

                    let emote = Emote::create(
                        emote.id.to_string().to_godot(),
                        emote.emote_set_id.to_string().to_godot(),
                        "".to_godot(),
                        array![]
                    );
                    Fragment::create(
                        text.to_godot(),
                        None,
                        Some(emote),
                        None
                    )
                }
                twitch_api::eventsub::automod::message::AutomodMessageFragment::Text{ text } => {
                    Fragment::create(
                        text.to_godot(),
                        None,
                        None,
                        None
                    )
                }
                _ => {
                    // Note : This should be unreachable
                    godot_error!("Invalid message fragment?");
                    godot_error!("{:?}", fragment);
                    continue;
                }
            };
            message_fragments.push(&fragment);
        }
        let message = Message::create(message_text, message_fragments);
        return message;
    }  
}