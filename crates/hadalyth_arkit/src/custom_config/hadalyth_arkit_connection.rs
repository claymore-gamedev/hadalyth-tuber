use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HadalythArkitConnection {
    base: Base<Resource>,

    #[export]
    pub port: i64,
}
