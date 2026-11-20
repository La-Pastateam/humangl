use crate::utils::math::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl Rgba {
    pub fn to_vec(self) -> Vec3 {
        Vec3 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
        }
    }
}

// ── Trait Implementations ─────────────────────────────────────────────────────────────────

impl Default for Rgba {
    fn default() -> Self {
        Rgba::new(255, 255, 255, 255)
    }
}
