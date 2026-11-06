use crate::parser::obj::ObjModel;
use crate::utils::math::Matrix4;

#[derive(PartialEq)]
pub struct Bone {
    label: String,
    parent_id: Option<usize>,
    transform: Matrix4,
    model: ObjModel,
}
