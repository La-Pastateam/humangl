use anyhow::{Context, Result};

use crate::model::Model;
use crate::model::bone::Bone;
use crate::utils::color::RGB;

#[derive(Debug, Clone)]
pub struct Skeleton {
    label: String,
    bones: Vec<Bone>,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Skeleton {
    pub const fn new(label: String) -> Self {
        Skeleton {
            label,
            bones: Vec::new(),
        }
    }

    pub const fn from_bones(label: String, bones: Vec<Bone>) -> Self {
        Skeleton { label, bones }
    }

    pub fn human(label: String) -> Self {
        let arms_color = RGB::new(100, 100, 100).to_vec();
        let legs_color = RGB::new(150, 150, 150).to_vec();

        let upperarm_model = Model::new(9.0, 28.0, 9.0, arms_color);
        let forearm_model = Model::new(7.5, 25.0, 7.5, arms_color);
        let thigh_model = Model::new(14.5, 42.0, 15.5, legs_color);
        let lowerleg_model = Model::new(10.5, 42.0, 11.0, legs_color);

        let mut bones: Vec<Bone> = vec![
            Bone::from_dimension(String::from("Torso"), 0, 34.0, 52.0, 21.0),
            Bone::from_dimension(String::from("Head"), 1, 16.0, 20.0, 18.5),
            Bone::new(String::from("RightUpperArm"), 2, upperarm_model.clone()),
            Bone::new(String::from("RightForeArm"), 3, forearm_model.clone()),
            Bone::new(String::from("LeftUpperArm"), 4, upperarm_model),
            Bone::new(String::from("LeftForeArm"), 5, forearm_model),
            Bone::new(String::from("RightThigh"), 6, thigh_model.clone()),
            Bone::new(String::from("RightLowerLeg"), 7, lowerleg_model.clone()),
            Bone::new(String::from("LeftThigh"), 8, thigh_model),
            Bone::new(String::from("LeftLowerLeg"), 9, lowerleg_model),
        ];

        bones[1].add_parent(0);
        bones[2].add_parent(0);
        bones[3].add_parent(2);
        bones[4].add_parent(0);
        bones[5].add_parent(4);
        bones[6].add_parent(0);
        bones[7].add_parent(6);
        bones[8].add_parent(0);
        bones[9].add_parent(8);

        Skeleton::from_bones(label, bones)
        // TODO: Créer les os avec les fonctions associées - HEAD, TORSO, 2ARMS, 2 LEGS.
        // + Ajouter les positions des bones.
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Skeleton {
    pub fn bones(&self) -> &[Bone] {
        &self.bones
    }

    pub fn add_bone(mut self, bone: Bone) -> Self {
        self.bones.push(bone);
        self
    }

    pub fn rename(mut self, new_label: String) -> Self {
        self.label = new_label;
        self
    }

    pub fn remove_bone<T: FindBone>(&mut self, target: &T) -> Result<()> {
        let index = target
            .find_bone(&self.bones)
            .context("Bone not found in skeleton")?;
        let removed = self.bones.remove(index);

        self.remove_children(removed.id())?;

        Ok(())
    }

    pub fn remove_children(&mut self, parent_id: usize) -> Result<()> {
        let children_ids: Vec<usize> = self.children(parent_id).map(Bone::id).collect();

        for id in children_ids {
            self.remove_bone(&id)?;
        }

        Ok(())
    }

    pub fn children(&self, parent_id: usize) -> impl Iterator<Item = &Bone> {
        self.bones
            .iter()
            .filter(move |&bone| bone.parent_id == Some(parent_id))
    }
}

// ── Trait Definitions ─────────────────────────────────────────────────────────────────

pub trait FindBone {
    fn find_bone(&self, bones: &[Bone]) -> Option<usize>;
}

// ── Trait Implementations ─────────────────────────────────────────────────────────────────

impl FindBone for &Bone {
    fn find_bone(&self, bones: &[Bone]) -> Option<usize> {
        bones.iter().position(|elem| *self == elem)
    }
}

impl FindBone for usize {
    fn find_bone(&self, bones: &[Bone]) -> Option<usize> {
        bones.iter().position(|elem| *self == elem.id())
    }
}
