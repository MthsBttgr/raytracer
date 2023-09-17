use crate::{material::Material, Point3, Ray, Vec3};

pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Struct for keeping track info regarding ray intersections with objects
pub struct HitRecord<'a> {
    point: Point3,              //point of the intersection
    normal: Vec3,               //The normal of the point
    t: f64,                     // Distance from the camera to the point
    material: &'a dyn Material, // The material of the object that was hit
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

    /// returns a refrence to the distance between intersection and camera
    pub fn t(&self) -> f64 {
        self.t
    }
}

/// Impl Hitable for list of hitable objects
impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in self.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t();
                temp_rec = Some(hit);
            }
        }

        temp_rec
    }
}
