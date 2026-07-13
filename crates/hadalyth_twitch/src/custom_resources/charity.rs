use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Charity {
    #[var]
    pub description: GString,

    #[var]
    pub logo: GString,

    #[var]
    pub name: GString,
    
    #[var]
    pub website : GString
}

#[godot_api]
impl Charity {
    #[func]
    pub fn create(description : GString, logo : GString, name : GString,  website : GString) -> Gd<Self> {
        return Gd::from_object(Self { description, logo, name, website });
    }
}
