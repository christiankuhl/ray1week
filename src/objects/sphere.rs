use std::f64::consts::PI;
use std::sync::Arc;

use super::hittable::{HitRecord, Hittable, Interval};
use crate::bounding_box::AaBb;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec3::{ONB, Point3, Vec3};

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Scatter>,
    bbox: AaBb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AaBb::new(center - rvec, center + rvec);
        Self {
            center,
            radius,
            material,
            bbox,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (h - sqrtd) / a;
        if !range.surrounds(t) {
            t = (h + sqrtd) / a;
            if !range.surrounds(t) {
                return None;
            }
        }
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        let material = self.material.clone();
        let (u, v) = sphere_uv(normal);
        Some(HitRecord {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        })
    }
    fn bbox(&self) -> AaBb {
        self.bbox
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if self
            .hit(
                &Ray::new(*origin, *direction),
                Interval::new(0.001, f64::INFINITY),
            )
            .is_some()
        {
            let dist_squared = (self.center - *origin).dot(&(self.center - *origin));
            let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center - *origin;
        let distance_squared = direction.dot(&direction);
        let uvw = ONB::from_normal(&direction);
        uvw.transform(&random_to_sphere(self.radius, distance_squared))
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        if self.material.is_emissive() {
            vec![Arc::new(self.clone())]
        } else {
            vec![]
        }
    }
}

#[derive(Clone, Debug)]
pub struct MovingSphere {
    center: Ray,
    radius: f64,
    material: Arc<dyn Scatter>,
    bbox: AaBb,
}

impl MovingSphere {
    pub fn new(center1: Point3, center2: Point3, radius: f64, material: Arc<dyn Scatter>) -> Self {
        let center = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AaBb::new(center1 - rvec, center1 + rvec);
        let box2 = AaBb::new(center2 - rvec, center2 + rvec);
        let bbox = AaBb::enclosing(&box1, &box2);
        Self {
            center,
            radius,
            material,
            bbox,
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        let oc = self.center.at(ray.time) - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut t = (h - sqrtd) / a;
        if !range.surrounds(t) {
            t = (h + sqrtd) / a;
            if !range.surrounds(t) {
                return None;
            }
        }
        let p = ray.at(t);
        let normal = (p - self.center.at(ray.time)) / self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        let material = self.material.clone();
        let (u, v) = sphere_uv(normal);
        Some(HitRecord {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        })
    }
    fn bbox(&self) -> AaBb {
        self.bbox
    }
    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        todo!()
    }
    fn random(&self, _origin: &Point3) -> Vec3 {
        todo!()
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        if self.material.is_emissive() {
            vec![Arc::new(self.clone())]
        } else {
            vec![]
        }
    }
}

pub(crate) fn sphere_uv(p: Point3) -> (f64, f64) {
    use std::f64::consts::PI;
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;

    (phi / (2.0 * PI), theta / PI)
}

fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = fastrand::f64();
    let r2 = fastrand::f64();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3::new(x, y, z)
}
