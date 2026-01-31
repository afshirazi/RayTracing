use super::Sampler;

pub struct IndependentSampler; // TODO: add seed?

impl Sampler for IndependentSampler {
    fn get_1d(&self) -> f32 {
        rand::random()
    }

    fn get_2d(&self) -> super::Sample2d {
        (rand::random(), rand::random())
    }
}