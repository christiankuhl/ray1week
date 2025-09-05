use crate::{colour::Colour, objects::HitRecord, random::random_unit_vector, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Colour,
}

pub trait Scatter: std::fmt::Debug {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let ray = Ray::new(hit.p, scatter_direction);
        Some(ScatterRecord {
            ray,
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = ray.direction - 2.0 * ray.direction.dot(&hit.normal) * hit.normal;
        let scatter_direction = scatter_direction.normalize() + self.fuzz * random_unit_vector();
        let ray = Ray::new(hit.p, scatter_direction);
        Some(ScatterRecord {
            ray,
            attenuation: self.albedo,
        })
    }
}
#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Colour::WHITE;
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let uv = ray.direction.normalize();
        let cos_theta = (-uv).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let dir = if cannot_refract || reflectance(cos_theta, ri) > fastrand::f64() {
            uv - 2.0 * uv.dot(&hit.normal) * hit.normal
        } else {
            refract(uv, hit.normal, ri)
        };
        let ray = Ray::new(hit.p, dir);
        Some(ScatterRecord { ray, attenuation })
    }
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(&n).min(1.0);
    let r_out_perp: Vec3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.dot(&r_out_perp)).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
