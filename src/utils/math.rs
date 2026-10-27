use std::f32::consts::PI;
use std::ops::{Div, Index, IndexMut, Mul, Sub};

// ── Type ─────────────────────────────────────────────────────────────────

pub type Vec2 = Vector2;
pub type Vec3 = Vector3;
pub type Mat4 = Matrix4;

// ── Point 3 ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sub<Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Point3) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Point3 {
    fn dot(&self, rhs: Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

pub const fn point3(x: f32, y: f32, z: f32) -> Point3 {
    Point3 { x, y, z }
}

// ── Vector 2 ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub const fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
}

// ── Vector 3 ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magn = self.magnitude();
        if magn == 0.0 {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let inv = 1.0 / magn; // + opti
            Self {
                x: self.x * inv,
                y: self.y * inv,
                z: self.z * inv,
            }
        }
    }

    pub fn cross(&self, rhs: Vector3) -> Vector3 {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
}

// ── Vector 4 ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
    Vector4 { x, y, z, w }
}

// ── Matrix 4 ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl Matrix4 {
    pub const fn new(
        c0r0: f32,
        c0r1: f32,
        c0r2: f32,
        c0r3: f32,
        c1r0: f32,
        c1r1: f32,
        c1r2: f32,
        c1r3: f32,
        c2r0: f32,
        c2r1: f32,
        c2r2: f32,
        c2r3: f32,
        c3r0: f32,
        c3r1: f32,
        c3r2: f32,
        c3r3: f32,
    ) -> Matrix4 {
        Matrix4::from_cols(
            Vector4::new(c0r0, c0r1, c0r2, c0r3),
            Vector4::new(c1r0, c1r1, c1r2, c1r3),
            Vector4::new(c2r0, c2r1, c2r2, c2r3),
            Vector4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    pub const fn from_cols(c0: Vector4, c1: Vector4, c2: Vector4, c3: Vector4) -> Matrix4 {
        Matrix4 {
            x: c0,
            y: c1,
            z: c2,
            w: c3,
        }
    }

    pub fn from_translation(v: Vector3) -> Matrix4 {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, v.x, v.y, v.z, 1.0,
        )
    }

    pub fn from_axis_angle<A: Into<Rad>>(axis: Vector3, angle: A) -> Matrix4 {
        let rad: Rad = angle.into();
        let s = rad.0.sin();
        let c = rad.0.cos();
        let _1subc = 1.0 - c;

        mat4(
            vec4(
                _1subc * axis.x * axis.x + c,
                _1subc * axis.x * axis.y + s * axis.z,
                _1subc * axis.x * axis.z - s * axis.y,
                0.0,
            ),
            vec4(
                _1subc * axis.x * axis.y - s * axis.z,
                _1subc * axis.y * axis.y + c,
                _1subc * axis.y * axis.z + s * axis.x,
                0.0,
            ),
            vec4(
                _1subc * axis.x * axis.z + s * axis.y,
                _1subc * axis.y * axis.z - s * axis.x,
                _1subc * axis.z * axis.z + c,
                0.0,
            ),
            vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn look_to_rh(eye: Point3, dir: Vector3, up: Vector3) -> Matrix4 {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        mat4(
            vec4(s.x, u.x, -f.x, 0.0),
            vec4(s.y, u.y, -f.y, 0.0),
            vec4(s.z, u.z, -f.z, 0.0),
            vec4(-eye.dot(s), -eye.dot(u), eye.dot(f), 1.0),
        )
    }

    pub fn look_at_rh(eye: Point3, center: Point3, up: Vector3) -> Matrix4 {
        Matrix4::look_to_rh(eye, center - eye, up)
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vector4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        let x = self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z + self.w.x * rhs.w;
        let y = self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z + self.w.y * rhs.w;
        let z = self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z + self.w.z * rhs.w;
        let w = self.x.w * rhs.x + self.y.w * rhs.y + self.z.w * rhs.z + self.w.w * rhs.w;

        Vector4 { x, y, z, w }
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let x = Vector4::new(
            self.x.x * rhs.x.x + self.y.x * rhs.x.y + self.z.x * rhs.x.z + self.w.x * rhs.x.w,
            self.x.y * rhs.x.x + self.y.y * rhs.x.y + self.z.y * rhs.x.z + self.w.y * rhs.x.w,
            self.x.z * rhs.x.x + self.y.z * rhs.x.y + self.z.z * rhs.x.z + self.w.z * rhs.x.w,
            self.x.w * rhs.x.x + self.y.w * rhs.x.y + self.z.w * rhs.x.z + self.w.w * rhs.x.w,
        );
        let y = Vector4::new(
            self.x.x * rhs.y.x + self.y.x * rhs.y.y + self.z.x * rhs.y.z + self.w.x * rhs.y.w,
            self.x.y * rhs.y.x + self.y.y * rhs.y.y + self.z.y * rhs.y.z + self.w.y * rhs.y.w,
            self.x.z * rhs.y.x + self.y.z * rhs.y.y + self.z.z * rhs.y.z + self.w.z * rhs.y.w,
            self.x.w * rhs.y.x + self.y.w * rhs.y.y + self.z.w * rhs.y.z + self.w.w * rhs.y.w,
        );
        let z = Vector4::new(
            self.x.x * rhs.z.x + self.y.x * rhs.z.y + self.z.x * rhs.z.z + self.w.x * rhs.z.w,
            self.x.y * rhs.z.x + self.y.y * rhs.z.y + self.z.y * rhs.z.z + self.w.y * rhs.z.w,
            self.x.z * rhs.z.x + self.y.z * rhs.z.y + self.z.z * rhs.z.z + self.w.z * rhs.z.w,
            self.x.w * rhs.z.x + self.y.w * rhs.z.y + self.z.w * rhs.z.z + self.w.w * rhs.z.w,
        );
        let w = Vector4::new(
            self.x.x * rhs.w.x + self.y.x * rhs.w.y + self.z.x * rhs.w.z + self.w.x * rhs.w.w,
            self.x.y * rhs.w.x + self.y.y * rhs.w.y + self.z.y * rhs.w.z + self.w.y * rhs.w.w,
            self.x.z * rhs.w.x + self.y.z * rhs.w.y + self.z.z * rhs.w.z + self.w.z * rhs.w.w,
            self.x.w * rhs.w.x + self.y.w * rhs.w.y + self.z.w * rhs.w.z + self.w.w * rhs.w.w,
        );

        Matrix4::from_cols(x, y, z, w)
    }
}

impl From<PerspectiveFov> for Matrix4 {
    fn from(p: PerspectiveFov) -> Matrix4 {
        assert!(
            p.fovy > Rad(0.0),
            "The vertical field of view cannot be below zero, found: {:?}",
            p.fovy
        );
        assert!(
            p.fovy < Rad::turn_div_2(),
            "The vertical field of view cannot be greater than a half turn, found: {:?}",
            p.fovy
        );

        assert!(
            p.aspect.abs() > f32::EPSILON,
            "The absolute aspect ratio cannot be zero, found: {:?}",
            p.aspect.abs()
        );
        assert!(
            p.near > 0.0,
            "The near plane distance cannot be below zero, found: {:?}",
            p.near
        );
        assert!(
            p.far > 0.0,
            "The far plane distance cannot be below zero, found: {:?}",
            p.far
        );
        assert!(
            (p.far - p.near).abs() > f32::EPSILON,
            "The far plane and near plane are too close, found: far: {:?}, near: {:?}",
            p.far,
            p.near
        );

        let two: f32 = 2.0;
        let f = Rad::cot(p.fovy.0 / two);

        mat4(
            vec4(f / p.aspect, 0.0, 0.0, 0.0),
            vec4(0.0, f, 0.0, 0.0),
            vec4(0.0, 0.0, (p.far + p.near) / (p.near - p.far), -1.0),
            vec4(0.0, 0.0, (two * p.far * p.near) / (p.near - p.far), 0.0),
        )
    }
}

pub const fn mat4(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Matrix4 {
    Matrix4 { x, y, z, w }
}

// ── Deg & Rad ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Deg(pub f32);

impl Mul<f32> for Deg {
    type Output = Deg;
    fn mul(self, rhs: f32) -> Deg {
        Deg(self.0 * rhs)
    }
}

impl From<Rad> for Deg {
    #[inline]
    fn from(rad: Rad) -> Deg {
        Deg(rad.0 * (180.0 / PI))
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Rad(pub f32);

impl Rad {
    pub fn turn_div_2() -> Rad {
        Rad(PI)
    }

    pub fn cot(x: f32) -> f32 {
        1.0 / x.tan()
    }
}

impl Div<f32> for Rad {
    type Output = Rad;
    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "div by zero");
        Rad(self.0 / rhs)
    }
}

impl Div<Rad> for Rad {
    type Output = f32;
    fn div(self, rhs: Rad) -> Self::Output {
        debug_assert!(rhs.0 != 0.0, "div by zero");
        self.0 / rhs.0
    }
}

impl Mul<f32> for Rad {
    type Output = Rad;
    fn mul(self, rhs: f32) -> Rad {
        Rad(self.0 * rhs)
    }
}

impl From<Deg> for Rad {
    #[inline]
    fn from(deg: Deg) -> Rad {
        Rad(deg.0 * (PI / 180.0))
    }
}

// ── Perspective ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Perspective {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct PerspectiveFov {
    pub fovy: Rad,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

pub fn perspective<A: Into<Rad>>(fovy: A, aspect: f32, near: f32, far: f32) -> Matrix4 {
    PerspectiveFov {
        fovy: fovy.into(),
        aspect,
        near,
        far,
    }
    .into()
}
