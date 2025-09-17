use std::ops::{Add, Index};
use std::sync::Arc;

const DELTA: f64 = 0.0001;

use crate::vec3::Vec3;
use crate::{
    objects::{HitRecord, Hittable, Interval},
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct AaBb {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
}

impl AaBb {
    pub fn new(a: Point3, b: Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };
        let mut res = Self { x, y, z };
        res.pad_to_minumums();
        res
    }

    pub fn enclosing(box1: &Self, box2: &Self) -> Self {
        let x = Interval::enclosing(box1.x, box2.x);
        let y = Interval::enclosing(box1.y, box2.y);
        let z = Interval::enclosing(box1.z, box2.z);
        let mut res = Self { x, y, z };
        res.pad_to_minumums();
        res
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<Interval> {
        let mut t = ray_t;
        for axis in 0..3 {
            let ax = self[axis];
            let adinv = 1.0 / r.direction[axis];

            let t0 = (ax.min - r.origin[axis]) * adinv;
            let t1 = (ax.max - r.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > t.min {
                    t.min = t0;
                }
                if t1 < t.max {
                    t.max = t1;
                }
            } else {
                if t1 > t.min {
                    t.min = t1;
                }
                if t0 < t.max {
                    t.max = t0;
                }
            }

            if t.max <= t.min {
                return None;
            }
        }
        Some(t)
    }

    fn longest_axis(&self) -> usize {
        [self.x, self.y, self.z]
            .iter()
            .map(|j| j.length())
            .enumerate()
            .max_by(|(_i, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0
    }

    fn pad_to_minumums(&mut self) {
        if self.x.length() < DELTA {
            self.x.extend(DELTA);
        }
        if self.y.length() < DELTA {
            self.y.extend(DELTA);
        }
        if self.z.length() < DELTA {
            self.z.extend(DELTA);
        }
    }
}

impl Index<usize> for AaBb {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Attempt to index AaBb in dimension {index}!"),
        }
    }
}

impl Add<Vec3> for AaBb {
    type Output = AaBb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug)]
pub struct BVHNode<'a> {
    left: Arc<dyn Hittable + 'a>,
    right: Arc<dyn Hittable + 'a>,
    bbox: AaBb,
}

impl<'a> BVHNode<'a> {
    pub fn new(objects: &mut Vec<Arc<dyn Hittable + 'a>>) -> Self {
        let mut bbox = AaBb::default();
        for obj in objects.iter() {
            bbox = AaBb::enclosing(&bbox, &obj.bbox());
        }
        let axis = bbox.longest_axis();

        if objects.len() == 1 {
            let obj = objects[0].clone();
            return Self {
                left: obj.clone(),
                right: obj.clone(),
                bbox,
            };
        } else if objects.len() == 2 {
            let bbox = AaBb::enclosing(&objects[0].bbox(), &objects[1].bbox());
            return Self {
                left: objects[0].clone(),
                right: objects[1].clone(),
                bbox,
            };
        }

        objects.sort_by(|obj1, obj2| {
            obj1.bbox()[axis]
                .min
                .partial_cmp(&obj2.bbox()[axis].min)
                .unwrap()
        });
        let mid = objects.len() / 2;
        let mut right = objects.split_off(mid);
        let left = Arc::new(Self::new(objects));
        let right = Arc::new(Self::new(&mut right));
        Self { left, right, bbox }
    }
}

impl<'a> Hittable for BVHNode<'a> {
    fn hit(&self, ray: &Ray, range: Interval) -> Option<HitRecord> {
        self.bbox.hit(ray, range)?;
        let mut max = range.max;
        let hit_left = self.left.hit(ray, range);
        if let Some(ref rec_left) = hit_left {
            max = rec_left.t;
        }
        let hit_right = self.right.hit(ray, Interval::new(range.min, max));
        hit_right.or(hit_left)
    }
    fn bbox(&self) -> AaBb {
        self.bbox
    }
    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        panic!("Asked for PDF on a bounding box!")
    }
    fn random(&self, _origin: &Point3) -> Vec3 {
        panic!("Asked for PDF on a bounding box!")
    }
    fn lights(&self) -> Vec<Arc<dyn Hittable>> {
        let mut res = self.left.lights();
        res.extend(self.right.lights());
        res
    }
}
