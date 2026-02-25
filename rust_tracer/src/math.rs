mod frame;
mod vec3;

use std::f32;

pub use frame::Frame;
use num::complex::Complex32;
pub use vec3::Vec3;
pub use vec3::reflect;

pub fn sample_uniform_disk_concentric(u: (f32, f32)) -> (f32, f32) {
    let u_offset = (2.0 * u.0 - 1.0, 2.0 * u.1 - 1.0); // map to [-1, 1]^2
    if u_offset == (0.0, 0.0) {
        return u_offset;
    }
    let theta;
    let r;
    if u_offset.0.abs() > u_offset.1.abs() {
        r = u_offset.0;
        theta = f32::consts::FRAC_PI_4 * (u_offset.1 / u_offset.0);
    } else {
        r = u_offset.1;
        theta = f32::consts::FRAC_PI_4 * (u_offset.0 / u_offset.1);
    }
    (r * theta.cos(), r * theta.sin())
}

pub fn sample_cosine_hemisphere(u: (f32, f32)) -> Vec3 {
    let d = sample_uniform_disk_concentric(u);
    let z = (1.0 - d.0 * d.0 - d.1 * d.1).sqrt().max(0.0);
    Vec3::new(d.0 as f64, d.1 as f64, z as f64)
}

pub fn cosine_hemisphere_pdf(cos_theta: f32) -> f32 {
    cos_theta * f32::consts::FRAC_1_PI
}

/////////////// CONVENIENCE EXTENSIONS /////////////////////

pub trait NumExtensions {
    type Output;
    fn safe_sqrt(self) -> Self::Output;
    fn sqr(self) -> Self::Output;
}

impl NumExtensions for f64 {
    type Output = f64;
    fn safe_sqrt(self) -> Self::Output {
        self.sqrt().max(0.0)
    }

    fn sqr(self) -> Self::Output {
        self * self
    }
}

impl NumExtensions for f32 {
    type Output = f32;
    fn safe_sqrt(self) -> Self::Output {
        self.sqrt().max(0.0)
    }

    fn sqr(self) -> Self::Output {
        self * self
    }
}

impl NumExtensions for Complex32 {
    type Output = Complex32;

    fn safe_sqrt(self) -> Self::Output {
        unimplemented!()
    }

    fn sqr(self) -> Self::Output {
        self * self
    }
}
