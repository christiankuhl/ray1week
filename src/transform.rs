use std::rc::Rc;

use crate::{
    bounding_box::AaBb,
    objects::{HitRecord, Hittable, Interval},
    ray::Ray,
    vec3::{Mat3, Point3, Vec3},
};

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AaBb,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bbox() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn bbox(&self) -> AaBb {
        self.bbox
    }

    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::time_dependent(ray.origin - self.offset, ray.direction, ray.time);
        let mut hit = self.object.hit(&offset_ray, range);
        if let Some(ref mut rec) = hit {
            rec.p += self.offset;
        }
        hit
    }
}

pub struct Rotate {
    object: Rc<dyn Hittable>,
    mat: Mat3,
    mat_t: Mat3,
    bbox: AaBb,
}

impl Rotate {
    pub fn new(object: Rc<dyn Hittable>, x: f64, y: f64, z: f64) -> Self {
        let mat = Mat3::rotation(x, y, z);
        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);
        let bbox = object.bbox();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let vertex = Point3::new(
                        (i as f64) * bbox.x.max + (1.0 - i as f64) * bbox.x.min,
                        (j as f64) * bbox.y.max + (1.0 - j as f64) * bbox.y.min,
                        (k as f64) * bbox.z.max + (1.0 - k as f64) * bbox.z.min,
                    );

                    let new_vertex = mat * vertex;

                    for c in 0..3 {
                        min[c] = min[c].min(new_vertex[c]);
                        max[c] = max[c].max(new_vertex[c]);
                    }
                }
            }
        }
        let bbox = AaBb::new(min, max);

        Self {
            object,
            mat,
            mat_t: mat.transpose(),
            bbox,
        }
    }
}

impl Hittable for Rotate {
    fn bbox(&self) -> AaBb {
        self.bbox
    }

    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let rotated_ray = Ray::time_dependent(
            self.mat_t * ray.origin,
            self.mat_t * ray.direction,
            ray.time,
        );
        let mut hit = self.object.hit(&rotated_ray, range);
        if let Some(ref mut rec) = hit {
            rec.p = self.mat * rec.p;
            rec.normal = self.mat * rec.normal;
        }
        hit
    }
}
