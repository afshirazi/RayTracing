mod point_light;

pub use point_light::PointLight;

use crate::{
    math::Vec3,
    spectrum::sampled_spectrum::{SampledSpectrum, SampledWavelengths},
};

pub trait Light {
    fn phi(&self, lambdas: &SampledWavelengths) -> SampledSpectrum;
    fn ltype(&self) -> LightType;
    fn sample_li(&self, ctx: &LightSampleContext) -> Option<LightLiSample>;
    fn pdf_li(&self, ctx: &LightSampleContext, w_i: Vec3) -> f32;
    /// Also referred to as L().
    // TODO: define point 2D and 3D and vec 2d?
    fn radiance(&self, point: &Vec3, normal: &Vec3, uv: (f32, f32), w: &Vec3, lambdas: &SampledWavelengths) -> SampledSpectrum;
    // fn le(&self, origin: Vec3, dir: Vec3, lambdas: &SampledWavelengths) -> SampledSpectrum;
}

pub struct LightSampleContext {
    pub point: Vec3,
    pub normal: Vec3,
}

impl LightSampleContext {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }
}

pub struct LightLiSample {
    pub radiance: SampledSpectrum,
    pub w_i: Vec3,
    pub pdf: f32,
}

impl LightLiSample {
    pub fn new(radiance: SampledSpectrum, w_i: Vec3, pdf: f32) -> Self {
        Self { radiance, w_i, pdf }
    }
}

pub enum LightType {
    DeltaPosition,
    DeltaDirection,
    Area,
}

impl LightType {
    pub fn is_delta(&self) -> bool {
        match self {
            Self::DeltaDirection | Self::DeltaPosition => true,
            _ => false,
        }
    }
}
