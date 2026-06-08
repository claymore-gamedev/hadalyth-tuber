use godot::prelude::*;

struct HadalythTwitchExtensionLibrary;

mod custom_config;
mod custom_events;
mod custom_resources;

mod hadalyth_twitch;
mod hadalyth_twitch_async;
mod hadalyth_twitch_parse_event;

#[gdextension]
unsafe impl ExtensionLibrary for HadalythTwitchExtensionLibrary {}
