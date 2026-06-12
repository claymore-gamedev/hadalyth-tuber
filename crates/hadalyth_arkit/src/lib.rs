use godot::prelude::*;

mod hadalyth_arkit_mesh_driver;

mod hadalyth_arkit;
mod hadalyth_arkit_async;

mod custom_config;
mod custom_events;
mod custom_resources;

pub struct HadalythArkitExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for HadalythArkitExtensionLibrary {}
