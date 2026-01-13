use crate::math::Vec3;

/// x, y, z must be orthonormal
//TODO: can add check for orthonormality in debug builds, like PBRT
pub struct Frame {
    x: Vec3,
    y: Vec3,
    z: Vec3,
} 

impl Frame {
    pub fn new(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self { x, y, z }
    }

    pub fn from_xz(x: Vec3, z: Vec3) -> Self {
        let y = z.cross(&x);
        Self { x, y, z }
    }

    pub fn render_to_local(&self, v: &Vec3) -> Vec3 {
        Vec3::new(v.dot(&self.x), v.dot(&self.y), v.dot(&self.z))
    }

    pub fn local_to_render(&self, v: &Vec3) -> Vec3 {
        v.x * &self.x + v.y * &self.y + v.z * &self.z
    }
}