use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct BitsCustomPowerUp {
    #[var]
    title: GString,

    #[var]
    reward_id: GString,
}

#[godot_api]
impl BitsCustomPowerUp {
    #[func]
    pub fn create(title: GString, reward_id: GString) -> Gd<Self> {
        return Gd::from_object(Self { title, reward_id });
    }
}
