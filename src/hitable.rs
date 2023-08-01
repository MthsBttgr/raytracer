use crate::{material::Material, Point3, Ray, Vec3};

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

//#[derive(Default, Clone)]
/// Struct for keeping track info regarding ray intersections with objects
pub struct HitRecord<'a> {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// creates a new hitrecord. Takes a Point, a value t (the scalar for ray that hit the point), and the material
    /// two other values, front_face and normal, still need to be set, with the function set_face_normal()
    pub fn new(point: Point3, t: f64, material: &'a dyn Material) -> Self {
        Self {
            point,
            normal: Point3::new(),
            t,
            front_face: false,
            material,
        }
    }

    pub fn material(&self) -> &dyn Material {
        self.material
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
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in self.list.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t();
                temp_rec = Some(hit);
            }
        }

        temp_rec
    }
}
