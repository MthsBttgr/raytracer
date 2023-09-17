use crate::point3::{Point3, Vec3};

/// Simple ray struct
/// Represents a straight line in 3d space
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get a point on the ray
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    /// Get the origin of the ray, which is basically just the camera
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// Get the direction of the ray
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
