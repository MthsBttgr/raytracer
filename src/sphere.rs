use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

/// Struct for a sphere
pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    /// Construct a sphere from the center position, it's radius, and it's material
    pub fn from_center_radius_material(
        center: Point3,
        radius: impl Into<f64>,
        material: M,
    ) -> Self {
        Sphere {
            center,
            radius: radius.into(),
            material,
        }
    }
}

impl<M: Material> Hitable for Sphere<M> {
    /// Calculates if a ray hits the sphere, and returns a hitrecord if it is hit.
    /// Else it returns none.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // using quadratic formula to calculate intersections
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot_product(&r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // calculates the sqrt once, and saves the value
        let discriminant_sqrt = discriminant.sqrt();

        // detect the nearest hit:
        let mut hit = (-half_b - discriminant_sqrt) / a;
        if hit <= t_min || t_max <= hit {
            // adding the sqrt is always further away
            hit = (-half_b + discriminant_sqrt) / a;
            if hit <= t_min || t_max <= hit {
                return None;
            }
        }

        // Constructs the hitrecord
        let point = r.at(hit);
        let record = HitRecord::new(
            point,
            hit,
            (point - self.center) / self.radius,
            &self.material,
        );

        Some(record)
    }
}
