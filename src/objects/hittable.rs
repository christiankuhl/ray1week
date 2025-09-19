use std::fmt::Debug;
use std::ops::Add;

use crate::{
    bounding_box::AaBb,
    linalg::{Point3, Vec3},
    material::Material,
    objects::Collection,
    ray::Ray,
};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
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
    fn lights(&self) -> Collection;
}
