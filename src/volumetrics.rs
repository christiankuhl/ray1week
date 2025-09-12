use std::rc::Rc;

use crate::{
    colour::Colour,
    material::{Isotropic, Scatter},
    objects::{HitRecord, Hittable, Interval},
    texture::SolidColour,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Scatter>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, phase_function: Rc<dyn Scatter>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn isotropic(boundary: Rc<dyn Hittable>, density: f64, albedo: Colour) -> Self {
        Self::new(
            boundary,
            density,
            Rc::new(Isotropic::new(Rc::new(SolidColour::new(albedo)))),
        )
    }
}

impl Hittable for ConstantMedium {
    fn bbox(&self) -> crate::bounding_box::AaBb {
        self.boundary.bbox()
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        range: crate::objects::Interval,
    ) -> Option<crate::objects::HitRecord> {
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
                    material: self.phase_function.clone(),
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
}
