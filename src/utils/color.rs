use crate::utils::math::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// ── Associated Functions ─────────────────────────────────────────────────────────────────

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

// ── Methods ─────────────────────────────────────────────────────────────────

impl RGBA {
    pub fn to_vec(&self) -> Vec3 {
        Vec3 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
        }
    }
}

// ── Trait Implementations ─────────────────────────────────────────────────────────────────

impl Default for RGBA {
    fn default() -> Self {
        RGBA::new(255, 255, 255, 255)
    }
}
