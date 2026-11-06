use anyhow::{Context, Result};

use crate::model::bone::Bone;

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
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Skeleton {
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
