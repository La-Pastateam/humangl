use crate::utils::color::Rgba;
use crate::utils::math::{Deg, Matrix4, Vector3, vec3};

// ── BONEBUILDER ─────────────────────────────────────────────────────────────────

#[must_use]
#[derive(Default, Debug)]
pub struct BoneBuilder {
    label: String,
    id: usize,
    parent_id: Option<usize>,
    color: Rgba,
    dimension: Vector3,
    anchor: Vector3,
    pivot: Vector3,
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl BoneBuilder {
    pub fn parent(mut self, id: usize) -> Self {
        self.parent_id = Some(id);
        self
    }

    pub fn color(mut self, color: Rgba) -> Self {
        self.color = color;
        self
    }

    pub fn dimension(mut self, dimension: Vector3) -> Self {
        self.dimension = dimension;
        self
    }

    pub fn anchor(mut self, anchor: Vector3) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn pivot(mut self, pivot: Vector3) -> Self {
        self.pivot = pivot;
        self
    }

    pub fn build(self) -> Bone {
        Bone {
            label: self.label,
            id: self.id,
            parent_id: self.parent_id,
            color: self.color,
            dimension: self.dimension,
            anchor: self.anchor,
            pivot: self.pivot,
            ..Default::default()
        }
    }
}

// ── BONE ─────────────────────────────────────────────────────────────────

#[derive(PartialEq, Default, Debug, Clone)]
pub struct Bone {
    pub label: String,
    id: usize,
    pub parent_id: Option<usize>,
    pub color: Rgba,
    pub dimension: Vector3,
    pub rotation: Vector3,
    pub anchor: Vector3,
    pub pivot: Vector3,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn builder(label: impl Into<String>, id: usize) -> BoneBuilder {
        BoneBuilder {
            label: label.into(),
            id,
            dimension: vec3(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Bone {
    pub fn change_color(&mut self, color: Rgba) {
        self.color = color;
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn color(&self) -> Rgba {
        self.color
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn transform(&self, parent: Option<&Bone>) -> Matrix4 {
        let anchor = match parent {
            Some(p) => vec3(
                (self.anchor.x - p.pivot.x) * p.dimension.x,
                (self.anchor.y - p.pivot.y) * p.dimension.y,
                (self.anchor.z - p.pivot.z) * p.dimension.z,
            ),
            None => self.anchor,
        };

        Matrix4::from_translation(anchor)
            * Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(self.rotation.x))
            * Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(self.rotation.y))
            * Matrix4::from_axis_angle(vec3(0.0, 0.0, 1.0), Deg(self.rotation.z))
    }
}
