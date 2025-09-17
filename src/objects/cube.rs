use std::sync::Arc;

use crate::{
    material::Scatter,
    objects::{Collection, HitRecord, Hittable, Interval, Quad},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub struct Cube<'a>(Collection<'a>);

impl<'a> Cube<'a> {
    pub fn new(a: Point3, b: Point3, material: Arc<dyn Scatter>) -> Self {
        let mut sides = Collection::new();
        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
        let dx = (max.x - min.x) * Vec3::EX;
        let dy = (max.y - min.y) * Vec3::EY;
        let dz = (max.z - min.z) * Vec3::EZ;

        sides.add(Quad::new(
            Point3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dx,
            dz,
            material.clone(),
        ));
        Self(sides)
    }
}

impl<'a> Hittable for Cube<'a> {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        self.0.hit(ray, range)
    }

    fn bbox(&self) -> crate::bounding_box::AaBb {
        self.0.bbox()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        self.0.pdf_value(origin, direction)
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.0.random(origin)
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        self.0.lights()
    }
}
