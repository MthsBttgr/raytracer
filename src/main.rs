use std::{fs, io::Write};
use std::io::BufWriter;

mod point3;
mod ray;
mod hitable;
mod sphere;
mod camera;

use hitable::{Hitable, HitRecord, HitableList};
use point3::{Color, Point3, Vec3};
use ray::Ray;
use sphere::Sphere;
use rand::{random, Rng};
use camera::Camera;

const INFINITY: f64 = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

fn main() {
    let mut file = BufWriter::new(fs::File::create("Images/testAntialiasing.ppm").unwrap());
    let mut stderr = std::io::stderr();

    let mut rng = rand::thread_rng();

    //image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as i32;
    let samples_pr_pixel = 100;

    //world
    let mut world = HitableList::default();
    world.add(Box::new(Sphere::from_center_and_radius(Point3::from_xyz(0,0,-1), 0.5)));
    world.add(Box::new(Sphere::from_center_and_radius(Point3::from_xyz(0,-100.5,-1), 100)));
    
    //3D camera
    let camera = Camera::default();

    //render
    file.write_all(format!("P3\n{} {}\n255\n", img_width, img_height).as_bytes()).expect("couldnt write header");

    let mut counter: i64 = 0;
    for y in (0 .. img_height).rev() {
        // prints how many coloumns of pixels remain
        stderr.write(format!("Scanlines remaining: {}\n", y)
            .as_bytes())
            .expect("cant write to stderr");
        stderr.flush().expect("couldnt flush stderr");
        
        for x in 0..img_width {
            let mut pixel_color = Color::new();

            for s in 0..samples_pr_pixel {
                let u = (x as f64 + rng.gen::<f64>()) / ((img_width - 1) as f64);
                let v = (y as f64 + rng.gen::<f64>()) / ((img_height - 1) as f64);
    
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world);
            }

            file.write_all(format!("\n{}", pixel_color.write_color(samples_pr_pixel as f64)).as_bytes())
                .expect("couldnt write all");

            counter += 1;
        }
    }

    println!("for loop ran: {} times", counter);
}


///function for making a quick color for the rays
fn ray_color(r: &Ray, world: &dyn Hitable) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (Into::<Color>::into(rec.normal()) + Color::from_rgb(1.0, 1.0, 1.0)) * 0.5;
    }
    
    /*
    let t = sphere_is_hit(&Point3::from_xyz(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Point3::from_xyz(0.0, 0.0, -1.0)).unit_vec();
        return Color::from_rgb((n.x() + 1.0) * 0.5, (n.y() +1.0) * 0.5, (n.z() + 1.0) * 0.5);
    }
    */

    let unit_vec = r.direction().unit_vec();
    let t = 0.5 * (unit_vec.y() + 1.0);

    if t < -1.0 || t > 1.0 {
        println!("t is fucked");
        panic!();
    }

    return Color::from_rgb(1,1, 1) * (1.0-t) + Color::from_rgb(0.5, 0.7, 1.0) * t;
}

/// function checks if the ray hits a given sphere
fn sphere_is_hit (sphere_center: &Point3, radius: f64, ray: &Ray) -> f64 {
    // using quadratic formula to calculate intersections
    let oc = ray.origin() - *sphere_center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot_product(&ray.direction());
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    } 

    (-half_b - discriminant.sqrt()) / a
}

#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}