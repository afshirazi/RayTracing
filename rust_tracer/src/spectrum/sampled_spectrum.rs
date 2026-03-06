use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::spectrum::N_SPECTRUM_SAMPLES;

pub struct SampledSpectrum {
    values: [f32; N_SPECTRUM_SAMPLES],
}

impl SampledSpectrum {
    pub fn new(values: [f32; N_SPECTRUM_SAMPLES]) -> Self {
        Self { values }
    }

    pub fn filled(value: f32) -> Self {
        Self {
            values: [value; N_SPECTRUM_SAMPLES],
        }
    }

    pub fn is_zero(&self) -> bool {
        for i in self.values {
            if i != 0.0 {
                return true;
            }
        }
        false
    }

    pub fn safe_div(&self, rhs: &Self) -> Self {
        let mut values = [0.0; N_SPECTRUM_SAMPLES];
        for i in 0..N_SPECTRUM_SAMPLES {
            values[i] = if rhs[i] != 0.0 { self[i] / rhs[i] } else { 0.0 };
        }

        Self { values }
    }
}

/////////////// OPERATOR OVERLOADING /////////////////////

impl Index<usize> for SampledSpectrum {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for SampledSpectrum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl AddAssign for SampledSpectrum {
    fn add_assign(&mut self, rhs: Self) {
        for idx in 0..N_SPECTRUM_SAMPLES {
            self[idx] += rhs[idx]
        }
    }
}

impl SubAssign for SampledSpectrum {
    fn sub_assign(&mut self, rhs: Self) {
        for idx in 0..N_SPECTRUM_SAMPLES {
            self[idx] -= rhs[idx]
        }
    }
}

impl MulAssign for SampledSpectrum {
    fn mul_assign(&mut self, rhs: Self) {
        for idx in 0..N_SPECTRUM_SAMPLES {
            self[idx] *= rhs[idx]
        }
    }
}

impl DivAssign for SampledSpectrum {
    fn div_assign(&mut self, rhs: Self) {
        for idx in 0..N_SPECTRUM_SAMPLES {
            self[idx] /= rhs[idx]
        }
    }
}

impl MulAssign<f32> for SampledSpectrum {
    fn mul_assign(&mut self, rhs: f32) {
        for idx in 0..N_SPECTRUM_SAMPLES {
            self[idx] *= rhs
        }
    }
}

impl DivAssign<f32> for SampledSpectrum {
    fn div_assign(&mut self, rhs: f32) {
        for idx in 0..N_SPECTRUM_SAMPLES {
            self[idx] /= rhs
        }
    }
}

impl Add for SampledSpectrum {
    type Output = SampledSpectrum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; N_SPECTRUM_SAMPLES];
        for i in 0..N_SPECTRUM_SAMPLES {
            values[i] = self[i] + rhs[i];
        }

        Self { values }
    }
}

impl Sub for SampledSpectrum {
    type Output = SampledSpectrum;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; N_SPECTRUM_SAMPLES];
        for i in 0..N_SPECTRUM_SAMPLES {
            values[i] = self[i] - rhs[i];
        }

        Self { values }
    }
}

impl Mul for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; N_SPECTRUM_SAMPLES];
        for i in 0..N_SPECTRUM_SAMPLES {
            values[i] = self[i] * rhs[i];
        }

        Self { values }
    }
}

impl Div for SampledSpectrum {
    type Output = SampledSpectrum;

    fn div(self, rhs: Self) -> Self::Output {
        let mut values = [0.0; N_SPECTRUM_SAMPLES];
        for i in 0..N_SPECTRUM_SAMPLES {
            values[i] = self[i] / rhs[i];
        }

        Self { values }
    }
}

impl Mul<f32> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut values = [0.0; N_SPECTRUM_SAMPLES];
        for i in 0..N_SPECTRUM_SAMPLES {
            values[i] = self[i] * rhs;
        }

        Self { values }
    }
}

impl Div<f32> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn div(self, rhs: f32) -> Self::Output {
        let mut values = [0.0; N_SPECTRUM_SAMPLES];
        for i in 0..N_SPECTRUM_SAMPLES {
            values[i] = self[i] / rhs;
        }

        Self { values }
    }
}