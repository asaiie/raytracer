mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use color::{write_color, Color};
use ray::Ray;
use vec3::Point3;
use vec3::Vec3;

use std::f64;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::sphere::Sphere;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(hit_rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
        return 0.5 * (hit_rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0; // ideal ratio
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = image_height.max(1);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport vectors across horizontal and down vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Pixel delta vectors
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    let viewport_upper_left: Point3 =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2; // topleft corner of viewport in 3D coord space
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let file = File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        std::io::stdout().flush()?;

        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&mut out, &pixel_color)?;
        }
    }

    println!("\rDone.                 ");
    Ok(())
}
