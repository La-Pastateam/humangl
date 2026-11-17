use crate::model::Model;
use crate::utils::color::RGB;
use crate::utils::math::{Matrix4, Vec3};

#[derive(PartialEq, Debug, Clone)]
pub struct Bone {
    label: String,
    id: usize,
    pub parent_id: Option<usize>,
    pub transform: Matrix4,
    pub model: Model,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn new(label: String, id: usize, model: Model) -> Self {
        Bone {
            label,
            id,
            parent_id: None,
            transform: Matrix4::identity(),
            model: model,
        }
    }

    pub fn from_dimension(label: String, id: usize, width: f32, height: f32, depth: f32) -> Self {
        // TODO: Revoir la gestion des couleurs, acutellement temporaire pour voir les différences.
        let color = RGB::new(
            (((id + 1) * 90) % 255) as u8,
            (((id + 1) * 10) % 255) as u8,
            (((id + 1) * 30) % 255) as u8,
        )
        .to_vec();
        let model = Model::new(width, height, depth, color);

        Bone {
            label,
            id,
            parent_id: None,
            transform: Matrix4::identity(),
            model,
        }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn change_color(&mut self, color: Vec3) -> &Self {
        self.model.change_color(color);
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
