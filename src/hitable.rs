use crate::{material::Material, Point3, Ray, Vec3};

pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

//#[derive(Default, Clone)]
/// Struct for keeping track info regarding ray intersections with objects
pub struct HitRecord<'a> {
    point: Point3,
    normal: Vec3,
    t: f64,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// creates a new hitrecord. Takes a Point, a value t (the scalar for ray that hit the point), and the material
    /// two other values, front_face and normal, still need to be set, with the function set_face_normal()
    pub fn new(point: Point3, t: f64, normal: Vec3, material: &'a dyn Material) -> Self {
        Self {
            point,
            t,
            normal,
            material,
        }
    }

    /// returns a refrence to the objects material
    pub fn material(&self) -> &dyn Material {
        self.material
    }

    /// returns a refrence to the point of object - ray intersection
    pub fn point(&self) -> Point3 {
        self.point
    }

    /// returns a refrence to normal
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// returns a refrence to the parameter used in the linear function to calculate intersections
    pub fn t(&self) -> f64 {
        self.t
    }
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    /// Add an object to the scene to be rendered
    pub fn add(&mut self, object: Box<dyn Hitable>) {
        self.list.push(object);
    }

    /// clear the list of objects to be rendered
    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Hitable for HitableList {
    /// Checks if the given ray hits any objects in the scene and returns the record of said hit
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
