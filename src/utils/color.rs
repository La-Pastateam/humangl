use crate::utils::math::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl RGB {
    pub fn to_vec(&self) -> Vec3 {
        Vec3 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
        }
    }
}
