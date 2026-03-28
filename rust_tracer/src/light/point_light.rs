use crate::{light::Light, math::Vec3, spectrum::sampled_spectrum::{SampledSpectrum, SampledWavelengths}};

use super::{LightLiSample, LightSampleContext, LightType};

pub struct PointLight {
    pub pos: Vec3,
    pub color: Vec3,
}

impl PointLight {
    pub fn new(pos: Vec3, color: Vec3) -> PointLight {
        PointLight { pos, color }
    }
}

impl Light for PointLight {
    fn phi(&self, lambdas: &SampledWavelengths) -> SampledSpectrum {
        todo!()
    }

    fn ltype(&self) -> LightType {
        LightType::DeltaPosition
    }

    fn sample_li(&self, ctx: &LightSampleContext) -> Option<LightLiSample> {
        todo!()
    }

    fn pdf_li(&self, _: &LightSampleContext, _: Vec3) -> f32 {
        0.0
    }

    fn radiance(&self, point: &Vec3, normal: &Vec3, uv: (f32, f32), w: &Vec3, lambdas: &SampledWavelengths) -> SampledSpectrum {
        todo!()
    }
}