use crate::{
    bounding_box::AaBb,
    linalg::{Point3, Vec3},
    objects::{Collection, HitRecord, Hittable, Interval, Object},
    ray::Ray,
};
use std::sync::Arc;

impl Hittable for UVTriangle {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        self.obj.hit(ray, range).map(|rec| {
            let p = rec.u * self.u + rec.v * self.v + self.q;
            HitRecord {
                u: p.x,
                v: p.y,
                ..rec
            }
        })
    }
    fn bbox(&self) -> AaBb {
        self.obj.bbox()
    }
    fn lights(&self) -> Collection {
        self.obj.lights()
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        self.obj.pdf_value(origin, direction)
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.obj.random(origin)
    }
}

#[derive(Debug)]
pub struct UVTriangle {
    q: Point3,
    u: Vec3,
    v: Vec3,
    obj: Object,
}

impl UVTriangle {
    pub fn new(q: Point3, u: Vec3, v: Vec3, obj: Object) -> Object {
        Object::new(Arc::new(Self { q, u, v, obj }))
    }
}
