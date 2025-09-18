use std::sync::Arc;

use crate::bounding_box::AaBb;
use crate::objects::{Collection, HitRecord, Hittable, Interval, IntoPrimitives, Object};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Scene(Collection);

impl Scene {
    pub fn new() -> Self {
        Self(Collection::new())
    }
    pub fn with_objects(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self(Collection::with_objects(objects))
    }
    pub fn add(&mut self, object: impl IntoPrimitives) {
        for obj in object.primitives() {
            self.0.add(obj.clone());
        }
    }
    pub fn objects(&self) -> &[Object] {
        &self.0.objects
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = range.max;

        for object in self.0.objects.iter() {
            if let Some(rec) = object.0.hit(ray, Interval::new(range.min, closest_so_far)) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
    fn bbox(&self) -> AaBb {
        self.0.bbox
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.0.objects.len() as f64;
        self.0
            .objects
            .iter()
            .map(|o| o.0.pdf_value(origin, direction) * weight)
            .sum()
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.0.objects[fastrand::usize(0..self.0.objects.len())]
            .0
            .random(origin)
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        self.0.objects.iter().flat_map(|o| o.0.lights()).collect()
    }
}
