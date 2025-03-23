use std::f64::EPSILON;

use crate::math::Vec3;

use super::RayOps;

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    diff: Vec3,
    spec: Vec3,
    shin: f64,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, diff: Vec3, spec: Vec3, shin: f64) -> Triangle {
        Triangle {
            a,
            b,
            c,
            diff,
            spec,
            shin,
        }
    }

    pub fn from_color(a: Vec3, b: Vec3, c: Vec3, color: &Vec3) -> Triangle {
        Triangle {
            a,
            b,
            c,
            diff: color * 0.4,
            spec: color * 0.7,
            shin: 20.0,
        }
    }
}

impl RayOps for Triangle {
    fn get_intersect(&self, ray: &Vec3, origin: &Vec3) -> Option<Vec3> {
        // Moller-Trumbore
        // t          (s x e1) . e2
        // u =  1/det (ray x e2) . s
        // v          (s x e1) . ray

        let e1 = &self.b - &self.a;
        let e2 = &self.c - &self.a;
        let ray_cross_e2 = ray.cross(&e2);

        let det = e1.dot(&ray_cross_e2); // (R x e2) . e1

        if det < EPSILON && det > -EPSILON {
            return None;
        }

        let s = origin - &self.a; // used in Cramer's rule
        let inv_det = 1.0 / det;
        let u = inv_det * ray_cross_e2.dot(&s);

        if u < 0.0 {
            return None;
        }

        let s_cross_e1 = s.cross(&e1);
        let v = inv_det * s_cross_e1.dot(&ray);

        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = inv_det * s_cross_e1.dot(&e2);
        if t > EPSILON {
            Some(origin + &(ray * t))
        } else {
            // t is negative, there is a line intersection but in the opposite direction of the ray.
            None
        }
    }

    fn get_normal(&self, _point: &Vec3) -> Vec3 {
        let ab = &self.b - &self.a;
        let ac = &self.c - &self.a;

        ab.cross(&ac).norm()
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

// equal if same dimensions, not necessarily same object
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect_success() {
        let tri = Triangle::from_color(
            Vec3::new(2.0, -3.0, -10.0),
            Vec3::new(0.0, 1.5, -11.0),
            Vec3::new(-1.5, -3.0, -9.0),
            &Vec3::new(0.4, 0.2, 0.76),
        );

        let ray = Vec3::new(0.0, 0.0, -1.0);
        let origin = Vec3::empty_vec();

        assert!(tri.get_intersect(&ray, &origin).is_some());
    }
}
