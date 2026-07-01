use godot::classes::INode;
use godot::classes::MeshInstance3D;
use godot::classes::Node;
use godot::prelude::*;

use crate::custom_resources::live_link_face_blend_shapes::LiveLinkFaceBlendShapes;

const BLEND_SHAPE_NAMES: [&str; 52] = [
    "eyeBlinkLeft",
    "eyeLookDownLeft",
    "eyeLookInLeft",
    "eyeLookOutLeft",
    "eyeLookUpLeft",
    "eyeSquintLeft",
    "eyeWideLeft",
    "eyeBlinkRight",
    "eyeLookDownRight",
    "eyeLookInRight",
    "eyeLookOutRight",
    "eyeLookUpRight",
    "eyeSquintRight",
    "eyeWideRight",
    "jawForward",
    "jawLeft",
    "jawRight",
    "jawOpen",
    "mouthClose",
    "mouthFunnel",
    "mouthPucker",
    "mouthLeft",
    "mouthRight",
    "mouthSmileLeft",
    "mouthSmileRight",
    "mouthFrownLeft",
    "mouthFrownRight",
    "mouthDimpleLeft",
    "mouthDimpleRight",
    "mouthStretchLeft",
    "mouthStretchRight",
    "mouthRollLower",
    "mouthRollUpper",
    "mouthShrugLower",
    "mouthShrugUpper",
    "mouthPressLeft",
    "mouthPressRight",
    "mouthLowerDownLeft",
    "mouthLowerDownRight",
    "mouthUpperUpLeft",
    "mouthUpperUpRight",
    "browDownLeft",
    "browDownRight",
    "browInnerUp",
    "browOuterUpLeft",
    "browOuterUpRight",
    "cheekPuff",
    "cheekSquintLeft",
    "cheekSquintRight",
    "noseSneerLeft",
    "noseSneerRight",
    "tongueOut",
];
// "headYaw",
// "headPitch",
// "headRoll",
// "leftEyeYaw",
// "leftEyePitch",
// "leftEyeRoll",
// "rightEyeYaw",
// "rightEyePitch",
// "rightEyeRoll",

#[derive(GodotClass)]
#[class(base=Node)]
struct HadalythArkitMeshDriver {
    #[export]
    target: Option<Gd<MeshInstance3D>>,

    #[export]
    blend_shape_lerp_speed: f32,

    #[export]
    blend_shape_jaw_open_mult: f32,

    #[export]
    blend_shape_jaw_open_root : f32,

    blend_shape_indexes: Vec<i32>,
    blend_shape_index_jaw_open: i32,

    #[var]
    target_blend_shapes: Gd<LiveLinkFaceBlendShapes>,

    #[var]
    current_blend_shapes: Gd<LiveLinkFaceBlendShapes>,

    base: Base<Node>,
}

#[godot_api]
impl INode for HadalythArkitMeshDriver {
    fn init(base: Base<Node>) -> Self {
        let target = None;

        let blend_shape_lerp_speed = 10.0;
        let blend_shape_jaw_open_mult = 2.0;
        let blend_shape_jaw_open_root = 0.5;

        let blend_shape_indexes = vec![];
        let blend_shape_index_jaw_open = -1;

        let target_blend_shapes = LiveLinkFaceBlendShapes::new();
        let current_blend_shapes = LiveLinkFaceBlendShapes::new();

        return Self {
            target,

            blend_shape_lerp_speed,
            blend_shape_jaw_open_mult,
            blend_shape_jaw_open_root,

            blend_shape_indexes,
            blend_shape_index_jaw_open,

            target_blend_shapes,
            current_blend_shapes,

            base,
        };
    }

    fn ready(&mut self) {
        let Some(ref mut target) = self.target else {
            godot_error!("Mesh not set");
            return;
        };

        for blend_shape_name in BLEND_SHAPE_NAMES {
            let blend_shape_index = target.find_blend_shape_by_name(blend_shape_name);
            if blend_shape_index == -1 {
                godot_print!(
                    "{} does not have blend shape {}",
                    target.get_name(),
                    blend_shape_name
                );
            }
            self.blend_shape_indexes.push(blend_shape_index);
        }
        self.blend_shape_index_jaw_open = target.find_blend_shape_by_name(BLEND_SHAPE_NAMES[17]);
    }

    fn process(&mut self, delta: f32) {
        // Make sure these are the same
        if self.current_blend_shapes.bind().blend_shapes.len()
            != self.target_blend_shapes.bind().blend_shapes.len()
        {
            return;
        }
        if self.current_blend_shapes.bind().blend_shapes.len() != self.blend_shape_indexes.len() {
            return;
        }

        // Make sure the mesh is valid
        let Some(ref mut target) = self.target else {
            return;
        };

        // Lerp towards the target blend shapes
        let len = self.current_blend_shapes.bind().blend_shapes.len();
        for i in 0..len {
            let cur_val = self.current_blend_shapes.bind().blend_shapes[i];
            let tar_val = self.target_blend_shapes.bind().blend_shapes[i];

            self.current_blend_shapes.bind_mut().blend_shapes[i] =
                cur_val + (tar_val - cur_val) * delta * self.blend_shape_lerp_speed;

            let mut nex_val = self.current_blend_shapes.bind_mut().blend_shapes[i];

            let blend_shape_index = self.blend_shape_indexes[i];
            if blend_shape_index == -1 {
                continue;
            }

            if blend_shape_index == self.blend_shape_index_jaw_open {
                nex_val = nex_val * self.blend_shape_jaw_open_mult;
                nex_val = f32::powf(nex_val, self.blend_shape_jaw_open_root);
            }

            target.set_blend_shape_value(
                blend_shape_index,
                nex_val,
            );
        }

        let head_yaw = self.current_blend_shapes.bind_mut().head_yaw;
        self.current_blend_shapes.bind_mut().head_yaw = head_yaw.lerp_angle(
            self.target_blend_shapes.bind().head_yaw,
            delta * self.blend_shape_lerp_speed,
        );

        let head_pitch = self.current_blend_shapes.bind_mut().head_pitch;
        self.current_blend_shapes.bind_mut().head_pitch = head_pitch.lerp_angle(
            self.target_blend_shapes.bind().head_pitch,
            delta * self.blend_shape_lerp_speed,
        );

        let head_roll = self.current_blend_shapes.bind_mut().head_roll;
        self.current_blend_shapes.bind_mut().head_roll = head_roll.lerp_angle(
            self.target_blend_shapes.bind().head_roll,
            delta * self.blend_shape_lerp_speed,
        );

        let left_eye_yaw = self.current_blend_shapes.bind_mut().left_eye_yaw;
        self.current_blend_shapes.bind_mut().left_eye_yaw = left_eye_yaw.lerp_angle(
            self.target_blend_shapes.bind().left_eye_yaw,
            delta * self.blend_shape_lerp_speed,
        );

        let left_eye_pitch = self.current_blend_shapes.bind_mut().left_eye_pitch;
        self.current_blend_shapes.bind_mut().left_eye_pitch = left_eye_pitch.lerp_angle(
            self.target_blend_shapes.bind().left_eye_pitch,
            delta * self.blend_shape_lerp_speed,
        );

        let left_eye_roll = self.current_blend_shapes.bind_mut().left_eye_roll;
        self.current_blend_shapes.bind_mut().left_eye_roll = left_eye_roll.lerp_angle(
            self.target_blend_shapes.bind().left_eye_roll,
            delta * self.blend_shape_lerp_speed,
        );

        let right_eye_yaw = self.current_blend_shapes.bind_mut().right_eye_yaw;
        self.current_blend_shapes.bind_mut().right_eye_yaw = right_eye_yaw.lerp_angle(
            self.target_blend_shapes.bind().right_eye_yaw,
            delta * self.blend_shape_lerp_speed,
        );

        let right_eye_pitch = self.current_blend_shapes.bind_mut().right_eye_pitch;
        self.current_blend_shapes.bind_mut().right_eye_pitch = right_eye_pitch.lerp_angle(
            self.target_blend_shapes.bind().right_eye_pitch,
            delta * self.blend_shape_lerp_speed,
        );

        let right_eye_roll = self.current_blend_shapes.bind_mut().right_eye_roll;
        self.current_blend_shapes.bind_mut().right_eye_roll = right_eye_roll.lerp_angle(
            self.target_blend_shapes.bind().right_eye_roll,
            delta * self.blend_shape_lerp_speed,
        );
    }
}

#[godot_api]
impl HadalythArkitMeshDriver {
    #[func]
    fn update_target_blend_shapes(&mut self, target_blend_shapes: Gd<LiveLinkFaceBlendShapes>) {
        self.target_blend_shapes = target_blend_shapes;
    }

    #[func]
    fn reset_target_blend_shapes(&mut self) {
        self.target_blend_shapes = LiveLinkFaceBlendShapes::new();
    }
}
