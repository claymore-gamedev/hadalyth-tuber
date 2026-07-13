
use crate::{custom_enums::automod_message_status::AutomodMessageStatus};

pub trait ToGodotAutomodMessageStatus {
    fn to_godot_automod_message_status(&self) -> AutomodMessageStatus;
}

impl ToGodotAutomodMessageStatus for twitch_api::eventsub::automod::message::AutomodMessageStatus {
    fn to_godot_automod_message_status(&self) -> AutomodMessageStatus {
        return match self {
            twitch_api::eventsub::automod::message::AutomodMessageStatus::Approved => {AutomodMessageStatus::Approved}
            twitch_api::eventsub::automod::message::AutomodMessageStatus::Denied => {AutomodMessageStatus::Denied}
            twitch_api::eventsub::automod::message::AutomodMessageStatus::Expired => {AutomodMessageStatus::Expired}
            twitch_api::eventsub::automod::message::AutomodMessageStatus::Invalid => {AutomodMessageStatus::Invalid}
            twitch_api::eventsub::automod::message::AutomodMessageStatus::Unknown(_) => {AutomodMessageStatus::Unknown}
            _ => {AutomodMessageStatus::Unknown}
        }
    }
}
