use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Scatter,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct Collection<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> Collection<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.objects.push(Box::new(object));
    }
}

impl<'a> Hittable for Collection<'a> {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = range.max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(ray, Interval::new(range.min, closest_so_far)) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
