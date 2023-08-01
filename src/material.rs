use crate::{
    hitable::HitRecord,
    point3::{Color, Vec3},
    ray::Ray,
};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

fn reflect(v_in: &Vec3, normal: &Vec3) -> Vec3 {
    *v_in - *normal * v_in.dot_product(normal) * 2.0
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new() -> Self {
        Self {
            albedo: Color::new(),
        }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal() + Vec3::random_unit_vec();

        // makes sure the scatter direction doesnt potentially mess stuff up
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        Some((Ray::new(rec.point(), scatter_direction), self.albedo))
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new() -> Self {
        Self {
            albedo: Color::new(),
            fuzz: 0.0,
        }
    }

    pub fn from_color(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&r.direction().unit_vec(), &rec.normal());

        if reflected.dot_product(&rec.normal()) > 0.0 {
            Some((
                Ray::new(
                    rec.point(),
                    reflected + Vec3::random_vec_in_unit_sphere() * self.fuzz,
                ),
                self.albedo,
            ))
        } else {
            None
        }
    }
}
