use std::{
    f64::{consts::PI, INFINITY},
    fs::File,
    io::{BufWriter, Write},
};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    hitable::Hitable,
    point3::{Color, Point3, Vec3},
    ray::Ray,
};

pub struct Camera {
    origin: Point3,

    samples_pr_pixel: i64,
    max_depth: i32, // The max amount of ray bounces in the scene

    vfov: f64, // the vertical field of view (stored in radians)
    vup: Vec3, // Camera-relative up direction
    look_from: Point3,
    look_at: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00_loc: Vec3,

    img_width: i64,
    img_height: i64,
    aspect_ratio: f64,
    focal_length: f64,
}

impl Default for Camera {
    fn default() -> Self {
        // Camera positioning
        let look_from = Point3::from_xyz(0, 0, -1);
        let look_at = Point3::from_xyz(0, 0, 0);
        let vup = Vec3::from_xyz(0, 1, 0); // Camera - relative up direction

        // Camera basic vectors
        let w = (look_from - look_at).unit_vec();
        let u = vup.cross_product(&w).unit_vec();
        let v = w.cross_product(&u);

        // image dimensions
        let aspect_ratio = 16.0 / 9.0;
        let img_width = 400;
        let img_height = (img_width as f64 / aspect_ratio) as i64;

        // Viewport dimensions:
        let focal_length = (look_from - look_at).length();
        let vfov = 90.0;
        let theta = Camera::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;
        let pixel_delta_u = viewport_u / img_width as f64;
        let pixel_delta_v = viewport_v / img_height as f64;
        let upper_left = look_from - (w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = upper_left + (pixel_delta_v + pixel_delta_u) * 0.5;

        // Usefull vectors
        let origin = look_from;

        // Render settings
        let samples_pr_pixel = 50;
        let max_depth = 50;

        Self {
            origin,
            samples_pr_pixel,
            max_depth,
            img_width,
            img_height,
            focal_length,
            aspect_ratio,
            vfov,
            vup,
            look_from,
            look_at,
            v,
            u,
            w,
            pixel_delta_u,
            pixel_delta_v,
            pixel_00_loc,
        }
    }
}

impl Camera {
    fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn get_ray(&self, x: f64, y: f64, rng: &mut ThreadRng) -> Ray {
        let pixel_center = self.pixel_00_loc
            + self.pixel_delta_u * (x + rng.gen::<f64>())
            + self.pixel_delta_v * (y + rng.gen::<f64>());
        Ray::new(self.origin, pixel_center - self.origin)
    }

    pub fn render<T: Hitable>(&self, world: &T, file: &mut BufWriter<File>) {
        let mut stderr = std::io::stderr();

        let mut rng = rand::thread_rng();

        //render
        file.write_all(format!("P3\n{} {}\n255\n", self.img_width, self.img_height).as_bytes())
            .expect("couldnt write header");

        let mut counter: i64 = 0;
        for y in 0..self.img_height {
            // prints how many coloumns of pixels remain
            stderr
                .write(format!("Scanlines remaining: {}\n", self.img_height - y).as_bytes())
                .expect("cant write to stderr");
            stderr.flush().expect("couldnt flush stderr");

            for x in 0..self.img_width {
                let mut pixel_color = Color::new();

                for _s in 0..self.samples_pr_pixel {
                    let r = self.get_ray(x as f64, y as f64, &mut rng);
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
