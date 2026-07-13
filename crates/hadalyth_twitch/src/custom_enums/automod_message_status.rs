use::godot::prelude::*;

#[derive(GodotConvert, Var, Export, Default, Clone, Debug)]
#[godot(via = GString)]
pub enum AutomodMessageStatus {
    #[default]
    Approved,
    Denied,
    Expired,
    Invalid,
    Unknown,
}