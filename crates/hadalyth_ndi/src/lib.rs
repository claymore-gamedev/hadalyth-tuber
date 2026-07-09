use godot::prelude::*;

mod hadalyth_ndi;
mod hadalyth_ndi_enums;

mod hadalyth_frame_resizer;

pub struct HadalythNdiExtensionLibrary;


#[gdextension]
unsafe impl ExtensionLibrary for HadalythNdiExtensionLibrary {}