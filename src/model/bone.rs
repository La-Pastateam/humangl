use crate::utils::color::RGB;
use crate::utils::math::{Matrix4, Vector3};

#[derive(PartialEq, Debug, Clone)]
pub struct Bone {
    label: String,
    id: usize,
    pub parent_id: Option<usize>,
    pub transform: Matrix4,
    pub color: RGB,
    pub dimension: Vector3,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn new(label: String, id: usize, color: RGB, dimension: Vector3) -> Self {
        Bone {
            label,
            id,
            parent_id: None,
            transform: Matrix4::identity(),
            color,
            dimension,
        }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn change_color(&mut self, color: RGB) -> &Self {
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
