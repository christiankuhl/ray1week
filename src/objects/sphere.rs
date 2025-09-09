use std::rc::Rc;

use super::hittable::{HitRecord, Hittable, Interval};
use crate::bounding_box::AaBb;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Scatter>,
    bbox: AaBb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Scatter>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AaBb::new(center - rvec, center + rvec);
        Self {
            center,
            radius,
            material,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (h - sqrtd) / a;
        if !range.surrounds(t) {
            t = (h + sqrtd) / a;
            if !range.surrounds(t) {
                return None;
            }
        }
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        let material = self.material.clone();
        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        })
    }
    fn bbox(&self) -> AaBb {
        self.bbox
    }
}

pub struct MovingSphere {
    center: Ray,
    radius: f64,
    material: Rc<dyn Scatter>,
    bbox: AaBb,
}

impl MovingSphere {
    pub fn new(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Scatter>) -> Self {
        let center = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AaBb::new(center1 - rvec, center1 + rvec);
        let box2 = AaBb::new(center2 - rvec, center2 + rvec);
        let bbox = AaBb::enclosing(&box1, &box2);
        Self {
            center,
            radius,
            material,
            bbox,
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let oc = self.center.at(ray.time) - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (h - sqrtd) / a;
        if !range.surrounds(t) {
            t = (h + sqrtd) / a;
            if !range.surrounds(t) {
                return None;
            }
        }
        let p = ray.at(t);
        let normal = (p - self.center.at(ray.time)) / self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        let material = self.material.clone();
        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        })
    }
    fn bbox(&self) -> AaBb {
        self.bbox
    }
}
