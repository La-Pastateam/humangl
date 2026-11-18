use crate::utils::color::RGB;
use crate::utils::math::Matrix4;

#[derive(PartialEq, Debug, Clone)]
pub struct Bone {
    label: String,
    id: usize,
    pub parent_id: Option<usize>,
    pub transform: Matrix4,
    pub color: RGB,
    pub dimension: (f32, f32, f32),
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn new(label: String, id: usize, color: RGB, dimension: (f32, f32, f32)) -> Self {
        Bone {
            label,
            id,
            parent_id: None,
            transform: Matrix4::identity(),
            color,
            dimension,
        }
    }

    pub fn from_dimension(label: String, id: usize, width: f32, height: f32, depth: f32) -> Self {
        // TODO: Revoir la gestion des couleurs, acutellement temporaire pour voir les différences.
        let color = RGB::new(
            (((id + 1) * 90) % 255) as u8,
            (((id + 1) * 10) % 255) as u8,
            (((id + 1) * 30) % 255) as u8,
        );
        let dimension = (width, height, depth);

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
