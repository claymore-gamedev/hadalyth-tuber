use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Emote {
    #[var]
    id: GString,

    #[var]
    emote_set_id: GString,

    #[var]
    owner_id: GString,

    #[var]
    format: Array<GString>,
}

#[godot_api]
impl Emote {
    #[func]
    pub fn create(
        id: GString,
        emote_set_id: GString,
        owner_id: GString,
        format: Array<GString>,
    ) -> Gd<Self> {
        return Gd::from_object(Self {
            id,
            emote_set_id,
            owner_id,
            format,
        });
    }
}
