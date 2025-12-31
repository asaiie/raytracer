mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Material::Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world)?;
    Ok(())
}
