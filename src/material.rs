use std::{f64::consts::PI, sync::Arc};

use crate::{
    colour::Colour,
    objects::HitRecord,
    random::{CosinePDF, DirectionalPDF, UniformSphericalPDF, random_unit_vector},
    ray::Ray,
    texture::{SolidColour, Texture},
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct ScatterRecord {
    pub attenuation: Colour,
    pub scattered: ScatterResult,
}

#[derive(Clone)]
pub enum ScatterResult {
    PDF(Arc<dyn DirectionalPDF>),
    SpecularRay(Ray),
}

pub trait Scatter: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<ScatterRecord>;
    fn emit(&self, hit: &HitRecord, u: f64, v: f64, p: Point3) -> Colour;
    fn scattering_pdf(&self, ray: Ray, hit: &HitRecord, scattered: Ray) -> f64;
    fn is_emissive(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self {
            texture: Arc::new(SolidColour::new(albedo)),
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(hit.u, hit.v, hit.p),
            scattered: ScatterResult::PDF(Arc::new(CosinePDF::new(&hit.normal))),
        })
    }

    fn emit(&self, _hit: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Colour {
        Colour::BLACK
    }

    fn scattering_pdf(&self, _ray: Ray, hit: &HitRecord, scattered: Ray) -> f64 {
        let cos_theta = hit.normal.dot(&scattered.direction.normalize());
        cos_theta.max(0.0) / PI
    }

    fn is_emissive(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
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
        let scattered = Ray::time_dependent(hit.p, scatter_direction, ray.time);
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: ScatterResult::SpecularRay(scattered),
        })
    }

    fn emit(&self, _hit: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Colour {
        Colour::BLACK
    }

    fn scattering_pdf(&self, _ray: Ray, _hit: &HitRecord, _scattered: Ray) -> f64 {
        panic!("Asked for a scattering PDF on a specular material!")
    }

    fn is_emissive(&self) -> bool {
        false
    }
}
#[derive(Debug, Clone, Copy)]
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
        let scattered = Ray::time_dependent(hit.p, dir, ray.time);
        Some(ScatterRecord {
            attenuation,
            scattered: ScatterResult::SpecularRay(scattered),
        })
    }

    fn emit(&self, _hit: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Colour {
        Colour::BLACK
    }

    fn scattering_pdf(&self, _ray: Ray, _hit: &HitRecord, _scattered: Ray) -> f64 {
        panic!("Asked for scattering PDF on a specular material!")
    }

    fn is_emissive(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_colour(albedo: Colour) -> Self {
        Self {
            texture: Arc::new(SolidColour::new(albedo)),
        }
    }
}

impl Scatter for DiffuseLight {
    fn scatter(&self, _ray: Ray, _hit: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emit(&self, hit: &HitRecord, u: f64, v: f64, p: Point3) -> Colour {
        if !hit.front_face {
            return Colour::BLACK;
        }
        self.texture.value(u, v, p)
    }

    fn scattering_pdf(&self, _ray: Ray, _hit: &HitRecord, _scattered: Ray) -> f64 {
        panic!("Asked for a scattering PDF on a purely emissive material!")
    }

    fn is_emissive(&self) -> bool {
        true
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

#[derive(Debug)]
pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Scatter for Isotropic {
    fn scatter(&self, _ray: Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(hit.u, hit.v, hit.p),
            scattered: ScatterResult::PDF(Arc::new(UniformSphericalPDF)),
        })
    }

    fn emit(&self, _hit: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Colour {
        Colour::BLACK
    }

    fn scattering_pdf(&self, _ray: Ray, _hit: &HitRecord, _scattered: Ray) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn is_emissive(&self) -> bool {
        false
    }
}
