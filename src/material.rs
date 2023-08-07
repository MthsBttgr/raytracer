use rand::{thread_rng, Rng};

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

#[derive(Clone, Copy)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    /// Returns a glasslike material from an index of refraction
    pub fn from_ir(index_of_refraction: f64) -> Self {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::from_rgb(1, 1, 1);

        // calculates different values depending on whether or not the ray hits a frontface or a backface
        let (normal, refraction_ratio) = if r.direction().dot_product(&rec.normal()) > 0.0 {
            (-rec.normal(), self.index_of_refraction)
        } else {
            (rec.normal(), 1.0 / self.index_of_refraction)
        };

        let scattered = if let Some(refracted) = refract(&r, &normal, refraction_ratio) {
            Ray::new(rec.point(), refracted)
        } else {
            let reflected = reflect(&r.direction(), &rec.normal());
            Ray::new(rec.point(), reflected)
        };

        Some((scattered, attenuation))
    }
}

fn refract(r: &Ray, n: &Vec3, refraction_ratio: f64) -> Option<Vec3> {
    let cos_theta = n.dot_product(&-r.direction().unit_vec()).min(1.0);
    let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

    let uv = r.direction().unit_vec();

    // checks if the ball can refract, and returns None if it cant
    if sin_theta * refraction_ratio > 1.0 || reflectance(cos_theta, refraction_ratio) {
        return None;
    } else {
        let r_out_perp = (uv + *n * cos_theta) * refraction_ratio;
        let r_out_par = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        return Some(r_out_perp + r_out_par);
    }
}

/// uses Schlicks approximation to figure out whether or not an object should reflect
fn reflectance(cosine: f64, refraction_ratio: f64) -> bool {
    let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    r0 = r0.powi(2);
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5) > thread_rng().gen();
}
