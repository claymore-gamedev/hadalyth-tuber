use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Timestamp {
    #[var]
    pub year: GString,
    #[var]
    pub month: GString,
    #[var]
    pub day: GString,
    #[var]
    pub hour: GString,
    #[var]
    pub minute: GString,
    #[var]
    pub second: GString,
}

#[godot_api]
impl Timestamp {
    #[func]
    pub fn create(
        year: GString,
        month: GString,
        day: GString,
        hour: GString,
        minute: GString,
        second: GString,
    ) -> Gd<Self> {
        Gd::from_object(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}
