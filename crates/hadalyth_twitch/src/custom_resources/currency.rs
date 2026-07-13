use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Currency {
    #[var]
    pub value: i32,

    #[var]
    pub decimal_places: i32,
    
    #[var]
    pub currency: GString,
}

#[godot_api]
impl Currency {
    #[func]
    pub fn create(value: i32, decimal_places: i32, currency: GString) -> Gd<Self> {
        return Gd::from_object(Self { value, decimal_places, currency });
    }
}
