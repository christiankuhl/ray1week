use std::f64::consts::PI;

use crate::{
    linalg::{ONB, Point3, Vec3},
    objects::Hittable,
};

pub fn sample_square() -> Vec3 {
    Vec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, 0.0)
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::new(
            2.0 * fastrand::f64() - 1.0,
            2.0 * fastrand::f64() - 1.0,
            2.0 * fastrand::f64() - 1.0,
        );
        let lp = p.dot(&p);
        if lp > 1e-160 && lp <= 1.0 {
            return p / lp.sqrt();
        }
    }
}

pub fn random_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * sample_square();
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = fastrand::f64();
    let r2 = fastrand::f64();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();
    Vec3::new(x, y, z)
}

pub trait DirectionalPDF: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct UniformSphericalPDF;

impl DirectionalPDF for UniformSphericalPDF {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }
    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(normal: &Vec3) -> Self {
        Self {
            uvw: ONB::from_normal(normal),
        }
    }
}

impl DirectionalPDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = direction.normalize().dot(&self.uvw.w);
        (cosine_theta / PI).max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.transform(&random_cosine_direction())
    }
}

pub struct HittablePDF<'a> {
    objects: &'a dyn Hittable,
    origin: Point3,
}

impl<'a> HittablePDF<'a> {
    pub fn new(objects: &'a dyn Hittable, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl<'a> DirectionalPDF for HittablePDF<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePDF<'a> {
    p0: &'a dyn DirectionalPDF,
    p1: &'a dyn DirectionalPDF,
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: &'a dyn DirectionalPDF, p1: &'a dyn DirectionalPDF) -> Self {
        Self { p0, p1 }
    }
}

impl<'a> DirectionalPDF for MixturePDF<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if fastrand::f64() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
