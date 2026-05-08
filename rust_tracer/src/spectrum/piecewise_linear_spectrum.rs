use crate::{math::lerp, spectrum::{N_SPECTRUM_SAMPLES, Spectrum}};

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
        *self.values.iter().max_by(|a, b| a.total_cmp(b)).unwrap_or(&0.0)
    }

    fn sample(&self, lambdas: &SampledWavelengths) -> SampledSpectrum {
        let mut s = SampledSpectrum::filled(0.0);
        for idx in 0..N_SPECTRUM_SAMPLES {
            s[idx] = self.index(lambdas[idx]);
        }
        s
    }
    
    fn index(&self, lambda: f32) -> f32 {
        if self.lambdas.is_empty() || lambda < *self.lambdas.first().unwrap() || lambda > *self.lambdas.last().unwrap() {
            0.0
        } else {
            self.lambdas.windows(2)
            .find(|interval| interval[0] <= lambda && interval[1] >= lambda)
            .map(|interval| {
                let u = (lambda - interval[0]) / (interval[1] - interval[0]);
                lerp(u, interval[0], interval[1])
            })
            .unwrap()
        }
    }
}