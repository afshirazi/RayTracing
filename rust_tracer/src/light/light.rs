use crate::math::Vec3;

pub struct Light {
    pub pos: Vec3,
    pub color: Vec3,
}

impl Light {
    pub fn new(pos: Vec3, color: Vec3) -> Light {
        Light { pos, color }
    }
}
