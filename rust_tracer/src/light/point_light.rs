use crate::{
    light::Light,
    math::Vec3,
    spectrum::sampled_spectrum::{SampledSpectrum, SampledWavelengths},
};

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

    fn radiance(
        &self,
        _: &Vec3,
        _: &Vec3,
        _: (f32, f32),
        _: &Vec3,
        _: &SampledWavelengths,
    ) -> SampledSpectrum {
        unimplemented!("Should not get called for point lights.")
    }
}
