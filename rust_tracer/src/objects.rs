use crate::math::Vec3;

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
}

pub enum Object {
    Circle(Circle),
    Triangle(Triangle)
}

impl RayOps for Object {
    fn get_intersect(&self, ray: &Vec3, origin: &Vec3) -> Option<Vec3> {
        match self {
            Object::Circle(c) => c.get_intersect(ray, origin),
            Object::Triangle(tri) => tri.get_intersect(ray, origin)
        }
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        match self {
            Object::Circle(c) => c.get_normal(point),
            Object::Triangle(tri) => tri.get_normal(point)
        }
    }
}