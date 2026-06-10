use godot::prelude::*;

mod hadalyth_ndi;
mod hadalyth_ndi_enums;

pub struct HadalythNdiExtensionLibrary;


#[gdextension]
unsafe impl ExtensionLibrary for HadalythNdiExtensionLibrary {}