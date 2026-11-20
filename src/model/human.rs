use crate::model::Skeleton;
use crate::model::bone::Bone;
use crate::utils::color::RGBA;
use crate::utils::math::{Vector3, vec3};

impl Skeleton {
    pub fn human(label: String) -> Self {
        let head_color = RGBA::new(50, 150, 50, 255);
        let torso_color = RGBA::new(50, 250, 50, 255);
        let arms_color = RGBA::new(100, 100, 100, 255);
        let legs_color = RGBA::new(150, 150, 150, 255);

        //TODO : Il faut ajouter les pivot `.pivot(...)` a tous les Bones builder sauf le torse.
        //les pivots des membres sont temporaires
        let mut bones: Vec<Bone> = vec![
            Bone::builder("Torso", 0)
                .color(torso_color)
                .dimension(vec3(34.0, 52.0, 21.0))
                .build(),
            Bone::builder("Head", 1)
                .color(head_color)
                .parent(0)
                .dimension(vec3(16.0, 20.0, 18.5))
                .build(),
        ];

        bones.extend(arm("Left", 2, vec3(0.0, 0.0, 0.0), arms_color));
        bones.extend(arm("Right", 4, vec3(0.0, 0.0, 0.0), arms_color));
        bones.extend(leg("Left", 6, vec3(0.0, 0.0, 0.0), arms_color));
        bones.extend(leg("Right", 8, vec3(0.0, 0.0, 0.0), arms_color));

        Skeleton::from_bones(label, bones)
    }
}

//TODO: Appliquer les pivot relatif à leur parent
fn arm(side: &str, first_id: usize, pivot: Vector3, color: RGBA) -> [Bone; 2] {
    [
        Bone::builder(format!("{side}UpperArm"), first_id)
            .parent(0)
            .color(color)
            .dimension(vec3(9.0, 28.0, 9.0))
            .pivot(pivot)
            .build(),
        Bone::builder(format!("{side}ForeArm"), first_id + 1)
            .parent(first_id)
            .color(color)
            .dimension(vec3(7.5, 25.0, 7.5))
            .pivot(pivot)
            .build(),
    ]
}

fn leg(side: &str, first_id: usize, pivot: Vector3, color: RGBA) -> [Bone; 2] {
    [
        Bone::builder(format!("{side}Thigh"), first_id)
            .parent(0)
            .color(color)
            .dimension(vec3(14.5, 42.0, 15.5))
            .pivot(pivot)
            .build(),
        Bone::builder(format!("{side}LowerLeg"), first_id + 1)
            .parent(first_id)
            .color(color)
            .dimension(vec3(10.5, 42.0, 11.0))
            .pivot(pivot)
            .build(),
    ]
}
