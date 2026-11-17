use crate::model::Model;
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

    pub fn from_dimension(label: String, id: usize, depth: f32, width: f32, height: f32) -> Self {
        println!("test : {id}");
        let color = Vec3 {
            x: 0.0,
            y: id as f32,
            z: 0.5 + (id as f32) * 10.0,
            // x: ((id + 1) as f32 * 15.0) % 155.0,
            // y: ((id + 1) as f32 * 25.0) % 155.0,
            // z: ((id + 1) as f32 * 35.0) % 155.0,
        };
        let model = Model::new(depth, width, height, color);

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

    pub fn id(&self) -> usize {
        self.id
    }
}
