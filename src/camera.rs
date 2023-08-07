use std::{
    f64::INFINITY,
    fs::File,
    io::{BufWriter, Write},
};

use rand::Rng;

use crate::{
    hitable::Hitable,
    point3::{Color, Point3, Vec3},
    ray::Ray,
};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,

    samples_pr_pixel: i64,
    max_depth: i32,

    img_width: i64,
    img_height: i64,
    aspect_ratio: f64,
    focal_length: f64,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let samples_pr_pixel = 50;
        let max_depth = 50;
        let img_width = 400;
        let img_height = (img_width as f64 / aspect_ratio) as i64;

        let origin = Point3::new();
        let horizontal = Vec3::from_xyz(viewport_width, 0.0, 0.0);
        let vertical = Vec3::from_xyz(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from_xyz(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            samples_pr_pixel,
            max_depth,
            img_width,
            img_height,
            focal_length,
            aspect_ratio,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }

    pub fn render<T: Hitable>(&self, world: &T, file: &mut BufWriter<File>) {
        let mut stderr = std::io::stderr();

        let mut rng = rand::thread_rng();

        //render
        file.write_all(format!("P3\n{} {}\n255\n", self.img_width, self.img_height).as_bytes())
            .expect("couldnt write header");

        let mut counter: i64 = 0;
        for y in (0..self.img_height).rev() {
            // prints how many coloumns of pixels remain
            stderr
                .write(format!("Scanlines remaining: {}\n", y).as_bytes())
                .expect("cant write to stderr");
            stderr.flush().expect("couldnt flush stderr");

            for x in 0..self.img_width {
                let mut pixel_color = Color::new();

                for _s in 0..self.samples_pr_pixel {
                    let u = (x as f64 + rng.gen_range(-1.0..1.0)) / ((self.img_width - 1) as f64);
                    let v = (y as f64 + rng.gen_range(-1.0..1.0)) / ((self.img_height - 1) as f64);

                    let r = self.get_ray(u, v);
                    pixel_color = pixel_color + Camera::ray_color(&r, world, self.max_depth);

                    counter += 1;
                }

                file.write_all(
                    format!(
                        "\n{}",
                        pixel_color.write_color(self.samples_pr_pixel as f64)
                    )
                    .as_bytes(),
                )
                .expect("couldnt write all");
            }
        }

        println!("for loop ran: {} times", counter);
    }

    ///function for making a quick color for the rays
    pub fn ray_color(r: &Ray, world: &dyn Hitable, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new();
        }

        if let Some(rec) = world.hit(r, 0.00001, INFINITY) {
            if let Some((scattered, attenuation)) = rec.material().scatter(r, &rec) {
                return attenuation * Camera::ray_color(&scattered, world, depth - 1);
            }

            return Color::new();
        }

        let unit_vec = r.direction().unit_vec();
        let t = 0.5 * (unit_vec.y() + 1.0);

        return Color::from_rgb(1, 1, 1) * (1.0 - t) + Color::from_rgb(0.5, 0.7, 1.0) * t;
    }

    /// Sets the dimensions of the final image based on aspect ratio and width measured in pixels.
    /// image height is calculated based on these two components
    pub fn set_img_dimensions(&mut self, aspect_ratio: f64, img_width: i64) {
        self.aspect_ratio = aspect_ratio;
        self.img_width = img_width;
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as i64;
    }
}
