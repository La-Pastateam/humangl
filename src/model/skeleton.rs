use crate::utils::math::Matrix4;
use anyhow::{Context, Result};

use crate::model::bone::Bone;

use crate::matrix_stack::MatrixStack;

#[derive(Debug, Clone)]
pub struct Skeleton {
    label: String,
    bones: Vec<Bone>,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Skeleton {
    pub fn new(label: impl Into<String>) -> Self {
        Skeleton {
            label: label.into(),
            bones: Vec::new(),
        }
    }

    pub fn from_bones(label: impl Into<String>, bones: Vec<Bone>) -> Self {
        Skeleton {
            label: label.into(),
            bones,
        }
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

    fn visit(
        &self,
        bone: &Bone,
        parent: Option<&Bone>,
        stack: &mut MatrixStack,
        result: &mut [Matrix4],
    ) {
        stack.push(bone.transform(parent));
        result[bone.id()] = *stack.top();

        self.children(bone.id())
            .for_each(|child| self.visit(child, Some(bone), stack, result));
        stack.pop();
    }

    pub fn world_matrices(&self) -> Vec<Matrix4> {
        let mut stack = MatrixStack::new();
        let mut result: Vec<Matrix4> = vec![Matrix4::identity(); self.bones.len()];

        for bone in self.bones() {
            if bone.parent_id.is_none() {
                self.visit(bone, None, &mut stack, &mut result);
            }
        }

        result
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
