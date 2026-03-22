use std::ops::Index;

use sampled_spectrum::SampledSpectrum;
use sampled_spectrum::SampledWavelengths;

pub mod sampled_spectrum;
pub mod densely_sampled_spectrum;

// pbrt defines these so I will too :)
const MIN_LAMBDA: f32 = 360.0;
const MAX_LAMBDA: f32 = 830.0;
const N_SPECTRUM_SAMPLES: usize = 4;

pub trait Spectrum: Index<f32, Output = f32> {
    fn max_value(&self) -> f32;
    fn sample(&self, lambdas: &SampledWavelengths) -> SampledSpectrum;
}
