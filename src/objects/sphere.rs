use super::hittable::{HitRecord, Hittable, Interval};
use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: &'a dyn Scatter,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a dyn Scatter) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
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
        let material = self.material;
        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        })
    }
}
