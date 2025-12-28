use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use rand::Rng;

use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,      // ratio of image width over height
    pub image_width: i32,       // rendered image width in pixel count
    pub samples_per_pixel: i32, // count of random samples for each pixel
    pub max_depth: i32,         // maximum number of ray bounces into scene

    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Viewport vectors across horizontal and down vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Pixel delta vectors
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        let viewport_upper_left: Point3 =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2; // topleft corner of viewport in 3D coord space
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            // let direction = Vec3::random_on_hemisphere(&hit_rec.normal);
            let direction = hit_rec.normal + Vec3::random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(hit_rec.p, direction), depth - 1, world);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    pub fn render(&mut self, world: &HittableList) -> io::Result<()> {
        self.initialize();

        let file = File::create("image.ppm")?;
        let mut out = BufWriter::new(file);

        writeln!(out, "P3")?;
        writeln!(out, "{} {}", self.image_width, self.image_height)?;
        writeln!(out, "255")?;

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            std::io::stdout().flush()?;

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&mut out, &(self.pixel_samples_scale * pixel_color))?;
            }
        }

        println!("\rDone.                 ");
        Ok(())
    }
}
