use crate::utils::color::RGBA;
use crate::utils::math::{Vector3, vec3};

// ── BONEBUILDER ─────────────────────────────────────────────────────────────────

#[must_use]
#[derive(Default, Debug)]
pub struct BoneBuilder {
    label: String,
    id: usize,
    parent_id: Option<usize>,
    color: RGBA,
    dimension: Vector3,
    pivot: Vector3,
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl BoneBuilder {
    pub fn parent(mut self, id: usize) -> Self {
        self.parent_id = Some(id);
        self
    }

    pub fn color(mut self, color: RGBA) -> Self {
        self.color = color;
        self
    }

    pub fn dimension(mut self, dimension: Vector3) -> Self {
        self.dimension = dimension;
        self
    }

    pub fn pivot(mut self, pivot: Vector3) -> Self {
        self.pivot = pivot;
        self
    }

    pub fn build(self) -> Bone {
        Bone {
            label: self.label,
            id: self.id,
            parent_id: self.parent_id,
            color: self.color,
            dimension: self.dimension,
            pivot: self.pivot,
            ..Default::default()
        }
    }
}

// ── BONE ─────────────────────────────────────────────────────────────────

#[derive(PartialEq, Default, Debug, Clone)]
pub struct Bone {
    pub label: String,
    id: usize,
    pub parent_id: Option<usize>,
    pub color: RGBA,
    pub dimension: Vector3,
    pub rotation: Vector3,
    pub pivot: Vector3,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn builder(label: impl Into<String>, id: usize) -> BoneBuilder {
        BoneBuilder {
            label: label.into(),
            id,
            dimension: vec3(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn change_color(&mut self, color: RGBA) {
        self.color = color;
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get_color(&self) -> RGBA {
        self.color
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

}
