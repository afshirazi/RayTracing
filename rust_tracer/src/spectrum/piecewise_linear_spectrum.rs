use std::ops::Index;

use crate::{math::lerp, spectrum::Spectrum};

use super::sampled_spectrum::{SampledSpectrum, SampledWavelengths};

pub struct PiecewiseLinearSpectrum {
    lambdas: Vec<f32>,
    values: Vec<f32>,
}

impl PiecewiseLinearSpectrum {
    /// `lambdas` needs to be sorted
    pub fn new(lambdas: Vec<f32>, values: Vec<f32>) -> Self {
        Self { lambdas, values }
    }
}



impl Spectrum for PiecewiseLinearSpectrum {
    fn max_value(&self) -> f32 {
        todo!()
    }

    fn sample(&self, lambdas: &SampledWavelengths) -> SampledSpectrum {
        todo!()
    }
    
    fn index(&self, lambda: f32) -> f32 {
        todo!()
    }
}