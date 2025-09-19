use std::{f64::consts::PI, sync::Arc};

use image::{ImageError, ImageReader, Rgb32FImage};

use crate::{
    colour::Colour,
    linalg::{Point3, Vec3},
    random::random_unit_vector,
};

const POINT_COUNT: usize = 256;

pub trait Texture: std::fmt::Debug + Send + Sync {
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
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn solid(scale: f64, even: Colour, odd: Colour) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColour::new(even)),
            odd: Arc::new(SolidColour::new(odd)),
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
    texture: Arc<dyn Texture>,
    z: f64,
}

impl UVSlice {
    pub fn new(texture: Arc<dyn Texture>, z: f64) -> Self {
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
    rand: [Vec3; POINT_COUNT],
    perm_x: [u8; POINT_COUNT],
    perm_y: [u8; POINT_COUNT],
    perm_z: [u8; POINT_COUNT],
}

impl Perlin {
    fn new() -> Self {
        let mut res = Self {
            rand: [Vec3::ZERO; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };
        for v in res.rand.iter_mut() {
            *v = random_unit_vector();
        }
        Self::generate(&mut res.perm_x);
        Self::generate(&mut res.perm_y);
        Self::generate(&mut res.perm_z);
        res
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cij) in ci.iter_mut().enumerate() {
                for (dk, cijk) in cij.iter_mut().enumerate() {
                    *cijk = self.rand[(self.perm_x[((i + di as isize) & 0xff) as usize]
                        ^ self.perm_y[((j + dj as isize) & 0xff) as usize]
                        ^ self.perm_z[((k + dk as isize) & 0xff) as usize])
                        as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turbulence(&self, p: Point3, depth: usize) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0.0;
        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += ((i as f64) * uu + (1.0 - (i as f64)) * (1.0 - uu))
                        * ((j as f64) * vv + (1.0 - (j as f64)) * (1.0 - vv))
                        * ((k as f64) * ww + (1.0 - (k as f64)) * (1.0 - ww))
                        * cijk.dot(&weight);
                }
            }
        }
        acc
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

#[derive(Debug)]
pub struct Plain;

#[derive(Debug)]
pub struct Turbulence(usize);

#[derive(Debug)]
pub struct Marble;

#[derive(Debug)]
pub struct NoiseTexture<T> {
    perlin: Perlin,
    scale: f64,
    kind: T,
}

impl NoiseTexture<Plain> {
    pub fn plain(scale: f64) -> Self {
        Self {
            perlin: Perlin::default(),
            scale,
            kind: Plain,
        }
    }
}

impl NoiseTexture<Turbulence> {
    pub fn turbulence(scale: f64, depth: usize) -> Self {
        Self {
            perlin: Perlin::default(),
            scale,
            kind: Turbulence(depth),
        }
    }
}

impl NoiseTexture<Marble> {
    pub fn marble(scale: f64) -> Self {
        Self {
            perlin: Perlin::default(),
            scale,
            kind: Marble,
        }
    }
}

impl Texture for NoiseTexture<Plain> {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        0.5 * (1.0 + self.perlin.noise(self.scale * p)) * Colour::WHITE
    }
}

impl Texture for NoiseTexture<Turbulence> {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        self.perlin.turbulence(self.scale * p, self.kind.0) * Colour::WHITE
    }
}

impl Texture for NoiseTexture<Marble> {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        0.5 * (1.0 + (self.scale * p.z + 10.0 * self.perlin.turbulence(p, 7)).sin()) * Colour::WHITE
    }
}

#[derive(Debug)]
pub struct SkyTexture {
    light: Colour,
    dark: Colour,
}

impl Texture for SkyTexture {
    fn value(&self, _u: f64, v: f64, _p: Point3) -> Colour {
        let y = -(v * PI).cos();
        let a = 0.5 * (y + 1.0);
        (1.0 - a) * self.light + a * self.dark
    }
}

impl Default for SkyTexture {
    fn default() -> Self {
        Self {
            light: Colour::WHITE,
            dark: Colour::new(0.5, 0.7, 1.0),
        }
    }
}
