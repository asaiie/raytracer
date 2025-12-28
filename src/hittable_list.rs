use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = hit_rec.t;
                closest_hit = Some(hit_rec);
            }
        }

        closest_hit
    }
}
