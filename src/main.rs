use std::io::BufWriter;
use std::{fs, sync::Arc};

mod camera;
mod hitable;
mod material;
mod point3;
mod ray;
mod sphere;

use camera::Camera;
use hitable::HitableList;
use material::Materials;
use point3::{Color, Point3, Vec3};
use ray::Ray;
use sphere::Sphere;

use crate::material::{Dielectric, Lambertian, Metal};

fn main() {
    let mut file =
        BufWriter::new(fs::File::create("Images/finalRender.ppm").expect("couldn't create file"));
    let mut file2 = BufWriter::new(
        fs::File::create("Images/finalRenderPar3.ppm").expect("couldn't create file"),
    );

    //world
    let mut world = HitableList::default();

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::from_xyz(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            let material = rand::random::<Materials>();

            if (center - Point3::from_xyz(4, 0.2, 9)).length() > 0.9 {
                match material {
                    Materials::Rough(mat) => world.add(Box::new(
                        Sphere::from_center_radius_material(center, 0.3, mat),
                    )),
                    Materials::Reflective(mat) => world.add(Box::new(
                        Sphere::from_center_radius_material(center, 0.3, mat),
                    )),
                    Materials::Glass(mat) => {
                        world.add(Box::new(Sphere::from_center_radius_material(
                            center, 0.3, mat,
                        )));
                        if rand::random::<f64>() >= 0.5 {
                            world.add(Box::new(Sphere::from_center_radius_material(
                                center, -0.2, mat,
                            )));
                        }
                    }
                }
            }
        }
    }

    let ground_material = Lambertian::from_color(Color::from_rgb(0.5, 0.5, 0.5));
    let center_material = Lambertian::from_color(Color::from_rgb(0.1, 0.2, 0.5));
    let left_material = Dielectric::from_ir(1.5);
    let right_material = Metal::from_color(Color::from_rgb(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(-4, 1, 0),
        1,
        center_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(0, 1, 0),
        1,
        left_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(0, 1, 0),
        -0.7,
        left_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(4, 1, 0),
        1,
        right_material,
    )));
    world.add(Box::new(Sphere::from_center_radius_material(
        Point3::from_xyz(0, -1000, -1),
        1000,
        ground_material,
    )));

    //3D camera
    let mut camera = Camera::default();
    camera.set_img_dimensions(16.0 / 9.0, 1200);
    camera.set_camera_settings(
        Point3::from_xyz(13, 2, 3),
        Point3::from_xyz(0, 0, 0),
        20.0,
        10,
        10,
        0.6,
        10.0,
    );
    // camera.render(&world, &mut file);

    println!("Starting parralel render");
    camera.par_render(&Arc::new(world), &mut file2);
}
