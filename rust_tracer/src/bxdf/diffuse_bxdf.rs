use std::f64;

use crate::{
    bxdf::{BsdfSample, Bxdf},
    math::{self, Vec3},
};

pub struct DiffuseBxdf {
    color: Vec3,
}

impl DiffuseBxdf {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Bxdf for DiffuseBxdf {
    fn f(&self, w_o: &Vec3, w_i: &Vec3) -> Vec3 {
        if super::same_hemisphere(w_o, w_i) {
            &self.color * f64::consts::FRAC_1_PI
        } else {
            Vec3::empty_vec()
        }
    }

    /// for now, guaranteed to return a Some(BsdfSample)
    fn sample_f(&self, w_o: &Vec3, _: f32, u: (f32, f32)) -> Option<BsdfSample> {
        let mut w_i = math::sample_cosine_hemisphere(u);
        if w_o.z < 0.0 {
            w_i.z *= -1.0;
        }
        let pdf = math::cosine_hemisphere_pdf(w_i.z.abs() as f32);
        Some(BsdfSample::new(&self.color * f64::consts::FRAC_1_PI, w_i, pdf))
    }
}
