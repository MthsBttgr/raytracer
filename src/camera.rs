use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use std::{
    f64::{consts::PI, INFINITY},
    fs::File,
    io::{BufWriter, Write},
    sync::{Arc, Mutex},
};

use crate::{
    hitable::Hitable,
    point3::{Color, Point3, Vec3},
    ray::Ray,
};

/// The virtual camera
pub struct Camera {
    samples_pr_pixel: i64,  // The amount of rays sent out pr pixel
    max_light_bounces: i32, // The max amount of ray bounces in the scene

    vfov: f64,         // the vertical field of view (stored in radians)
    vup: Vec3,         // Camera-relative up direction
    look_from: Point3, // Where the camera is looking from
    look_at: Point3,   // where the camera is looking at

    // Vectors that define camera dimensions
    u: Vec3,
    v: Vec3,
    w: Vec3,

    pixel_delta_u: Vec3, // The distance between horizontal pixels
    pixel_delta_v: Vec3, // THe distance betweem vertical pixels
    pixel_00_loc: Vec3,  // the location of the top left pixel of the camera

    img_width: i64,    // The width of the image in pixels
    img_height: i64,   // The height of the image in pixels
    aspect_ratio: f64, // The ratio between the height and width of the image

    focus_distance: f64,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {}

impl Default for Camera {
    fn default() -> Self {
        // Camera positioning
        let look_from = Point3::from_xyz(0, 0, 1);
        let look_at = Point3::from_xyz(0, 0, 0);
        let vup = Vec3::from_xyz(0, 1, 0); // Camera - relative up direction

        // Camera basic vectors
        let w = (look_from - look_at).unit_vec();
        let u = vup.cross_product(&w).unit_vec();
        let v = w.cross_product(&u);

        let focus_distance = 10.0;
        let defocus_angle = 0.0;

        let defocus_radius = focus_distance * Camera::degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        // image dimensions
        let aspect_ratio = 16.0 / 9.0;
        let img_width = 400;
        let img_height = (img_width as f64 / aspect_ratio) as i64;

        // Viewport dimensions:
        let vfov = 90.0;
        let theta = Camera::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;
        let pixel_delta_u = viewport_u / img_width as f64;
        let pixel_delta_v = viewport_v / img_height as f64;
        let upper_left = look_from - (w * focus_distance) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = upper_left + (pixel_delta_v + pixel_delta_u) * 0.5;

        // Render settings
        let samples_pr_pixel = 50;
        let max_depth = 50;

        Self {
            samples_pr_pixel,
            max_light_bounces: max_depth,
            img_width,
            img_height,
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
            focus_distance,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

impl Camera {
    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Point3 {
        let p = Point3::random_in_unit_circle(rng);

        return self.look_from + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y());
    }

    fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn get_ray(&self, x: f64, y: f64, rng: &mut ThreadRng) -> Ray {
        let pixel_center = self.pixel_00_loc
            + self.pixel_delta_u * (x + rng.gen::<f64>())
            + self.pixel_delta_v * (y + rng.gen::<f64>());

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from
        } else {
            self.defocus_disk_sample(rng)
        };

        Ray::new(ray_origin, pixel_center - ray_origin)
    }

    ///function for making a quick color for the rays
    pub fn par_ray_color<T: Hitable>(r: &Ray, world: Arc<T>, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new();
        }

        // let world = **world.lock().unwrap();
        if let Some(rec) = world.hit(r, 0.001, INFINITY) {
            if let Some((scattered, attenuation)) = rec.material().scatter(r, &rec) {
                return attenuation * Camera::par_ray_color(&scattered, world, depth - 1);
            }

            return Color::new();
        }

        let unit_vec = r.direction().unit_vec();
        let t = 0.5 * (unit_vec.y() + 1.0);

        return Color::from_rgb(1, 1, 1) * (1.0 - t) + Color::from_rgb(0.5, 0.7, 1.0) * t;
    }

    ///Render the image in parallel with threads
    pub fn render_with_threads<'a, T: Hitable + 'static>(
        &'static self,
        world: Arc<T>,
        file: &'a mut BufWriter<File>,
    ) {
        // First write the header for the image file
        file.write_all(format!("P3\n{} {}\n255\n", self.img_width, self.img_height).as_bytes())
            .expect("couldnt write header");

        // find out how many threads are available
        let available_threads = std::thread::available_parallelism()
            .expect("Couldnt get number of available cores for parallelism")
            .get();

        // the amount of pixelrows each thread is supposed to render
        let chunk_size = self.img_height / available_threads as i64;

        // Vector to store the pixels in
        let pixel_results: Arc<Mutex<Vec<Option<Vec<_>>>>> =
            Arc::new(Mutex::new(vec![None; available_threads]));

        let new_self = Arc::new(self.clone());

        // spawning threads and rendering
        let handles: Vec<_> = (0..available_threads)
            .into_iter()
            .map(|i| {
                let start = chunk_size * i as i64;
                let end = if i - 1 != available_threads {
                    chunk_size * (i + 1) as i64
                } else {
                    self.img_height
                };

                let arc_self = new_self.clone();
                let arc_world = world.clone();
                let pixels_clone = pixel_results.clone();

                std::thread::spawn(move || {
                    let mut rng = rand::thread_rng();

                    let mut pixels = Vec::new();
                    for y in start..end {
                        for x in 0..arc_self.img_width {
                            let mut pixel_color = Color::new();

                            for _s in 0..arc_self.samples_pr_pixel {
                                let r = arc_self.get_ray(x as f64, y as f64, &mut rng);
                                pixel_color = pixel_color
                                    + Camera::par_ray_color(
                                        &r,
                                        arc_world.to_owned(),
                                        arc_self.max_light_bounces,
                                    );
                            }
                            pixels.push(pixel_color);
                        }
                    }

                    pixels_clone.lock().expect("couldnt lock pixel_results")[i] = Some(pixels);
                })
            })
            .collect();

        for (_, handle) in handles.into_iter().enumerate() {
            handle.join().expect("cant join thread");
        }

        let mut pixels = Vec::new();
        for i in pixel_results
            .lock()
            .unwrap()
            // .into_inner()
            // .expect("couldnt get inner value of mutex")
            .clone()
        // .iter()
        {
            pixels.push(i);
        }
        let pixels: Vec<Color> = pixels.into_iter().flat_map(|i| i.unwrap()).collect();

        // write to file
        for col in pixels {
            file.write_all(
                format!("\n{}", col.write_color(self.samples_pr_pixel as f64)).as_bytes(),
            )
            .expect("couldnt write all");
        }
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

    /// Render the image without parallelisation
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
                .write(format!("\x1b[2K \x1b[0G").as_bytes())
                .expect("couldnt clear terminal");
            stderr
                .write(format!("Scanlines remaining: {}", self.img_height - y).as_bytes())
                .expect("cant write to stderr");
            stderr.flush().expect("couldnt flush stderr");

            for x in 0..self.img_width {
                let mut pixel_color = Color::new();

                for _s in 0..self.samples_pr_pixel {
                    let r = self.get_ray(x as f64, y as f64, &mut rng);
                    pixel_color =
                        pixel_color + Camera::ray_color(&r, world, self.max_light_bounces);

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

        println!("\nFor loop ran: {} times", counter);
    }

    /// Set camera settings that aren't the default values
    // Should probably have use a builder pattern or something, but this was just faster...
    pub fn set_camera_settings(
        &mut self,
        camera_placement: Point3,
        look_at: Point3,
        vfov: f64,
        max_light_bounces: i32,
        samples: i64,
        defocus_angle: f64,
        focus_distance: f64,
    ) {
        self.look_from = camera_placement;
        self.look_at = look_at;
        self.vfov = vfov;
        self.samples_pr_pixel = samples;
        self.max_light_bounces = max_light_bounces;
        self.focus_distance = focus_distance;
        self.defocus_angle = defocus_angle;

        // Camera basic vectors
        self.w = (self.look_from - self.look_at).unit_vec();
        self.u = self.vup.cross_product(&self.w).unit_vec();
        self.v = self.w.cross_product(&self.u);

        let defocus_radius =
            self.focus_distance * Camera::degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        // Viewport dimensions:
        let h = (Camera::degrees_to_radians(self.vfov) / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * self.aspect_ratio;

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;
        self.pixel_delta_u = viewport_u / self.img_width as f64;
        self.pixel_delta_v = viewport_v / self.img_height as f64;
        let upper_left =
            self.look_from - (self.w * self.focus_distance) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_00_loc = upper_left + (self.pixel_delta_v + self.pixel_delta_u) * 0.5;
    }

    /// Sets the dimensions of the final image based on aspect ratio and width measured in pixels.
    /// image height is calculated based on these two components
    pub fn set_img_dimensions(&mut self, aspect_ratio: f64, img_width: i64) {
        self.aspect_ratio = aspect_ratio;
        self.img_width = img_width;
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as i64;
    }
}
