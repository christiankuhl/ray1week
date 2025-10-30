mod materials;

pub use materials::*;

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{colour::Colour, linalg::Point3, objects::HitRecord, random::DirectionalPDF, ray::Ray};

pub struct ScatterRecord {
    pub attenuation: Colour,
    pub scattered: ScatterResult,
}

pub enum ScatterResult {
    PDF(Box<dyn DirectionalPDF>),
    SpecularRay(Ray),
}

pub trait Scatter: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<ScatterRecord>;
    fn emit(&self, hit: &HitRecord, u: f64, v: f64, p: Point3) -> Colour;
    fn scattering_pdf(&self, ray: Ray, hit: &HitRecord, scattered: Ray) -> f64;
    fn is_emissive(&self) -> bool;
}

#[derive(Debug)]
pub struct Material(Arc<dyn Scatter>);

impl Material {
    pub(crate) fn new(material: Arc<dyn Scatter>) -> Self {
        Self(material)
    }
}

impl Deref for Material {
    type Target = Arc<dyn Scatter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Material {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
