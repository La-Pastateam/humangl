use crate::parser::obj::ObjModel;
use crate::utils::math::Matrix4;

#[derive(PartialEq)]
pub struct Bone {
    label: String,
    id: usize,
    pub parent_id: Option<usize>,
    transform: Matrix4,
    model: ObjModel,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn new(label: String, id: usize, model: ObjModel) -> Self {
        Bone {
            label,
            id,
            parent_id: None,
            transform: Matrix4::identity(),
            model: model,
        }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn id(&self) -> usize {
        self.id
    }
}
