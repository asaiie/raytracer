use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
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
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
