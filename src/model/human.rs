use crate::model::Skeleton;
use crate::model::bone::Bone;
use crate::utils::color::Rgba;
use crate::utils::math::{Vector3, vec3};

// ── Constants ─────────────────────────────────────────────────────────────────

const HEAD_DIM: Vector3 = vec3(16.0, 20.0, 18.5);

const TORSO_DIM: Vector3 = vec3(34.0, 52.0, 21.0);

const UPPERARM_DIM: Vector3 = vec3(9.0, 28.0, 9.0);
const FOREARM_DIM: Vector3 = vec3(7.5, 25.0, 7.5);

const THIGH_DIM: Vector3 = vec3(14.5, 42.0, 15.5);
const LOWERLEG_DIM: Vector3 = vec3(10.5, 42.0, 11.0);
const LEG_LENGTH: f32 = THIGH_DIM.y + LOWERLEG_DIM.y;

// ── Side ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Side {
    const fn sign(self) -> f32 {
        match self {
            Side::Left => -1.0,
            Side::Right => 1.0,
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Side::Left => "Left",
            Side::Right => "Right",
        }
    }
}

// ── Human ─────────────────────────────────────────────────────────────────

impl Skeleton {
    pub fn human(label: String) -> Self {
        let head_color = Rgba::new(50, 150, 50, 255);
        let torso_color = Rgba::new(50, 250, 50, 255);
        let arms_color = Rgba::new(100, 100, 255, 255);
        let legs_color = Rgba::new(250, 150, 000, 255);

        let mut bones: Vec<Bone> = vec![
            Bone::builder("Torso", 0)
                .color(torso_color)
                .dimension(TORSO_DIM)
                .anchor(vec3(0.0, LEG_LENGTH, 0.0))
                .pivot(vec3(0.0, -0.5, 0.0))
                .build(),
            Bone::builder("Head", 1)
                .color(head_color)
                .parent(0)
                .dimension(HEAD_DIM)
                .anchor(vec3(0.0, 0.5, 0.0))
                .pivot(vec3(0.0, -0.5, 0.0))
                .build(),
        ];

        bones.extend(arm(Side::Left, 2, arms_color));
        bones.extend(arm(Side::Right, 4, arms_color));
        bones.extend(leg(Side::Left, 6, legs_color));
        bones.extend(leg(Side::Right, 8, legs_color));

        Skeleton::from_bones(label, bones)
    }
}

//TODO: Appliquer les anchor relatif à leur parent
fn arm(side: Side, first_id: usize, color: Rgba) -> [Bone; 2] {
    let s = side.sign();
    let name = side.name();

    [
        Bone::builder(format!("{name}UpperArm"), first_id)
            .parent(0)
            .color(color)
            .dimension(UPPERARM_DIM)
            .anchor(vec3(s * 0.5, 0.5, 0.0))
            .pivot(vec3(-s * 0.5, 0.5, 0.0))
            .build(),
        Bone::builder(format!("{name}ForeArm"), first_id + 1)
            .parent(first_id)
            .color(color)
            .dimension(FOREARM_DIM)
            .anchor(vec3(0.0, -0.5, 0.0))
            .pivot(vec3(0.0, 0.5, 0.0))
            .build(),
    ]
}
fn leg(side: Side, first_id: usize, color: Rgba) -> [Bone; 2] {
    let s = side.sign();
    let name = side.name();
    [
        Bone::builder(format!("{name}Thigh"), first_id)
            .parent(0)
            .color(color)
            .dimension(THIGH_DIM)
            .anchor(vec3(s * 0.25, -0.5, 0.0))
            .pivot(vec3(0.0, 0.5, 0.0))
            .build(),
        Bone::builder(format!("{name}LowerLeg"), first_id + 1)
            .parent(first_id)
            .color(color)
            .dimension(LOWERLEG_DIM)
            .anchor(vec3(0.0, -0.5, 0.0))
            .pivot(vec3(0.0, 0.5, 0.0))
            .build(),
    ]
}
