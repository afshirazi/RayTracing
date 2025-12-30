use crate::math::Vec3;

pub mod diffuse_bxdf;

pub struct BsdfSample {
    color: Vec3,
    w_i: Vec3,
    pdf: f32
}

// may add TransportMode later

pub trait Bxdf {
    fn f(w_o: Vec3, w_i: Vec3) -> Vec3; // gonna change to sampled spectrum once I figure that out
    fn sample_f(w_o: Vec3, u: f32, uc: (f32, f32)) -> BsdfSample;
    //fn pdf();
}