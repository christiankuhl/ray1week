use std::sync::Arc;

use crate::{
    bounding_box::AaBb,
    objects::{HitRecord, Hittable, Interval},
    ray::Ray,
    vec3::{Mat3, Point3, Vec3},
};

#[derive(Debug)]
pub struct Translate<'a> {
    object: Arc<dyn Hittable + 'a>,
    offset: Vec3,
    bbox: AaBb,
}

impl<'a> Translate<'a> {
    pub fn new(object: Arc<dyn Hittable + 'a>, offset: Vec3) -> Self {
        let bbox = object.bbox() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<'a> Hittable for Translate<'a> {
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
        let lights = self.object.lights();
        let mut result: Vec<Arc<dyn Hittable>> = vec![];
        for light in lights {
            result.push(Arc::new(Translate::new(light, self.offset)));
        }
        result
    }
}

#[derive(Debug)]
pub struct Rotate {
    object: Arc<dyn Hittable>,
    mat: Mat3,
    mat_t: Mat3,
    bbox: AaBb,
}

impl Rotate {
    pub fn new(object: Arc<dyn Hittable>, x: f64, y: f64, z: f64) -> Self {
        let mat = Mat3::rotation(x, y, z);
        let bbox = Self::bbox_rotate(object.clone(), mat);

        Self {
            object,
            mat,
            mat_t: mat.transpose(),
            bbox,
        }
    }

    pub fn from_matrix(object: Arc<dyn Hittable>, matrix: Mat3) -> Self {
        let bbox = Self::bbox_rotate(object.clone(), matrix);

        Self {
            object,
            mat: matrix,
            mat_t: matrix.transpose(),
            bbox,
        }
    }

    fn bbox_rotate(object: Arc<dyn Hittable>, matrix: Mat3) -> AaBb {
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
        let lights = self.object.lights();
        let mut result: Vec<Arc<dyn Hittable>> = vec![];
        for light in lights {
            result.push(Arc::new(Rotate::from_matrix(light, self.mat)));
        }
        result
    }
}
