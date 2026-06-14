use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct LiveLinkFaceBlendShapes {
    #[var]
    pub blend_shapes: PackedArray<f32>,

    #[var]
    pub head_yaw: f32,

    #[var]
    pub head_pitch: f32,

    #[var]
    pub head_roll: f32,

    #[var]
    pub left_eye_yaw: f32,

    #[var]
    pub left_eye_pitch: f32,

    #[var]
    pub left_eye_roll: f32,

    #[var]
    pub right_eye_yaw: f32,

    #[var]
    pub right_eye_pitch: f32,

    #[var]
    pub right_eye_roll: f32,
}

impl LiveLinkFaceBlendShapes {
    pub fn create(
        blend_shapes: PackedArray<f32>,
        head_yaw: f32,
        head_pitch: f32,
        head_roll: f32,
        left_eye_yaw: f32,
        left_eye_pitch: f32,
        left_eye_roll: f32,
        right_eye_yaw: f32,
        right_eye_pitch: f32,
        right_eye_roll: f32,
    ) -> Gd<Self> {
        return Gd::from_object(Self {
            blend_shapes,
            head_yaw,
            head_pitch,
            head_roll,
            left_eye_yaw,
            left_eye_pitch,
            left_eye_roll,
            right_eye_yaw,
            right_eye_pitch,
            right_eye_roll,
        });
    }
    pub fn new() -> Gd<Self> {
        let blend_shapes = PackedArray::from(vec![0.0_f32; 52]);

        let head_yaw: f32 = 0.0;
        let head_pitch: f32 = 0.0;
        let head_roll: f32 = 0.0;
        let left_eye_yaw: f32 = 0.0;
        let left_eye_pitch: f32 = 0.0;
        let left_eye_roll: f32 = 0.0;
        let right_eye_yaw: f32 = 0.0;
        let right_eye_pitch: f32 = 0.0;
        let right_eye_roll: f32 = 0.0;

        Gd::from_object(Self {
            blend_shapes,
            head_yaw,
            head_pitch,
            head_roll,
            left_eye_yaw,
            left_eye_pitch,
            left_eye_roll,
            right_eye_yaw,
            right_eye_pitch,
            right_eye_roll,
        })
    }
}
