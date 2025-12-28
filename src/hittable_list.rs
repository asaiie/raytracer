use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_rec.t;
                closest_hit = Some(hit_rec);
            }
        }

        closest_hit
    }
}
