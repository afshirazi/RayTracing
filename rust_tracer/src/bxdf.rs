use crate::{
    bxdf::{conductor_bxdf::ConductorBxdf, dielectric_bxdf::DielectricBxdf, diffuse_bxdf::DiffuseBxdf},
    math::{Frame, Vec3},
};

pub mod conductor_bxdf;
pub mod dielectric_bxdf;
pub mod diffuse_bxdf;
pub mod trowbridge_reitz_distribution;

#[derive(Clone)]
pub enum Bxdfs {
    Diffuse(DiffuseBxdf),
    Conductor(ConductorBxdf),
    Dielectric(DielectricBxdf)
}

pub struct Bsdf {
    bxdf: Bxdfs,
    frame: Frame,
}

pub struct BsdfSample {
    pub color: Vec3,
    pub w_i: Vec3,
    pub pdf: f32,
}

impl BsdfSample {
    pub fn new(color: Vec3, w_i: Vec3, pdf: f32) -> Self {
        Self { color, w_i, pdf }
    }
}

// may add TransportMode later

pub trait Bxdf {
    fn f(&self, w_o: &Vec3, w_i: &Vec3) -> Vec3; // gonna change to sampled spectrum once I figure that out
    fn sample_f(&self, w_o: &Vec3, uc: f32, u: (f32, f32)) -> Option<BsdfSample>;
    //fn pdf();
}

impl Bsdf {
    pub fn new(normal: Vec3, dpdus: Vec3, bxdf: Bxdfs) -> Self {
        Self {
            bxdf,
            frame: Frame::from_xz(dpdus.norm(), normal),
        }
    }
}

impl Bxdf for Bsdf {
    fn f(&self, w_o: &Vec3, w_i: &Vec3) -> Vec3 {
        let local_w_o = self.frame.render_to_local(w_o);
        let local_w_i = self.frame.render_to_local(w_i);
        self.bxdf.f(&local_w_o, &local_w_i)
    }

    fn sample_f(&self, w_o: &Vec3, uc: f32, u: (f32, f32)) -> Option<BsdfSample> {
        let local_w_o = self.frame.render_to_local(w_o);
        let mut bs = self.bxdf.sample_f(&local_w_o, uc, u)?;
        if bs.pdf == 0.0 || bs.w_i.z == 0.0 {
            return None;
        }
        bs.w_i = self.frame.local_to_render(&bs.w_i);
        Some(bs)
    }
}

impl Bxdf for Bxdfs {
    fn f(&self, w_o: &Vec3, w_i: &Vec3) -> Vec3 {
        match self {
            Bxdfs::Diffuse(diffuse_bxdf) => diffuse_bxdf.f(w_o, w_i),
            Bxdfs::Conductor(conductor_bxdf) => conductor_bxdf.f(w_o, w_i),
            Bxdfs::Dielectric(dielectric_bxdf) => dielectric_bxdf.f(w_o, w_i),
        }
    }

    fn sample_f(&self, w_o: &Vec3, uc: f32, u: (f32, f32)) -> Option<BsdfSample> {
        match self {
            Bxdfs::Diffuse(diffuse_bxdf) => diffuse_bxdf.sample_f(w_o, uc, u),
            Bxdfs::Conductor(conductor_bxdf) => conductor_bxdf.sample_f(w_o, uc, u),
            Bxdfs::Dielectric(dielectric_bxdf) => dielectric_bxdf.sample_f(w_o, uc, u),
        }
    }
}

pub fn same_hemisphere(w: &Vec3, wp: &Vec3) -> bool {
    w.z * wp.z > 0.0
}
