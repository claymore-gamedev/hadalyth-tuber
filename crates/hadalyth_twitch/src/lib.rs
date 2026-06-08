use godot::prelude::*;

struct HadalythTwitchExtensionLibrary;

mod resources;

mod hadalyth_twitch_resources;
mod hadalyth_twitch_enums;
mod hadalyth_twitch_async;
mod hadalyth_twitch_parse_event;
mod hadalyth_twitch;

#[gdextension]
unsafe impl ExtensionLibrary for HadalythTwitchExtensionLibrary {}