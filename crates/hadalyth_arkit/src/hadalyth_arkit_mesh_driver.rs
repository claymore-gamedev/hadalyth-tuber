use godot::classes::INode;
use godot::classes::Node;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct HadalythArkitMeshDriver {
    base: Base<Node>,
}

#[godot_api]
impl INode for HadalythArkitMeshDriver {
    fn init(base: Base<Node>) -> Self {
        return Self { base };
    }

    fn ready(&mut self) {}

    fn process(&mut self, _delta: f64) {
        // TODO : Update mesh blend shapes

        // TODO : If not new blend shapes are received for X amount of time, start blending towards rest instead of towards the
        // goal
    }
}

#[godot_api]
impl HadalythArkitMeshDriver {}
