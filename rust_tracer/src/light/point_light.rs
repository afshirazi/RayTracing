use std::f32;

use crate::{
    light::Light, math::{Matrix, Transform, Vec3}, sampler::Sample2d, spectrum::{Spectrum, densely_sampled_spectrum::DenselySampledSpectrum, sampled_spectrum::{SampledSpectrum, SampledWavelengths}}
};

use super::{LightLiSample, LightSampleContext, LightType};

pub struct PointLight {
    pub pos: Vec3,
    pub color: Vec3,
    spectrum: Option<DenselySampledSpectrum>,
    scale: f32,
    transform: Transform
}

impl PointLight {
    pub fn new(pos: Vec3, color: Vec3) -> PointLight {
        PointLight { pos, color, spectrum: None, scale: 0.0, transform: Transform::new(Matrix::identity()) }
    }
}

impl Light for PointLight {
    fn phi(&self, lambdas: &SampledWavelengths) -> SampledSpectrum {
        if let Some(spc) = self.spectrum.as_ref() {
            4.0 * f32::consts::PI * self.scale * &spc.sample(lambdas)
        } else {
            SampledSpectrum::filled(0.0)
        }
    }

    fn ltype(&self) -> LightType {
        LightType::DeltaPosition
    }

    //TODO: remove optional spectrum
    fn sample_li(&self, ctx: &LightSampleContext, _: Sample2d, lambdas: &SampledWavelengths) -> Option<LightLiSample> {
        let point = self.transform.apply_point(&Vec3::empty_vec());
        let w_i = (&point - &ctx.point).norm();
        let li = self.spectrum.as_ref()?.sample(lambdas) * self.scale / Vec3::euclid_dist_sq(&point, &ctx.point);
        Some(LightLiSample::new(li, w_i, 1.0))
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
