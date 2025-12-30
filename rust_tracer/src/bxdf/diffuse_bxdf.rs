use crate::{bxdf::Bxdf, math::Vec3};


pub struct DiffuseBxdf {
    color: Vec3
}

impl DiffuseBxdf {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Bxdf for DiffuseBxdf {
    fn f(w_o: Vec3, w_i: Vec3) -> Vec3 {
        todo!()
    }

    fn sample_f(w_o: Vec3, u: f32, uc: (f32, f32)) -> super::BsdfSample {
        todo!()
    }
}
