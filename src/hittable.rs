use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, r: &Ray, outward_normal: &Vec3, mat: Material) -> Self {
        // NOTE: the param outward_normal is assumed to have unit length
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };

        HitRecord {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
