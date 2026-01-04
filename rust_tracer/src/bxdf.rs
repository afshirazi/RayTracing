use crate::math::{Frame, Vec3};

pub mod diffuse_bxdf;

pub struct Bsdf<T: Bxdf> {
    bxdf: T,
    frame: Frame,
}

pub struct BsdfSample {
    color: Vec3,
    w_i: Vec3,
    pdf: f32,
}

impl BsdfSample {
    pub fn new(color: Vec3, w_i: Vec3, pdf: f32) -> Self {
        Self { color, w_i, pdf }
    }
}

// may add TransportMode later

pub trait Bxdf {
    fn f(&self, w_o: &Vec3, w_i: &Vec3) -> Vec3; // gonna change to sampled spectrum once I figure that out
    fn sample_f(&self, w_o: &Vec3, u: f32, uc: (f32, f32)) -> Option<BsdfSample>;
    //fn pdf();
}

impl<T: Bxdf> Bsdf<T> {
    pub fn new(normal: Vec3, dpdus: Vec3, bxdf: T) -> Self {
        Self {
            bxdf,
            frame: Frame::from_xz(dpdus.norm(), normal),
        }
    }
}

impl<T: Bxdf> Bxdf for Bsdf<T> {
    fn f(&self, w_o: &Vec3, w_i: &Vec3) -> Vec3 {
        let local_w_o = self.frame.to_local(w_o);
        let local_w_i = self.frame.to_local(w_i);
        self.bxdf.f(&local_w_o, &local_w_i)
    }

    fn sample_f(&self, w_o: &Vec3, uc: f32, u: (f32, f32)) -> Option<BsdfSample> {
        let local_w_o = self.frame.to_local(w_o);
        let mut bs = self.bxdf.sample_f(&local_w_o, uc, u)?;
        if bs.pdf == 0.0 || bs.w_i.z == 0.0 {
            return None;
        }
        bs.w_i = self.frame.from_local(&bs.w_i);
        Some(bs)
    }
}

pub fn same_hemisphere(w: &Vec3, wp: &Vec3) -> bool {
    w.z * wp.z > 0.0
}