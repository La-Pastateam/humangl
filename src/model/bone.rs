use crate::parser::obj::ObjModel;
use crate::utils::math::Matrix4;

pub struct Bone {
    label: String,
    parent_id: usize,
    transform: Matrix4,
    model: ObjModel,
}
