use std::sync::Arc;

use crate::{
    bounding_box::AaBb,
    linalg::{Point3, Vec3},
    material::Material,
    objects::{Collection, HitRecord, Hittable, Interval, Object},
    ray::Ray,
};

const EPSILON: f64 = 1e-8;

#[derive(Debug, Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    alpha0: Vec3,
    beta0: Vec3,
    material: Material,
    bbox: AaBb,
    normal: Vec3,
    intercept: f64,
    area: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Material) -> Object {
        let box1 = AaBb::new(q, q + u + v);
        let box2 = AaBb::new(q + u, q + v);
        let bbox = AaBb::enclosing(&box1, &box2);
        let n = u.cross(&v);
        let w = n / n.dot(&n);
        let alpha0 = v.cross(&w);
        let beta0 = w.cross(&u);
        let normal = n.normalize();
        let intercept = normal.dot(&q);
        let area = n.length();

        Object::new(Arc::new(Self {
            q,
            u,
            v,
            alpha0,
            beta0,
            material,
            bbox,
            normal,
            intercept,
            area,
        }))
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let det = self.normal.dot(&ray.direction);
        // No hit if the ray is parallel to the plane.
        if det.abs() < EPSILON {
            return None;
        }
        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.intercept - self.normal.dot(&ray.origin)) / det;
        if !range.surrounds(t) {
            return None;
        }
        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = planar_hitpt_vector.dot(&self.alpha0);
        let beta = planar_hitpt_vector.dot(&self.beta0);

        if !(0.0..1.0).contains(&alpha) || !(0.0..1.0).contains(&beta) {
            return None;
        }
        // Finally, we have a hit
        let front_face = det < 0.0;
        let normal = (2 * (front_face as isize) - 1) as f64 * self.normal;
        Some(HitRecord {
            p: intersection,
            t,
            material: self.material.as_ref(),
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
    fn lights(&self) -> Collection {
        let mut res = Collection::new();
        if self.material.is_emissive() {
            res.add(Object::new(Arc::new(self.clone())));
        }
        res
    }
}
