use std::ops::Index;

use sampled_spectrum::SampledSpectrum;
use sampled_spectrum::SampledWavelengths;

pub mod densely_sampled_spectrum;
pub mod sampled_spectrum;
pub mod piecewise_linear_spectrum;

// pbrt defines these so I will too :)
pub const MIN_LAMBDA: f32 = 360.0;
pub const MAX_LAMBDA: f32 = 830.0;
pub const N_SPECTRUM_SAMPLES: usize = 4;

pub trait Spectrum {
    fn max_value(&self) -> f32;
    fn sample(&self, lambdas: &SampledWavelengths) -> SampledSpectrum;
    fn index(&self, lambda: f32) -> f32;
}
