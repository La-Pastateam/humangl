use anyhow::{Context, Result, anyhow};
use std::fs::read_to_string;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Clone, Debug, Default)]
pub struct ObjModel {
    pub positions: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub normals: Vec<[f32; 3]>,
    pub faces: Vec<[FaceVertex; 3]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FaceVertex {
    pub position_index: usize,
    pub tex_coord_index: Option<usize>,
    pub normal_index: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct RawFaceVertex {
    position_index: i32,
    tex_coord_index: Option<i32>,
    normal_index: Option<i32>,
}

// ── Gestion des erreurs ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ObjError {
    #[error("Ligne {line_number} : ligne malformée '{content}'")]
    MalformedLine { line_number: usize, content: String },

    #[error("Ligne {line_number} : indice invalide ou hors-limites")]
    InvalidIndex { line_number: usize },

    #[error("Erreur I/O : {0}")]
    IoError(std::io::Error),
}

pub fn parse_obj(path: &PathBuf) -> Result<ObjModel> {
    let content = read_to_string(path).map_err(ObjError::IoError)?;

    parse_obj_str(&content)
}

fn parse_obj_str(content: &str) -> Result<ObjModel> {
    let mut model = ObjModel::default();

    for (line_number, line) in content.lines().enumerate().map(|(i, l)| (i + 1, l.trim())) {
        let resolve =
            |idx, len| resolve_index(idx, len).map_err(|_| ObjError::InvalidIndex { line_number });

        let ctx = || ObjError::MalformedLine {
            line_number,
            content: (line.to_string()),
        };
        match line.split_whitespace().next() {
            Some("v") => model
                .positions
                .push(parse_vertex_line(line).with_context(ctx)?),
            Some("vt") => model
                .tex_coords
                .push(parse_texcoord_line(line).with_context(ctx)?),
            Some("vn") => model
                .normals
                .push(parse_normal_line(line).with_context(ctx)?),
            Some("f") => {
                let resolved = parse_face_line(line)
                    .with_context(ctx)?
                    .into_iter()
                    .map(|raw| {
                        Ok(FaceVertex {
                            position_index: resolve(raw.position_index, model.positions.len())?,
                            tex_coord_index: raw
                                .tex_coord_index
                                .map(|i| resolve_index(i, model.tex_coords.len()))
                                .transpose()?,
                            normal_index: raw
                                .normal_index
                                .map(|i| resolve_index(i, model.normals.len()))
                                .transpose()?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()
                    .with_context(ctx)?;

                model.faces.extend(triangulate_face(&resolved));
            }
            _ => {}
        }
    }

    Ok(model)
}

// ── Parsing par type de ligne ────────────────────────────────────────────────

fn parse_vertex_line(line: &str) -> Result<[f32; 3]> {
    let mut split = line.split_whitespace();
    split.next();

    let mut coords = [0.0f32; 3];
    for coord in coords.iter_mut() {
        *coord = split
            .next()
            .ok_or_else(|| anyhow!("not a number"))?
            .parse()?;
    }
    split.next(); // w optionnekl
    if split.next().is_some() {
        return Err(anyhow!("invalid vertex line"));
    }
    Ok(coords)
}

fn parse_texcoord_line(line: &str) -> Result<[f32; 2]> {
    let mut split = line.split_whitespace();
    split.next();

    let mut coords = [0.0f32; 2];
    for coord in coords.iter_mut() {
        *coord = split
            .next()
            .ok_or_else(|| anyhow!("not a number"))?
            .parse()?;
    }
    split.next();
    if split.next().is_some() {
        return Err(anyhow!("Texcoord line has too much values"));
    }
    Ok(coords)
}

fn parse_normal_line(line: &str) -> Result<[f32; 3]> {
    let mut split = line.split_whitespace();
    split.next();

    let mut coords = [0.0f32; 3];
    for coord in coords.iter_mut() {
        *coord = split
            .next()
            .ok_or_else(|| anyhow!("not a number"))?
            .parse()?;
    }

    if split.next().is_some() {
        return Err(anyhow!("Normal line has too much values"));
    }
    Ok(coords)
}

fn parse_face_line(line: &str) -> Result<Vec<RawFaceVertex>> {
    let mut result: Vec<RawFaceVertex> = Vec::with_capacity(4);
    let mut split = line.split_whitespace();

    split.next(); // skip 'f'

    let mut count: Option<usize> = None;
    for parts in split {
        let mut sub_parts = parts.split("/");
        let curr_count = parts.chars().filter(|c| *c == '/').count();
        if *count.get_or_insert(curr_count) != curr_count {
            return Err(anyhow!("Malformed line"));
        }
        let pos = sub_parts.next();
        let tex = sub_parts.next();
        let norm = sub_parts.next();
        let (position_index, tex_coord_index, normal_index) = match (pos, tex, norm) {
            (Some(pos), None, None) => (pos.parse::<i32>()?, None, None),
            (Some(pos), Some(tex), None) => (pos.parse::<i32>()?, Some(tex.parse::<i32>()?), None),
            (Some(pos), Some(""), Some(norm)) => {
                (pos.parse::<i32>()?, None, Some(norm.parse::<i32>()?))
            }
            (Some(pos), Some(tex), Some(norm)) => (
                pos.parse::<i32>()?,
                Some(tex.parse::<i32>()?),
                Some(norm.parse::<i32>()?),
            ),
            _ => return Err(anyhow!("Invalid line")),
        };

        let raw_vertex = RawFaceVertex {
            position_index,
            tex_coord_index,
            normal_index,
        };
        result.push(raw_vertex);
    }

    if result.len() < 3 {
        return Err(anyhow!("not enough parameters"));
    }
    Ok(result)
}

// ── Triangulation ────────────────────────────────────────────────────────────

fn triangulate_face(vertices: &[FaceVertex]) -> Vec<[FaceVertex; 3]> {
    if vertices.len() < 3 {
        return Vec::new();
    }
    let mut triangles: Vec<[FaceVertex; 3]> = Vec::with_capacity(vertices.len() - 2);
    let first_vertice = vertices[0];
    for i in 1..vertices.len() - 1 {
        triangles.push([first_vertice, vertices[i], vertices[i + 1]]);
    }
    triangles
}

// ── Résolution des indices négatifs (relatifs) ──────────────────────────────

fn resolve_index(index: i32, current_count: usize) -> Result<usize> {
    let resolved = if index > 0 {
        (index - 1) as usize
    } else if index < 0 {
        let offset = index.unsigned_abs() as usize;
        current_count
            .checked_sub(offset)
            .ok_or_else(|| anyhow!("relative index out of bounds"))?
    } else {
        return Err(anyhow!("index cannot be 0"));
    };

    if resolved >= current_count {
        return Err(anyhow!("resolved index is out of bound"));
    }
    Ok(resolved)
}

// ── Bounding box / centroid ─────────────────────────────────────────────────

pub fn compute_centroid(positions: &[[f32; 3]]) -> [f32; 3] {
    let (min, max) = compute_bounding_box(positions);

    [
        (max[0] + min[0]) / 2.0,
        (max[1] + min[1]) / 2.0,
        (max[2] + min[2]) / 2.0,
    ]
}

pub fn compute_bounding_box(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    if positions.is_empty() {
        return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    }
    let mut max: [f32; 3] = positions[0];
    let mut min: [f32; 3] = positions[0];

    for pos in &positions[1..] {
        for (i, &val) in pos.iter().enumerate() {
            max[i] = val.max(max[i]);
            min[i] = val.min(min[i]);
        }
    }

    (min, max)
}
