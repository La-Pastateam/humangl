mod box_mesh;
pub use box_mesh::BoxMesh;
mod load_model;
pub use load_model::{Vertex, load_model};

mod bone;
mod skeleton;
pub use skeleton::Skeleton;

mod human;
pub use human::Human;
