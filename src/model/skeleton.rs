use anyhow::{Context, Result};

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
        todo!();
        let arms_color = RGB::new(100, 100, 100).to_vec();
        let legs_color = RGB::new(150, 150, 150).to_vec();

        let mut bones: Vec<Bone> = vec![];

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
