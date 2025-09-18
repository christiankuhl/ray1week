use std::sync::Arc;

use crate::{
    colour::Colour,
    material::{Isotropic, Scatter},
    objects::{Collection, HitRecord, Hittable, Interval, IntoPrimitives, Object},
    ray::Ray,
    texture::SolidColour,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Debug)]
pub struct ConstantMedium {
    boundary: Object,
    neg_inv_density: f64,
    phase_function: Arc<dyn Scatter>,
}

impl ConstantMedium {
    pub fn new(
        boundary: impl IntoPrimitives,
        density: f64,
        phase_function: Arc<dyn Scatter>,
    ) -> Collection {
        let mut result = Collection::new();
        for bd in boundary.primitives() {
            result.add(Object::new(Arc::new(Self {
                boundary: bd,
                neg_inv_density: -1.0 / density,
                phase_function: Arc::clone(&phase_function),
            })));
        }
        result
    }

    pub fn isotropic(boundary: impl IntoPrimitives, density: f64, albedo: Colour) -> Collection {
        Self::new(
            boundary,
            density,
            Arc::new(Isotropic::new(Arc::new(SolidColour::new(albedo)))),
        )
    }
}

impl Hittable for ConstantMedium {
    fn bbox(&self) -> crate::bounding_box::AaBb {
        self.boundary.bbox()
    }

    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        if let Some(ref mut rec1) = self.boundary.hit(ray, Interval::universe()) {
            if let Some(ref mut rec2) = self
                .boundary
                .hit(ray, Interval::new(rec1.t + 0.0001, f64::INFINITY))
            {
                if rec1.t < range.min {
                    rec1.t = range.min;
                }
                if rec2.t > range.max {
                    rec2.t = range.max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = ray.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * fastrand::f64().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_length;
                Some(HitRecord {
                    t,
                    p: ray.at(t),
                    normal: Vec3::EX,
                    front_face: true,
                    material: Arc::clone(&self.phase_function),
                    u: 0.0,
                    v: 0.0,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _origin: &Point3) -> Vec3 {
        Vec3::EX
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        if self.phase_function.is_emissive() {
            vec![Arc::new(self.clone())]
        } else {
            vec![]
        }
    }
}
