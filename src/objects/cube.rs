use crate::{
    linalg::{Point3, Vec3},
    material::Material,
    objects::{IntoPrimitives, Object, Quad},
};

#[derive(Debug)]
pub struct Cube(Vec<Object>);

impl Cube {
    pub fn new(a: Point3, b: Point3, material: Material) -> Self {
        let mut sides = Vec::new();
        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
        let dx = (max.x - min.x) * Vec3::EX;
        let dy = (max.y - min.y) * Vec3::EY;
        let dz = (max.z - min.z) * Vec3::EZ;

        sides.push(Quad::new(
            Point3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        ));
        sides.push(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        ));
        sides.push(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        ));
        sides.push(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        ));
        sides.push(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        ));
        sides.push(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dx,
            dz,
            material.clone(),
        ));
        Self(sides)
    }
}

impl IntoPrimitives for Cube {
    fn primitives(&self) -> Vec<super::Object> {
        self.0.to_vec()
    }
}
