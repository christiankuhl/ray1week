use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

const EPSILON: f64 = 1e-8;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const EX: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const EY: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const EZ: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self {
            x: fastrand::f64() * (max - min) + min,
            y: fastrand::f64() * (max - min) + min,
            z: fastrand::f64() * (max - min) + min,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Attempt to index AaBb in dimension {index}!"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Attempt to index AaBb in dimension {index}!"),
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Mat3([[f64; 3]; 3]);

impl Mat3 {
    pub fn rotation(x: f64, y: f64, z: f64) -> Self {
        let alpha = z.to_radians();
        let beta = y.to_radians();
        let gamma = x.to_radians();

        Self([
            [
                beta.cos() * alpha.cos(),
                alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.cos(),
                alpha.cos() * beta.sin() * gamma.cos() + alpha.sin() * gamma.sin(),
            ],
            [
                alpha.sin() * beta.cos(),
                alpha.sin() * beta.sin() * gamma.sin() + alpha.cos() * gamma.cos(),
                alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin(),
            ],
            [
                -beta.sin(),
                beta.cos() * gamma.sin(),
                beta.cos() * gamma.cos(),
            ],
        ])
    }

    pub fn transpose(&self) -> Self {
        Self([
            [self.0[0][0], self.0[1][0], self.0[2][0]],
            [self.0[0][1], self.0[1][1], self.0[2][1]],
            [self.0[0][2], self.0[1][2], self.0[2][2]],
        ])
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let x = Vec3::new(self.0[0][0], self.0[0][1], self.0[0][2]).dot(&rhs);
        let y = Vec3::new(self.0[1][0], self.0[1][1], self.0[1][2]).dot(&rhs);
        let z = Vec3::new(self.0[2][0], self.0[2][1], self.0[2][2]).dot(&rhs);
        Self::Output { x, y, z }
    }
}

impl Mul<f64> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: f64) -> Self::Output {
        Mat3([
            [rhs * self.0[0][0], rhs * self.0[0][1], rhs * self.0[0][2]],
            [rhs * self.0[1][0], rhs * self.0[1][1], rhs * self.0[1][2]],
            [rhs * self.0[2][0], rhs * self.0[2][1], rhs * self.0[2][2]],
        ])
    }
}

pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self { u, v, w }
    }

    pub fn from_normal(n: &Vec3) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 { Vec3::EY } else { Vec3::EX };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);
        Self { u, v, w }
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        v.x * self.u + v.y * self.v + v.z * self.w
    }
}
