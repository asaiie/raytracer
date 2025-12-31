use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    // Returns None if ray absorbed, otherwise returns Some((attenuation, scattered))
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match *self {
            Material::Lambertian { albedo } => Self::scatter_lambertian(albedo, rec),
            Material::Metal { albedo, fuzz } => Self::scatter_metal(albedo, fuzz, r_in, rec),
        }
    }

    fn scatter_lambertian(albedo: Color, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((albedo, scattered))
    }

    fn scatter_metal(
        albedo: Color,
        fuzz: f64,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        let reflected = reflected.unit_vector() + (fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((albedo, scattered))
        } else {
            None
        }
    }
}
