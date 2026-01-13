use crate::{bxdf::Bsdf, math::Vec3};

pub mod circle;
pub mod triangle;

pub use circle::Circle;
pub use triangle::Triangle;

pub trait RayOps {
    /// Returns a `Some(Vec3)` containing a point on the surface where
    /// there's an intersection (closest point if it's a circle), or `None`
    /// if there's no intersection.
    /// `ray` must be normalized.
    fn get_intersect(&self, ray: &Vec3, origin: &Vec3) -> Option<Vec3>;

    /// Get the normal to a point on the surface. Will return the
    /// same for any input if it's a triangle. Expects a valid point on the
    /// surface, does not check for validity.
    fn get_normal(&self, point: &Vec3) -> Vec3;
    
    /// Returns a tangent vector to the surface of the shape
    /// Expects a valid point on the surface.
    fn get_tangent(&self, point: &Vec3) -> Vec3;

    /// Returns the material (BSDF for now)
    fn get_mat(&self, norm: &Vec3, dpdu: &Vec3) -> Bsdf;
}

pub enum Object {
    Circle(Circle),
    Triangle(Triangle),
}

impl RayOps for Object {
    fn get_intersect(&self, ray: &Vec3, origin: &Vec3) -> Option<Vec3> {
        match self {
            Object::Circle(c) => c.get_intersect(ray, origin),
            Object::Triangle(tri) => tri.get_intersect(ray, origin),
        }
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        match self {
            Object::Circle(c) => c.get_normal(point),
            Object::Triangle(tri) => tri.get_normal(point),
        }
    }
    
    fn get_tangent(&self, point: &Vec3) -> Vec3 {
        match self {
            Object::Circle(circle) => circle.get_tangent(point),
            Object::Triangle(triangle) => triangle.get_tangent(point),
        }
    }
    
    fn get_mat(&self, norm: &Vec3, dpdu: &Vec3) -> Bsdf {
        match self {
            Object::Circle(circle) => circle.get_mat(norm, dpdu),
            Object::Triangle(triangle) => triangle.get_mat(norm, dpdu),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Circle(l), Self::Circle(r)) => l == r,
            (Self::Triangle(l), Self::Triangle(r)) => l == r,
            _ => false,
        }
    }
}
