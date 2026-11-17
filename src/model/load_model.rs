use std::collections::HashMap;
use std::mem::size_of;

use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use crate::app::AppData;
use crate::model::{Model, Skeleton};
use crate::parser::obj::FaceVertex;
use crate::utils::math::{Vec2, Vec3, vec2, vec3};

// ── Constants ─────────────────────────────────────────────────────────────────

// ── Vertex Data ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: Vec3,
    color: Vec3,
    tex_coord: Vec2,
}

impl Vertex {
    const fn new(pos: Vec3, color: Vec3, tex_coord: Vec2) -> Self {
        Self {
            pos,
            color,
            tex_coord,
        }
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
            .build();

        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<Vec3>() as u32)
            .build();

        let tex_coord = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(vk::Format::R32G32_SFLOAT)
            .offset((size_of::<Vec3>() + size_of::<Vec3>()) as u32)
            .build();
        [pos, color, tex_coord]
    }
}

// ── Model Loading ─────────────────────────────────────────────────────────────────

fn build_vertex(model: &Model, fv: &FaceVertex) -> Vertex {
    let model_pos = model.obj_model.positions[fv.position_index];
    let tex_coord = match fv.tex_coord_index {
        Some(i) => {
            let model_vert = model.obj_model.tex_coords[i];
            vec2(model_vert[0], 1.0 - model_vert[1])
        }
        None => vec2(0.0, 0.0),
    };

    Vertex {
        pos: vec3(model_pos[0], model_pos[1], model_pos[2]),
        color: model.color,
        tex_coord,
    }
}

pub fn load_model(data: &mut AppData, model: &Model) -> Result<()> {
    let mut unique_vertices = HashMap::new();

    for face in &model.obj_model.faces {
        for fv in face {
            if let Some(i) = unique_vertices.get(fv) {
                data.indices.push(*i as u32);
            } else {
                let vertex = build_vertex(model, fv);
                let index = data.vertices.len();
                unique_vertices.insert(fv, index);
                data.vertices.push(vertex);
                data.indices.push(index as u32);
            }
        }
    }
    Ok(())
}

pub fn load_skeleton(skeleton: &Skeleton, data: &mut AppData) -> Result<()> {
    for bone in skeleton.bones() {
        load_model(data, &bone.model)?;
    }
    Ok(())
}
