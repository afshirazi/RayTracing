use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use num::complex::Complex32;
use super::NumExtensions;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn empty_vec() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_vec(other: &Vec3) -> Vec3 {
        Vec3 {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }

    /// Returns the dot product
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Vector magnitude
    pub fn mag(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Returns a normalized vector
    pub fn norm(&self) -> Vec3 {
        self * self.mag().recip()
    }

    /// Squared Euclidean distance, useful for comparisons
    pub fn euclid_dist_sq(v1: &Vec3, v2: &Vec3) -> f64 {
        let dx = v1.x - v2.x;
        let dy = v1.y - v2.y;
        let dz = v1.z - v2.z;

        dx * dx + dy * dy + dz * dz
    }

    /// multiply each element by corresponding element in the other Vec3
    pub fn elwise_mul(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

pub mod reflect {
    use super::*;

    /// Reflects `w_i`` around `n`. Expects `n` to be normalized.
    pub fn reflect(w_i: &Vec3, n: &Vec3) -> Vec3 {
        -w_i + 2.0 * w_i.dot(n) * n
    }

    /// Transmits `w_i` into the material. Expects `n` to be normalized.
    pub fn refract(w_i: &Vec3, n: &Vec3, eta: f32) -> Option<Vec3> {
        let mut eta= eta as f64;
        let dummy; // TODO: check if you want to make Vec3 implement Copy instead.
        let mut n = n;
        let mut cos_theta_i = w_i.dot(n);
        
        if cos_theta_i < 0.0 {
            eta = eta.recip();
            cos_theta_i = -cos_theta_i;
            dummy = -n;
            n = &dummy;
        }

        let sin2_theta_i = f64::max(1.0 - cos_theta_i * cos_theta_i, 0.0);
        let sin2_theta_t = sin2_theta_i / eta.sqr();
        if sin2_theta_t > 1.0 {
            return None
        }
        let cos_theta_t = (1.0 - sin2_theta_t).safe_sqrt();

        let w_t = -w_i / eta + (cos_theta_i / eta - cos_theta_t) * n;

        Some(w_t)
    }

    pub fn fresnel_dielectric(cos_theta_i: f32, eta: f32) -> f32 {
        let mut cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
        let mut eta = eta;
        
        if cos_theta_i < 0.0 {
            cos_theta_i = -cos_theta_i;
            eta = eta.recip();
        }

        let sin2_theta_i = f32::max(1.0 - cos_theta_i * cos_theta_i, 0.0);
        let sin2_theta_t = sin2_theta_i / eta.sqr();
        if sin2_theta_t > 1.0 {
            return 1.0
        }
        let cos_theta_t = (1.0 - sin2_theta_t).safe_sqrt();

        let r_parl = (eta * cos_theta_i - cos_theta_t) / (eta * cos_theta_i + cos_theta_t);
        let r_perp = (cos_theta_i - eta * cos_theta_t) / (cos_theta_i + eta * cos_theta_t);

        (r_parl.sqr() + r_perp.sqr()) / 2.0
    }

    pub fn fresnel_complex(cos_theta_i: f32, eta: Complex32) -> f32 {
        let cos_theta_i = cos_theta_i.clamp(0.0, 1.0);

        let sin2_theta_i = 1.0 - cos_theta_i.sqr();
        let sin2_theta_t = sin2_theta_i / eta.sqr();
        let cos_theta_t = (1.0 - sin2_theta_t).sqrt();

        let r_parl = (eta * cos_theta_i - cos_theta_t) / (eta * cos_theta_i + cos_theta_t);
        let r_perp = (cos_theta_i - eta * cos_theta_t) / (cos_theta_i + eta * cos_theta_t);

        (r_parl.norm_sqr() + r_perp.norm_sqr()) / 2.0
    }

    pub fn fresnel_complex_spec(cos_theta_i: f32, eta: &Vec3, k: &Vec3) -> Vec3 {
        let mut res = Vec3::empty_vec();
        // TODO: clean up (f64 -> f32, Vec3 -> SampledSpectrum, build the result more dynamically)
        res.x = fresnel_complex(cos_theta_i, Complex32::new(eta.x as f32, k.x as f32)) as f64;
        res.y = fresnel_complex(cos_theta_i, Complex32::new(eta.y as f32, k.y as f32)) as f64;
        res.z = fresnel_complex(cos_theta_i, Complex32::new(eta.z as f32, k.z as f32)) as f64;

        res
    }
}


/////////////// OPERATOR OVERLOADING /////////////////////

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
