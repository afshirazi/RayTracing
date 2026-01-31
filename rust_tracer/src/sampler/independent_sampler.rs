use super::Sampler;

pub struct IndependentSampler; // TODO: add seed?

impl Sampler for IndependentSampler {
    fn get_1d() -> f32 {
        rand::random()
    }

    fn get_2d() -> super::Sample2d {
        (rand::random(), rand::random())
    }
}