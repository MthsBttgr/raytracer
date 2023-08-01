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
use rand::Rng;
use ray::Ray;
use sphere::Sphere;

use crate::material::{Lambertian, Metal};

const INFINITY: f64 = f64::INFINITY;
//const PI: f64 = std::f64::consts::PI;

fn main() {
    let mut file = BufWriter::new(
        fs::File::create("Images/renderWithMaterialsAndFuzz.ppm").expect("couldn't create file"),
    );
    let mut stderr = std::io::stderr();

    let mut rng = rand::thread_rng();

    //image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 800;
    let img_height = (img_width as f64 / aspect_ratio) as i32;
    let samples_pr_pixel = 50;
    let max_depth = 50;

    //world
    let mut world = HitableList::default();

    let ground_material = Lambertian::from_color(Color::from_rgb(0.8, 0.8, 0.0));
    let center_material = Lambertian::from_color(Color::from_rgb(0.7, 0.3, 0.3));
    let left_material = Metal::from_color(Color::from_rgb(0.8, 0.8, 0.8), 0.3);
    let right_material = Metal::from_color(Color::from_rgb(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(0, -100.5, -1),
        100,
        ground_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(0, 0, -1.0),
        0.5,
        center_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(-1.0, 0, -1.0),
        0.5,
        left_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(1.0, 0, -1.0),
        0.5,
        right_material,
    )));

    //3D camera
    let camera = Camera::default();

    //render
    file.write_all(format!("P3\n{} {}\n255\n", img_width, img_height).as_bytes())
        .expect("couldnt write header");

    let mut counter: i64 = 0;
    for y in (0..img_height).rev() {
        // prints how many coloumns of pixels remain
        stderr
            .write(format!("Scanlines remaining: {}\n", y).as_bytes())
            .expect("cant write to stderr");
        stderr.flush().expect("couldnt flush stderr");

        for x in 0..img_width {
            let mut pixel_color = Color::new();

            for _s in 0..samples_pr_pixel {
                let u = (x as f64 + rng.gen_range(-1.0..1.0)) / ((img_width - 1) as f64);
                let v = (y as f64 + rng.gen_range(-1.0..1.0)) / ((img_height - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);

                counter += 1;
            }

            file.write_all(
                format!("\n{}", pixel_color.write_color(samples_pr_pixel as f64)).as_bytes(),
            )
            .expect("couldnt write all");
        }
    }

    println!("for loop ran: {} times", counter);
}

///function for making a quick color for the rays
fn ray_color(r: &Ray, world: &dyn Hitable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new();
    }

    if let Some(rec) = world.hit(r, 0.00001, INFINITY) {
        if let Some((scattered, attenuation)) = rec.material().scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        //return Color::from_rgb(1,1, 1) * (1.0-v) + Color::from_rgb(0.5, 0.7, 1.0) * v;
        return Color::new();
    }

    let unit_vec = r.direction().unit_vec();
    let t = 0.5 * (unit_vec.y() + 1.0);

    if t < -1.0 || t > 1.0 {
        println!("t is fucked");
        panic!();
    }

    return Color::from_rgb(1, 1, 1) * (1.0 - t) + Color::from_rgb(0.5, 0.7, 1.0) * t;
}

/*#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
} */
