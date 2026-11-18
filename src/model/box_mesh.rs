use crate::parser::obj::{FaceVertex, ObjModel};

struct QuadFacePos {
    normal: usize,
    vertices: [usize; 4],
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct BoxMesh {
    model: ObjModel,
}

// ── Constants ─────────────────────────────────────────────────────────────────

const BOXMESH_FACES: [QuadFacePos; 6] = [
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

const BOXMESH_VERTICES: [[f32; 3]; 8] = [
    [-0.5, -0.5, -0.5],
    [-0.5, -0.5, 0.5],
    [-0.5, 0.5, -0.5],
    [-0.5, 0.5, 0.5],
    [0.5, -0.5, -0.5],
    [0.5, -0.5, 0.5],
    [0.5, 0.5, -0.5],
    [0.5, 0.5, 0.5],
];

const BOXMESH_NORMALS: [[f32; 3]; 6] = [
    [1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, -1.0],
];

// ── BoxMesh Creation ─────────────────────────────────────────────────────────────────

fn create_faces() -> Vec<[FaceVertex; 3]> {
    BOXMESH_FACES
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
        .collect()
}

impl BoxMesh {
    pub fn new() -> Self {
        let positions: Vec<[f32; 3]> = BOXMESH_VERTICES.to_vec();
        let normals = BOXMESH_NORMALS.to_vec();
        let tex_coords: Vec<[f32; 2]> = Vec::new();
        let faces = create_faces();

        let model = ObjModel {
            positions,
            tex_coords,
            normals,
            faces,
        };

        Self { model }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl BoxMesh {
    pub fn model(&self) -> &ObjModel {
        &self.model
    }
}
