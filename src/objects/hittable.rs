use std::fmt::Debug;
use std::ops::Add;
use std::sync::Arc;

use crate::bounding_box::AaBb;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Scatter>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }

    pub fn extend(&mut self, delta: f64) {
        self.min -= delta;
        self.max += delta;
    }

    pub fn enclosing(interval1: Self, interval2: Self) -> Self {
        let min = if interval1.min <= interval2.min {
            interval1.min
        } else {
            interval2.min
        };
        let max = if interval1.max >= interval2.max {
            interval1.max
        } else {
            interval2.max
        };
        Self { min, max }
    }

    pub fn length(&self) -> f64 {
        self.max - self.min
    }

    pub fn universe() -> Self {
        Self {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord>;
    fn bbox(&self) -> AaBb;
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64;
    fn random(&self, origin: &Point3) -> Vec3;
    fn lights(&self) -> Vec<Arc<dyn Hittable>>;
}

#[derive(Default, Debug)]
pub struct Collection<'a> {
    pub objects: Vec<Arc<dyn Hittable + 'a>>,
    bbox: AaBb,
}

impl<'a> Collection<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AaBb::default(),
        }
    }

    pub fn with_objects(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self {
            objects,
            bbox: AaBb::default(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.bbox = AaBb::enclosing(&self.bbox, &object.bbox());
        self.objects.push(Arc::new(object));
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
    fn bbox(&self) -> AaBb {
        self.bbox
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        self.objects
            .iter()
            .map(|o| o.pdf_value(origin, direction) * weight)
            .sum()
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.objects[fastrand::usize(0..self.objects.len())].random(origin)
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        self.objects.iter().flat_map(|o| o.lights()).collect()
    }
}
