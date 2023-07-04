use crate::point3::Point3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};

/// Struct for a sphere
#[derive(Default, Copy, Clone, Debug)]
pub struct Sphere {
	center: Point3,
	radius: f64,
}

impl Sphere {
	pub fn from_center_and_radius(center: Point3, radius: impl Into<f64>) -> Self {
		Sphere {
			center,
			radius: radius.into()
		}
	}
}

impl Hitable for Sphere {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
		// using quadratic formula to calculate intersections
		let oc = r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = oc.dot_product(&r.direction());
		let c = oc.length_squared() - self.radius * self.radius;
	
		let discriminant = half_b*half_b - a*c;

		if discriminant < 0.0 { return false;}

		// calculates the sqrt once, and saves the value
		let discriminant_sqrt = discriminant.sqrt();

		// detect the nearest hit:
		let hit = (-half_b - discriminant_sqrt) / a;

		if hit < t_min || t_max < hit {
			// adding the sqrt is always closer, since the camera is in the positive z-direction compared to the object
			let hit = (-half_b + discriminant_sqrt) / a;
			if hit < t_min || t_max < hit {
				return false;
			}
		}

		record.mut_t(hit);
		record.mut_point(r.at(hit));
		let outward_normal = (record.point() - self.center) / self.radius;
		record.set_face_normal(r, outward_normal);

		return true;
	}
}