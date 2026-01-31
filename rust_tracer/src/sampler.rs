mod independent_sampler;

pub use independent_sampler::IndependentSampler;

pub type Sample2d = (f32, f32); // TODO: maybe use Point2f like pbrt does, or something else, but refactor under math when needed

pub trait Sampler {
    fn get_1d(&self) -> f32;
    fn get_2d(&self) -> Sample2d;
}