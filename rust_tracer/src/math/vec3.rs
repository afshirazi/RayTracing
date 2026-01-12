use std::ops::{Add, Div, Mul, Sub};

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
}

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

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
