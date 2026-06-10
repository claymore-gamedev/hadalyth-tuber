use godot::prelude::*;

mod hadalyth_ndi;

mod custom_events;

pub struct HadalythNdiExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for HadalythNdiExtensionLibrary {}
