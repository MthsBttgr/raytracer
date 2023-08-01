use crate::point3::Point3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;

/// Struct for a sphere
//#[derive(Default, Clone)]
pub struct Sphere<M: Material> {
	center: Point3,
	radius: f64,
	material: M
}

impl <M: Material> Sphere<M>  {
	pub fn from_center_radius_material(center: Point3, radius: impl Into<f64>, material: M) -> Self {
		Sphere {
			center,
			radius: radius.into(),
			material
		}
	}
}

impl<M: Material> Hitable for Sphere<M> {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		// using quadratic formula to calculate intersections
		let oc = r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = oc.dot_product(&r.direction());
		let c = oc.length_squared() - self.radius * self.radius;
	
		let discriminant = half_b*half_b - a*c;

		if discriminant < 0.0 { return None;}

		// calculates the sqrt once, and saves the value
		let discriminant_sqrt = discriminant.sqrt();

		// detect the nearest hit:
		let hit = (-half_b - discriminant_sqrt) / a;

		if hit < t_min || t_max < hit {
			// adding the sqrt is always further away
			let hit = (-half_b + discriminant_sqrt) / a;
			if hit < t_min || t_max < hit {
				return None;
			}
		}

		let mut record = HitRecord::new(r.at(hit), hit, &self.material);
		let outward_normal = (record.point() - self.center) / self.radius;
		record.set_face_normal(r, outward_normal);

		Some(record)
	}
}