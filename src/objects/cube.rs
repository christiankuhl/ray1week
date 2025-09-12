use std::rc::Rc;

use crate::{
    material::Scatter,
    objects::{Collection, Hittable, Quad},
    vec3::{Point3, Vec3},
};

pub struct Cube<'a>(Collection<'a>);

impl<'a> Cube<'a> {
    pub fn new(a: Point3, b: Point3, material: Rc<dyn Scatter>) -> Self {
        let mut sides = Collection::new();
        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
        let dx = (max.x - min.x) * Vec3::EX;
        let dy = (max.y - min.y) * Vec3::EY;
        let dz = (max.z - min.z) * Vec3::EZ;

        sides.add(Quad::new(
            Point3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dx,
            dz,
            material.clone(),
        ));
        Self(sides)
    }
}

impl<'a> Hittable for Cube<'a> {
    fn hit(&self, ray: &crate::ray::Ray, range: super::Interval) -> Option<super::HitRecord> {
        self.0.hit(ray, range)
    }

    fn bbox(&self) -> crate::bounding_box::AaBb {
        self.0.bbox()
    }
}
