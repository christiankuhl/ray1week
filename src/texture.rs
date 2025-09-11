use std::rc::Rc;

use image::{ImageError, ImageReader, Rgb32FImage};

use crate::{
    colour::Colour,
    vec3::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

pub trait Texture: std::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour;
}

#[derive(Debug, Clone, Copy)]
pub struct SolidColour {
    albedo: Colour,
}

impl SolidColour {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Colour {
        self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn solid(scale: f64, even: Colour, odd: Colour) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(SolidColour::new(even)),
            odd: Rc::new(SolidColour::new(odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour {
        let x = (p.x * self.inv_scale).floor() as isize;
        let y = (p.y * self.inv_scale).floor() as isize;
        let z = (p.z * self.inv_scale).floor() as isize;
        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct UVSlice {
    texture: Rc<dyn Texture>,
    z: f64,
}

impl UVSlice {
    pub fn new(texture: Rc<dyn Texture>, z: f64) -> Self {
        Self { texture, z }
    }
}

impl Texture for UVSlice {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Colour {
        self.texture.value(u, v, Vec3::new(u, v, self.z))
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    buffer: Rgb32FImage,
}

impl ImageTexture {
    pub fn new(path: &str) -> Result<Self, ImageError> {
        Ok(Self {
            buffer: ImageReader::open(path)?.decode()?.into_rgb32f(),
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Colour {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let row = ((v * self.buffer.height() as f64) as u32).min(self.buffer.height() - 1);
        let col = ((u * self.buffer.width() as f64) as u32).min(self.buffer.width() - 1);
        let pixel = self.buffer.get_pixel(col, row);
        Colour::new(pixel.0[0] as f64, pixel.0[1] as f64, pixel.0[2] as f64)
    }
}

#[derive(Debug)]
struct Perlin {
    rand: [f64; POINT_COUNT],
    perm_x: [u8; POINT_COUNT],
    perm_y: [u8; POINT_COUNT],
    perm_z: [u8; POINT_COUNT],
}

impl Perlin {
    fn new() -> Self {
        let mut res = Self {
            rand: [0.0; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };
        for v in res.rand.iter_mut() {
            *v = fastrand::f64();
        }
        Self::generate(&mut res.perm_x);
        Self::generate(&mut res.perm_y);
        Self::generate(&mut res.perm_z);
        res
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x) as isize) & 0xff;
        let j = ((4.0 * p.y) as isize) & 0xff;
        let k = ((4.0 * p.z) as isize) & 0xff;
        self.rand
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }

    fn generate(p: &mut [u8]) {
        for (i, v) in p.iter_mut().enumerate() {
            *v = i as u8;
        }
        Self::permute(p);
    }

    fn permute(p: &mut [u8]) {
        for i in (1..POINT_COUNT).rev() {
            let tgt = fastrand::u8(0..i as u8);
            p.swap(i, tgt as usize);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct NoiseTexture {
    perlin: Perlin,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        self.perlin.noise(p) * Colour::WHITE
    }
}
