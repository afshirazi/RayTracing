use std::{f32, ops::Index};

use crate::spectrum::{
    N_SPECTRUM_SAMPLES, Spectrum,
    sampled_spectrum::{SampledSpectrum, SampledWavelengths},
};

pub struct DenselySampledSpectrum {
    values: Vec<f32>,
    lambda_min: usize,
    lambda_max: usize,
}

impl DenselySampledSpectrum {
    pub fn new(other: impl Spectrum, lambda_min: usize, lambda_max: usize) -> Self {
        let mut values = Vec::with_capacity(lambda_max - lambda_min + 1);
        for lambda in lambda_min..lambda_max {
            values[lambda - lambda_min] = other[lambda as f32];
        }
        Self {
            values,
            lambda_min,
            lambda_max,
        }
    }
}

impl Index<f32> for DenselySampledSpectrum {
    type Output = f32;

    fn index(&self, index: f32) -> &Self::Output {
        let offset = index.round() as usize - self.lambda_min;
        if offset > self.values.len() {
            &0.0
        } else {
            &self.values[offset]
        }
    }
}

impl Spectrum for DenselySampledSpectrum {
    fn max_value(&self) -> f32 {
        self.values
            .iter()
            .fold(f32::NEG_INFINITY, |acc, e| acc.max(*e))
    }

    fn sample(&self, lambdas: &SampledWavelengths) -> SampledSpectrum {
        let mut s = SampledSpectrum::filled(0.0);
        for idx in 0..N_SPECTRUM_SAMPLES {
            s[idx] = self[lambdas[idx]];
        }
        s
    }
}
