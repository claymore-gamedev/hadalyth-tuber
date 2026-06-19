use godot::prelude::*;

mod hadalyth_networking;

pub struct HadalythNetworkingExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for HadalythNetworkingExtensionLibrary {}
