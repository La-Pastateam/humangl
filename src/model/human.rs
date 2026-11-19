use crate::model::Skeleton;
use crate::model::bone::Bone;
use crate::utils::color::RGBA;
use crate::utils::math::{Vector3, vec3};

// ── Constants ─────────────────────────────────────────────────────────────────

const HEAD_DIM: Vector3 = vec3(16.0, 20.0, 18.5);

const TORSO_DIM: Vector3 = vec3(34.0, 52.0, 21.0);

const UPPERARM_DIM: Vector3 = vec3(9.0, 28.0, 9.0);
const FOREARM_DIM: Vector3 = vec3(7.5, 25.0, 7.5);

const THIGH_DIM: Vector3 = vec3(14.5, 42.0, 15.5);
const LOWERLEG_DIM: Vector3 = vec3(10.5, 42.0, 11.0);
const LEG_LENGTH: f32 = THIGH_DIM.y + LOWERLEG_DIM.y;

// ── Human ─────────────────────────────────────────────────────────────────

impl Skeleton {
    pub fn human(label: String) -> Self {
        let head_color = RGBA::new(50, 150, 50, 255);
        let torso_color = RGBA::new(50, 250, 50, 255);
        let arms_color = RGBA::new(100, 100, 255, 255);
        let legs_color = RGBA::new(250, 150, 000, 255);

        //TODO : Il faut ajouter les pivot `.pivot(...)` a tous les Bones builder sauf le torse.
        //les anchors des membres sont temporaires
        let mut bones: Vec<Bone> = vec![
            Bone::builder("Torso", 0)
                .color(torso_color)
                .dimension(TORSO_DIM)
                .anchor(vec3(0.0, LEG_LENGTH, 0.0))
                .build(),
            Bone::builder("Head", 1)
                .color(head_color)
                .parent(0)
                .dimension(HEAD_DIM)
                .anchor(vec3(0.0, TORSO_DIM.y, 0.0))
                .build(),
        ];

        bones.extend(arm(
            "Left",
            2,
            vec3(-TORSO_DIM.x, TORSO_DIM.y, 0.0),
            arms_color,
        ));
        bones.extend(arm(
            "Right",
            4,
            vec3(TORSO_DIM.x, TORSO_DIM.y, 0.0),
            arms_color,
        ));
        bones.extend(leg("Left", 6, vec3(-TORSO_DIM.x, 0.0, 0.0), legs_color));
        bones.extend(leg("Right", 8, vec3(TORSO_DIM.x, 0.0, 0.0), legs_color));

        Skeleton::from_bones(label, bones)
    }
}

//TODO: Appliquer les anchor relatif à leur parent
fn arm(side: &str, first_id: usize, anchor: Vector3, color: RGBA) -> [Bone; 2] {
    [
        Bone::builder(format!("{side}UpperArm"), first_id)
            .parent(0)
            .color(color)
            .dimension(UPPERARM_DIM)
            .anchor(anchor)
            .build(),
        Bone::builder(format!("{side}ForeArm"), first_id + 1)
            .parent(first_id)
            .color(color)
            .dimension(FOREARM_DIM)
            .anchor(anchor)
            .build(),
    ]
}

fn leg(side: &str, first_id: usize, anchor: Vector3, color: RGBA) -> [Bone; 2] {
    [
        Bone::builder(format!("{side}Thigh"), first_id)
            .parent(0)
            .color(color)
            .dimension(THIGH_DIM)
            .anchor(anchor)
            .build(),
        Bone::builder(format!("{side}LowerLeg"), first_id + 1)
            .parent(first_id)
            .color(color)
            .dimension(LOWERLEG_DIM)
            .anchor(anchor)
            .build(),
    ]
}
