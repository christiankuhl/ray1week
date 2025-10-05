use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use image::Rgb;

use crate::linalg::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

impl Colour {
    pub const BLACK: Colour = Colour {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn ppm(&self) -> String {
        let r = (256.0 * self.r.clamp(0.0, 0.999).sqrt()) as usize;
        let g = (256.0 * self.g.clamp(0.0, 0.999).sqrt()) as usize;
        let b = (256.0 * self.b.clamp(0.0, 0.999).sqrt()) as usize;
        format!("{r} {g} {b}")
    }

    pub fn attenuate(&self, other: &Colour) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }

    pub fn random() -> Self {
        Self {
            r: fastrand::f64(),
            g: fastrand::f64(),
            b: fastrand::f64(),
        }
    }
}

impl From<&Colour> for Rgb<f32> {
    fn from(value: &Colour) -> Self {
        Self([
            value.r.clamp(0.0, 1.0).sqrt() as f32,
            value.g.clamp(0.0, 1.0).sqrt() as f32,
            value.b.clamp(0.0, 1.0).sqrt() as f32,
        ])
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl From<Vec3> for Colour {
    fn from(value: Vec3) -> Self {
        Self {
            r: value.x,
            g: value.y,
            b: value.z,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Div<f64> for Colour {
    type Output = Colour;

    fn div(self, rhs: f64) -> Self::Output {
        Colour {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Default for Colour {
    fn default() -> Self {
        Colour::BLACK
    }
}

pub type Color = Colour;
