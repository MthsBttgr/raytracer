use crate::{Point3, Vec3, Ray};


#[derive(Default, Clone, Copy)]
/// Struct for keeping track info regarding ray intersections with objects
pub struct HitRecord {
	point: Point3,
	normal: Vec3,
	
	t: f64,
	front_face: bool,
} 

impl HitRecord {
	/// Sets internal value of "t" to the given value
	pub fn mut_t (&mut self, t: f64) {
		self.t = t;
	}

	/// Sets internal value of "point" to the given value
	pub fn mut_point(&mut self, point: Point3) {
		self.point = point;
	}

	/// Sets internal value of "normal" to the given value
	pub fn mut_normal(&mut self, normal: Vec3) {
		self.normal = normal;
	}

	pub fn point(&self) -> Point3 {
		self.point
	}

	pub fn normal(&self) -> Vec3 {
		self.normal
	}

	pub fn t(&self) -> f64 {
		self.t
	}

	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
		self.front_face = r.direction().dot_product(&outward_normal) < 0.0;
		self.normal = if self.front_face {outward_normal} else {-outward_normal};
	}
}

#[derive(Default)]
pub struct HitableList {
	list: Vec<Box<dyn Hitable>>
}

impl HitableList {
	pub fn add(&mut self, object: Box<dyn Hitable>) {
		self.list.push(object);
	}

	pub fn clear(&mut self) {
		self.list.clear();
	}
}

impl Hitable for HitableList {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::default();
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for object in self.list.iter() {
			if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t();
				*record = temp_rec;
			}

		}

		hit_anything
	}
}

pub trait Hitable{
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}