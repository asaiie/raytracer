use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Material {
    // Returns None if ray absorbed, otherwise returns Some((attenuation, scattered))
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match *self {
            Material::Lambertian { albedo } => Self::scatter_lambertian(albedo, rec),
            Material::Metal { albedo, fuzz } => Self::scatter_metal(albedo, fuzz, r_in, rec),
            Material::Dielectric { refraction_index } => {
                Self::scatter_dielectric(refraction_index, r_in, rec)
            }
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

    fn scatter_dielectric(
        refraction_index: f64,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / refraction_index
        } else {
            refraction_index
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let mut rng = rand::thread_rng();

        let direction = if cannot_refract || reflectance(cos_theta, ri) > rng.gen::<f64>() {
            // cannot refract
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Schlick's approximation for reflectance
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
