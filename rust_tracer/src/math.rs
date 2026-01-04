mod vec3;
mod frame;

use std::f32;

pub use vec3::Vec3;
pub use frame::Frame;

fn sample_uniform_disk_concentric(u: (f32, f32)) -> (f32, f32) {
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

fn sample_cosine_hemisphere(u: (f32, f32)) -> Vec3 {
    let d = sample_uniform_disk_concentric(u);
    let z = (1.0 - d.0 * d.0 - d.1 * d.1).sqrt().max(0.0);
    Vec3::new(d.0 as f64, d.1 as f64, z as f64)
}

fn cosine_hemisphere_pdf(cos_theta: f32) -> f32 {
    cos_theta * f32::consts::FRAC_1_PI
}