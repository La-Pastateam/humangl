use crate::utils::color::RGBA;
use crate::utils::math::{Vector3, vec3};

#[derive(PartialEq, Debug, Clone)]
pub struct Bone {
    label: String,
    id: usize,
    pub parent_id: Option<usize>,
    pub color: RGBA,
    pub dimension: Vector3,
    pub rotation: Vector3,
    pub pivot: Vector3,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn new(
        label: impl Into<String>,
        id: usize,
        color: RGBA,
        dimension: Vector3,
        pivot: Vector3,
    ) -> Self {
        Bone {
            label,
            id,
            parent_id: None,
            color,
            dimension,
            rotation: vec3(0.0, 0.0, 0.0),
            pivot,
        }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn change_color(&mut self, color: RGBA) -> &Self {
        self.color = color;
        self
    }

    pub fn add_parent(&mut self, id: usize) -> &Self {
        self.parent_id = Some(id);
        self
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
