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

// ── Methods ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn id(&self) -> usize {
        self.id
    }
}
