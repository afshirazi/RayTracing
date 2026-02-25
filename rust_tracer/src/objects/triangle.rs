use crate::{
    bxdf::{Bsdf, Bxdfs, diffuse_bxdf::DiffuseBxdf},
    math::Vec3,
};

use super::RayOps;

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    diff: Vec3,
}

impl Triangle {
    pub fn from_color(a: Vec3, b: Vec3, c: Vec3, color: &Vec3) -> Triangle {
        Triangle {
            a,
            b,
            c,
            diff: color * 0.4,
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

        if det < f64::EPSILON && det > -f64::EPSILON {
            return None;
        }

        let s = origin - &self.a; // used in Cramer's rule
        let inv_det = 1.0 / det;
        let u = inv_det * ray_cross_e2.dot(&s);

        if u < 0.0 {
            return None;
        }

        let s_cross_e1 = s.cross(&e1);
        let v = inv_det * s_cross_e1.dot(ray);

        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = inv_det * s_cross_e1.dot(&e2);
        if t > f64::EPSILON {
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

    fn get_tangent(&self, _: &Vec3) -> Vec3 {
        ((&self.c - &self.a) + (&self.c - &self.b)).norm()
    }

    fn get_mat(&self, norm: &Vec3, dpdu: &Vec3) -> Bsdf {
        let bxdf = DiffuseBxdf::new(self.diff.clone());
        Bsdf::new(norm.clone(), dpdu.clone(), Bxdfs::Diffuse(bxdf))
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
