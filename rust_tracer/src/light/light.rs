use crate::math::Vec3;

pub struct PointLight {
    pub pos: Vec3,
    pub color: Vec3,
}

impl PointLight {
    pub fn new(pos: Vec3, color: Vec3) -> PointLight {
        PointLight { pos, color }
    }
}
