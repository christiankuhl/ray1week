use image::{ImageError, ImageReader, Rgb32FImage};

use std::{f64::consts::PI, sync::Arc};

use crate::{
    colour::Colour,
    linalg::{Point3, Vec3},
    texture::{Texture, Textured, perlin::Perlin},
};

#[derive(Debug, Clone, Copy)]
pub struct SolidColour {
    albedo: Colour,
}

impl SolidColour {
    pub fn new(albedo: Colour) -> Texture {
        Texture::new(Arc::new(Self { albedo }))
    }
}

impl Textured for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Colour {
        self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Texture,
    odd: Texture,
}

impl CheckerTexture {
    pub fn solid(scale: f64, even: Colour, odd: Colour) -> Texture {
        Texture::new(Arc::new(Self {
            inv_scale: 1.0 / scale,
            even: SolidColour::new(even),
            odd: SolidColour::new(odd),
        }))
    }
}

impl Textured for CheckerTexture {
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
    texture: Texture,
    z: f64,
}

impl UVSlice {
    pub fn new(texture: Texture, z: f64) -> Texture {
        Texture::new(Arc::new(Self { texture, z }))
    }
}

impl Textured for UVSlice {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Colour {
        self.texture.value(u, v, Vec3::new(u, v, self.z))
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    buffer: Rgb32FImage,
}

impl ImageTexture {
    pub fn new(path: &str) -> Result<Texture, ImageError> {
        Ok(Texture::new(Arc::new(Self {
            buffer: ImageReader::open(path)?.decode()?.into_rgb32f(),
        })))
    }
}

impl Textured for ImageTexture {
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
    pub fn plain(scale: f64) -> Texture {
        Texture::new(Arc::new(Self {
            perlin: Perlin::default(),
            scale,
            kind: Plain,
        }))
    }
}

impl NoiseTexture<Turbulence> {
    pub fn turbulence(scale: f64, depth: usize) -> Texture {
        Texture::new(Arc::new(Self {
            perlin: Perlin::default(),
            scale,
            kind: Turbulence(depth),
        }))
    }
}

impl NoiseTexture<Marble> {
    pub fn marble(scale: f64) -> Texture {
        Texture::new(Arc::new(Self {
            perlin: Perlin::default(),
            scale,
            kind: Marble,
        }))
    }
}

impl Textured for NoiseTexture<Plain> {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        0.5 * (1.0 + self.perlin.noise(self.scale * p)) * Colour::WHITE
    }
}

impl Textured for NoiseTexture<Turbulence> {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        self.perlin.turbulence(self.scale * p, self.kind.0) * Colour::WHITE
    }
}

impl Textured for NoiseTexture<Marble> {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Colour {
        0.5 * (1.0 + (self.scale * p.z + 10.0 * self.perlin.turbulence(p, 7)).sin()) * Colour::WHITE
    }
}

#[derive(Debug)]
pub struct SkyTexture {
    light: Colour,
    dark: Colour,
}

impl Textured for SkyTexture {
    fn value(&self, _u: f64, v: f64, _p: Point3) -> Colour {
        let y = -(v * PI).cos();
        let a = 0.5 * (y + 1.0);
        (1.0 - a) * self.light + a * self.dark
    }
}

impl SkyTexture {
    fn new(light: Colour, dark: Colour) -> Texture {
        Texture::new(Arc::new(Self { light, dark }))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Texture {
        Self::new(Colour::WHITE, Colour::new(0.5, 0.7, 1.0))
    }
}
