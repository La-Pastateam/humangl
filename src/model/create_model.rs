use crate::parser::obj::{FaceVertex, ObjModel, triangulate_face};
use crate::utils::math::{Vec2, Vec3, vec2, vec3};

struct QuadFacePos {
    normal: usize,
    vertices: [usize; 4],
}

pub struct Model {
    pub obj_model: ObjModel,
    pub color: Vec3,
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

// ── Constants ─────────────────────────────────────────────────────────────────

const CUBE_FACES: [QuadFacePos; 6] = [
    QuadFacePos {
        normal: 1,
        vertices: [0, 1, 3, 2],
    },
    QuadFacePos {
        normal: 3,
        vertices: [3, 7, 6, 2],
    },
    QuadFacePos {
        normal: 0,
        vertices: [5, 4, 6, 7],
    },
    QuadFacePos {
        normal: 2,
        vertices: [0, 4, 5, 1],
    },
    QuadFacePos {
        normal: 5,
        vertices: [4, 0, 2, 6],
    },
    QuadFacePos {
        normal: 4,
        vertices: [5, 7, 3, 1],
    },
];

// ── Model Creation ─────────────────────────────────────────────────────────────────

fn create_vertices(length: f32, width: f32, height: f32) -> Vec<[f32; 3]> {
    vec![
        [0.0, 0.0, 0.0],
        [0.0, 0.0, height],
        [0.0, width, 0.0],
        [0.0, width, height],
        [length, 0.0, 0.0],
        [length, 0.0, height],
        [length, width, 0.0],
        [length, width, height],
    ]
}

fn create_normals() -> Vec<[f32; 3]> {
    vec![
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
    ]
}

fn create_faces() -> Vec<[FaceVertex; 3]> {
    CUBE_FACES
        .iter()
        .flat_map(|face| {
            let normal = Some(face.normal);
            [
                [
                    FaceVertex::new(face.vertices[0], normal),
                    FaceVertex::new(face.vertices[1], normal),
                    FaceVertex::new(face.vertices[2], normal),
                ],
                [
                    FaceVertex::new(face.vertices[0], normal),
                    FaceVertex::new(face.vertices[2], normal),
                    FaceVertex::new(face.vertices[3], normal),
                ],
            ]
        })
        .collect::<Vec<_>>()
}

impl Model {
    pub fn new(length: f32, width: f32, height: f32, color: Vec3) -> Self {
        let positions: Vec<[f32; 3]> = create_vertices(length, width, height);
        let normals = create_normals();
        let tex_coords: Vec<[f32; 2]> = Vec::new();
        let faces = create_faces();

        let obj_model = ObjModel {
            positions,
            tex_coords,
            normals,
            faces,
        };

        Self {
            obj_model,
            color,
            length,
            width,
            height,
        }
    }
}
