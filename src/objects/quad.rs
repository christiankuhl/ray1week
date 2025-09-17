use std::sync::Arc;

use crate::{
    bounding_box::AaBb,
    material::Scatter,
    objects::{HitRecord, Hittable, Interval},
    ray::Ray,
    vec3::{Point3, Vec3},
};

const EPSILON: f64 = 1e-8;

#[derive(Debug, Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Scatter>,
    bbox: AaBb,
    normal: Vec3,
    intercept: f64,
    w: Vec3,
    area: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Scatter>) -> Self {
        let box1 = AaBb::new(q, q + u + v);
        let box2 = AaBb::new(q + u, q + v);
        let bbox = AaBb::enclosing(&box1, &box2);
        let n = u.cross(&v);
        let w = n / n.dot(&n);
        let normal = n.normalize();
        let intercept = normal.dot(&q);
        let area = n.length();

        Self {
            q,
            u,
            v,
            material,
            bbox,
            normal,
            intercept,
            w,
            area,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < EPSILON {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.intercept - self.normal.dot(&ray.origin)) / denom;
        if !range.surrounds(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !(0.0..1.0).contains(&alpha) || !(0.0..1.0).contains(&beta) {
            return None;
        }

        let front_face = ray.direction.dot(&self.normal) < 0.0;
        let normal = if front_face {
            self.normal
        } else {
            -self.normal
        };
        Some(HitRecord {
            p: intersection,
            t,
            material: self.material.clone(),
            normal,
            front_face,
            u: alpha,
            v: beta,
        })
    }

    fn bbox(&self) -> AaBb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if let Some(rec) = self.hit(
            &Ray::new(*origin, *direction),
            Interval::new(0.001, f64::INFINITY),
        ) {
            let distance_squared = rec.t * rec.t * direction.dot(direction);
            let cosine = (direction.dot(&rec.normal) / direction.length()).abs();
            distance_squared / (cosine * self.area)
        } else {
            0.0
        }
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        let p = self.q + (fastrand::f64() * self.u) + (fastrand::f64() * self.v);
        p - *origin
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        if self.material.is_emissive() {
            vec![Arc::new(self.clone())]
        } else {
            vec![]
        }
    }
}
