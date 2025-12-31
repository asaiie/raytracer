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
    utils,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,      // ratio of image width over height
    pub image_width: i32,       // rendered image width in pixel count
    pub samples_per_pixel: i32, // count of random samples for each pixel
    pub max_depth: i32,         // maximum number of ray bounces into scene

    pub vfov: f64,        // vertical view angle (field of view)
    pub lookfrom: Point3, // point camera is looking from
    pub lookat: Point3,   // point camera is looking at
    pub vup: Vec3,        // camera-relative "up" direction

    pub defocus_angle: f64, // variation angle of rays through each pixel
    pub focus_dist: f64,    // distance from camera lookfrom point to plane of perfect focus

    image_height: i32,        // rendered image height
    pixel_samples_scale: f64, // color scale factor for a sum of pixel samples
    center: Point3,           // camera center
    pixel00_loc: Point3,      // location of pixel 0, 0
    pixel_delta_u: Vec3,      // offset to pixel to the right
    pixel_delta_v: Vec3,      // offset to pixel below
    u: Vec3,                  // camera frame basis vectors
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // defocus disk horizontal radius
    defocus_disk_v: Vec3, // defocus disk vertical radius
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = utils::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Viewport vectors across horizontal and down vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Pixel delta vectors
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        let viewport_upper_left: Point3 =
            self.center - (self.focus_dist * self.w) - viewport_u / 2 - viewport_v / 2; // topleft corner of viewport in 3D coord space
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate camera defocus disk basis vectors
        let defocus_radius =
            self.focus_dist * utils::degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = hit_rec.mat.scatter(r, &hit_rec) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle < 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
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
