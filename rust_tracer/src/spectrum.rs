use std::ops::Index;

pub trait Spectrum : Index<usize> {
    fn max_value(&self) -> f32;
}
