use std::sync::Arc;

use crate::{
    bounding_box::AaBb,
    objects::{Collection, HitRecord, Hittable, Interval, IntoPrimitives, Object},
    ray::Ray,
    vec3::{Mat3, Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Translate {
    pub object: Object,
    pub offset: Vec3,
    pub bbox: AaBb,
}

impl Translate {
    pub fn new(object: impl IntoPrimitives, offset: Vec3) -> Collection {
        let mut result = Collection::new();
        for obj in object.primitives() {
            let bbox = obj.bbox() + offset;
            result.add(Object(Arc::new(Self {
                object: obj,
                offset,
                bbox,
            })));
        }
        result
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

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        self.object.pdf_value(&(*origin - self.offset), direction)
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.object.random(&(*origin - self.offset))
    }

    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        let lights = Translate::new(Collection::with_objects(self.object.lights()), self.offset);
        lights.objects.iter().map(|o| Arc::clone(&o.0)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Rotate {
    object: Object,
    mat: Mat3,
    mat_t: Mat3,
    bbox: AaBb,
}

impl Rotate {
    pub fn new(object: impl IntoPrimitives, x: f64, y: f64, z: f64) -> Collection {
        let mat = Mat3::rotation(x, y, z);
        Self::from_matrix(object, mat)
    }

    fn from_matrix(object: impl IntoPrimitives, mat: Mat3) -> Collection {
        let mut result = Collection::new();
        let mat_t = mat.transpose();
        for obj in object.primitives() {
            let bbox = Self::bbox_rotate(obj.clone(), mat);
            result.add(Object(Arc::new(Self {
                object: obj,
                mat,
                mat_t,
                bbox,
            })));
        }
        result
    }

    fn bbox_rotate(object: Object, matrix: Mat3) -> AaBb {
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

                    let new_vertex = matrix * vertex;

                    for c in 0..3 {
                        min[c] = min[c].min(new_vertex[c]);
                        max[c] = max[c].max(new_vertex[c]);
                    }
                }
            }
        }
        AaBb::new(min, max)
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

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        self.object
            .pdf_value(&(self.mat_t * (*origin)), &(self.mat_t * (*direction)))
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.mat * self.object.random(&(self.mat_t * (*origin)))
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        let lights = Rotate::from_matrix(Collection::with_objects(self.object.lights()), self.mat);
        lights.objects.iter().map(|o| Arc::clone(&o.0)).collect()
    }
}
