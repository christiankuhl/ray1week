use std::rc::Rc;

use crate::bounding_box::AaBb;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Scatter>,
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

    pub fn extend(&self, delta: f64) -> Self {
        Self {
            min: self.min - delta,
            max: self.max + delta,
        }
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
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord>;
    fn bbox(&self) -> AaBb;
}

#[derive(Default)]
pub struct Collection<'a> {
    pub objects: Vec<Rc<dyn Hittable + 'a>>,
    bbox: AaBb,
}

impl<'a> Collection<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AaBb::default(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.bbox = AaBb::enclosing(&self.bbox, &object.bbox());
        self.objects.push(Rc::new(object));
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
}
