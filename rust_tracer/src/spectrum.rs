use std::ops::Index;

// pbrt defines these so I will too :)
const MIN_LAMBDA: f32 = 360.0;
const MAX_LAMBDA: f32 = 830.0;

pub trait Spectrum : Index<usize> {
    fn max_value(&self) -> f32;
}
