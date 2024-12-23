use crate::math::Vec3;

pub struct Light {
    pub pos: Vec3,
    pub diff: Vec3,
    pub spec: Vec3,
}

impl Light {
    pub fn default_light() -> Light {
        Light {
            pos: Vec3::empty_vec(),
            diff: Vec3::new(1.0, 1.0, 1.0),
            spec: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn new(pos: Vec3, diff: Vec3, spec: Vec3) -> Light {
        Light { pos, diff, spec }
    }
}
