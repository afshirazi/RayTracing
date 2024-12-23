use core::f64;

use super::RayOps;
use crate::math::Vec3;

pub struct Circle {
    center: Vec3,
    radius: f64,
    diff: Vec3,
    spec: Vec3,
    shin: f64,
}

impl Circle {
    pub fn new(center: Vec3, radius: f64, diff: Vec3, spec: Vec3, shin: f64) -> Circle {
        Circle {
            center,
            radius,
            diff,
            spec,
            shin,
        }
    }

    pub fn from_color(center: Vec3, radius: f64, color: &Vec3) -> Circle {
        Circle {
            center,
            radius,
            diff: color * 0.4,
            spec: color * 0.7,
            shin: 20.0,
        }
    }
}

impl RayOps for Circle {
    fn get_intersect(&self, ray: &Vec3, origin: &Vec3) -> Option<Vec3> {
        // a is always 1, unneeded
        let b = 2.0 * (origin - &self.center).dot(ray);
        let c = (origin - &self.center).dot(&(origin - &self.center)) - self.radius * self.radius;

        // quadratic formula
        let mut t1 = (-b + f64::sqrt(b * b - 4.0 * c)) / 2.0;
        let mut t2 = (-b - f64::sqrt(b * b - 4.0 * c)) / 2.0;

        t1 = if t1 < 0.0 { f64::NAN } else { t1 };
        t2 = if t2 < 0.0 { f64::NAN } else { t2 };

        if t1.is_nan() && t2.is_nan() {
            None
        } else if t1.is_nan() {
            Some(origin + &(ray * t2))
        } else if t2.is_nan() {
            Some(origin + &(ray * t1))
        } else if t1 < t2 {
            Some(origin + &(ray * t1))
        } else {
            Some(origin + &(ray * t2))
        }
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        (point - &self.center).norm()
    }
    
    fn get_diff(&self) -> &Vec3 {
        &self.diff
    }
    
    fn get_spec(&self) -> &Vec3 {
        &self.spec
    }
    
    fn get_shin(&self) -> f64 {
        self.shin
    }
}

// equal if same radius and center, not necessarily same object
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect_success() {
        let c = Circle::new(
            Vec3::new(0.0, 0.0, -4.0),
            2.0,
            Vec3::new(0.4, 0.2, 0.76),
            Vec3::empty_vec(),
            10.0,
        );

        let ray = Vec3::new(0.0, 0.0, -1.0);
        let origin = Vec3::empty_vec();

        assert_eq!(c.get_intersect(&ray, &origin).unwrap(), Vec3::new(0.0, 0.0, -2.0));
    }

    #[test]
    fn test_intersect_fail() {
        let c = Circle::new(
            Vec3::new(0.0, 0.0, -4.0),
            2.0,
            Vec3::new(0.4, 0.2, 0.76),
            Vec3::empty_vec(),
            10.0,
        );

        let ray = Vec3::new(20.0, 30.0, -1.0).norm();
        let origin = Vec3::empty_vec();

        assert_eq!(c.get_intersect(&ray, &origin), None);
    }
}