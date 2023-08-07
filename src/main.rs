use std::io::BufWriter;
use std::{fs, io::Write};

mod camera;
mod hitable;
mod material;
mod point3;
mod ray;
mod sphere;

use camera::Camera;
use hitable::{Hitable, HitableList};
use point3::{Color, Point3, Vec3};
use ray::Ray;
use sphere::Sphere;

use crate::material::{Dielectric, Lambertian, Metal};

fn main() {
    let mut file = BufWriter::new(
        fs::File::create("Images/renderWithHollowGlassBall2.ppm").expect("couldn't create file"),
    );

    //world
    let mut world = HitableList::default();

    let ground_material = Lambertian::from_color(Color::from_rgb(0.8, 0.8, 0.0));
    let center_material = Lambertian::from_color(Color::from_rgb(0.1, 0.2, 0.5));
    let left_material = Dielectric::from_ir(1.5);
    let right_material = Metal::from_color(Color::from_rgb(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(0, -100.5, -1),
        100,
        ground_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(0, 0, -1.5),
        0.5,
        center_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(-1.0, 0, -1.5),
        0.5,
        left_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(-1.0, 0, -1.5),
        -0.4,
        left_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(1.0, 0, -1.5),
        0.5,
        right_material,
    )));

    //3D camera
    let mut camera = Camera::default();
    camera.set_img_dimensions(16.0 / 9.0, 1200);
    camera.render(&world, &mut file);
}
