use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Cheermote {
    #[var]
    prefix: GString,

    #[var]
    bits: i64,

    #[var]
    tier: i64,
}

#[godot_api]
impl Cheermote {
    #[func]
    pub fn create(prefix: GString, bits: i64, tier: i64) -> Gd<Self> {
        return Gd::from_object(Self { prefix, bits, tier });
    }
}
