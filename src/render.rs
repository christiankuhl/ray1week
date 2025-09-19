use std::f64;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

use image::buffer::ConvertBuffer;
use image::{ImageError, Rgb32FImage, RgbImage};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::bounding_box::BVHNode;
use crate::colour::Colour;
use crate::linalg::{Point3, Vec3};
use crate::material::ScatterResult;
use crate::objects::{Hittable, Interval, sphere_uv};
use crate::random::{DirectionalPDF, HittablePDF, MixturePDF, random_unit_disk};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::texture::{SkyTexture, Texture};

#[derive(Debug, Clone)]
pub struct Camera {
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub up: Vec3,
    pub vfov: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub background: Texture,
}

pub struct Renderer {
    image_width: usize,
    image_height: usize,
    sqrt_spp: usize,
    recip_sqrt_spp: f64,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: usize,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    center: Point3,
    background: Texture,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            lookfrom: Point3::ZERO,
            lookat: -Point3::EZ,
            up: Vec3::EY,
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            background: SkyTexture::default(),
        }
    }
}

impl Camera {
    pub fn renderer(&self, samples_per_pixel: usize, max_depth: usize) -> Renderer {
        let image_height = ((self.image_width as f64 / self.aspect_ratio) as usize).max(1);

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.lookfrom - self.focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as usize;

        Renderer {
            image_width: self.image_width,
            image_height,
            sqrt_spp,
            recip_sqrt_spp: 1.0 / sqrt_spp as f64,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth,
            defocus_disk_u,
            defocus_disk_v,
            center: self.lookfrom,
            background: self.background.clone(),
        }
    }
}

impl Renderer {
    const BLOCK_SIZE: usize = 64;
    fn get_ray(&self, x: usize, y: usize, si: usize, sj: usize) -> Ray {
        let offset = self.sample_square_stratified(si, sj);
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_disk_u.near_zero() {
            self.center
        } else {
            self.defocus_sample()
        };
        Ray::time_dependent(ray_origin, pixel_sample - ray_origin, fastrand::f64())
    }

    fn sample_square_stratified(&self, si: usize, sj: usize) -> Vec3 {
        Vec3::new(
            ((si as f64) + fastrand::f64()) * self.recip_sqrt_spp - 0.5,
            ((sj as f64) + fastrand::f64()) * self.recip_sqrt_spp - 0.5,
            0.0,
        )
    }

    fn defocus_sample(&self) -> Vec3 {
        let p = random_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn render_block(&self, block: &mut ImageBlock, world: &BVHNode, lights: Arc<Scene>) {
        let pixel_samples_scale = 1.0 / (self.sqrt_spp * self.sqrt_spp) as f64;
        for y in block.ymin..block.ymax {
            for x in block.xmin..block.xmax {
                let mut c = Colour::new(0.0, 0.0, 0.0);
                for sj in 0..self.sqrt_spp {
                    for si in 0..self.sqrt_spp {
                        let r = self.get_ray(x, y, si, sj);
                        c += ray_colour(
                            r,
                            world,
                            Arc::clone(&lights),
                            self.max_depth,
                            self.background.clone(),
                        );
                    }
                }
                c = pixel_samples_scale * c;
                block.write(x, y, c);
            }
        }
    }

    pub fn render<P>(&self, world: &mut Scene, p: &mut P) -> RgbImage
    where
        P: Write + Sync + Send,
    {
        let p = Arc::new(Mutex::new(p));
        writeln!(p.lock().unwrap(), "Collecting light sources...").unwrap();
        let lights = Arc::new(Scene::with_objects(world.lights()));
        writeln!(p.lock().unwrap(), "Building render node hierarchy...").unwrap();
        let mut raw_objects = world.objects().iter().map(|o| Arc::clone(o)).collect();
        let bvh = BVHNode::new(&mut raw_objects);
        let mut blocks = self.image_blocks();
        writeln!(
            p.lock().unwrap(),
            "Split target image into {} blocks...",
            blocks.len()
        )
        .unwrap();
        let done = AtomicUsize::new(0);
        let total = blocks.len();
        blocks.par_iter_mut().for_each(|block| {
            self.render_block(block, &bvh, lights.clone());
            done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            write!(
                p.lock().unwrap(),
                "\r{:.2}% done...",
                done.load(std::sync::atomic::Ordering::Relaxed) as f64 / total as f64 * 100.0
            )
            .unwrap();
        });
        writeln!(
            p.lock().unwrap(),
            "\rRay tracing done.                             "
        )
        .unwrap();
        self.assemble_image(&blocks)
    }

    pub fn render_to_file<F, P>(
        &self,
        world: &mut Scene,
        path: F,
        p: &mut P,
    ) -> Result<(), ImageError>
    where
        F: AsRef<Path> + Display,
        P: Write + Sync + Send,
    {
        let buffer = self.render(world, p);
        writeln!(p, "Saving image to {path}...").unwrap();
        buffer.save(path)
    }

    fn image_blocks(&self) -> Vec<ImageBlock> {
        let mut blocks = Vec::new();
        for i in 0..self.image_height / Self::BLOCK_SIZE + 1 {
            let ymin = i * Self::BLOCK_SIZE;
            let ymax = ((i + 1) * Self::BLOCK_SIZE).min(self.image_height);
            for j in 0..self.image_width / Self::BLOCK_SIZE + 1 {
                let xmin = j * Self::BLOCK_SIZE;
                let xmax = ((j + 1) * Self::BLOCK_SIZE).min(self.image_width);
                blocks.push(ImageBlock::new(xmin, xmax, ymin, ymax));
            }
        }
        blocks
    }

    fn assemble_image(&self, blocks: &[ImageBlock]) -> RgbImage {
        let mut buffer = Rgb32FImage::new(self.image_width as u32, self.image_height as u32);
        for block in blocks {
            for (k, colour) in block.buffer.iter().enumerate() {
                let x = k % (block.xmax - block.xmin) + block.xmin;
                let y = k / (block.xmax - block.xmin) + block.ymin;
                *buffer.get_pixel_mut(x as u32, y as u32) = colour.into();
            }
        }
        buffer.convert()
    }
}

fn ray_colour(
    ray: Ray,
    world: &BVHNode,
    lights: Arc<Scene>,
    depth: usize,
    background: Texture,
) -> Colour {
    if depth == 0 {
        return Colour::BLACK;
    }
    if let Some(rec) = world.hit(&ray, Interval::new(0.001, f64::INFINITY)) {
        let colour_from_emission = rec.material.emit(&rec, rec.u, rec.v, rec.p);
        if let Some(scatter) = rec.material.scatter(ray, &rec) {
            match scatter.scattered {
                ScatterResult::SpecularRay(specular_ray) => {
                    ray_colour(specular_ray, world, lights, depth - 1, background)
                        .attenuate(&scatter.attenuation)
                }
                ScatterResult::PDF(pdf) => {
                    let mixture = if !lights.objects().is_empty() {
                        let lights_ptr = Arc::clone(&lights);
                        let light_pdf = HittablePDF::new(lights_ptr, rec.p);
                        MixturePDF::new(Arc::new(light_pdf), pdf)
                    } else {
                        MixturePDF::new(pdf.clone(), pdf)
                    };
                    let scattered = Ray::time_dependent(rec.p, mixture.generate(), ray.time);
                    let pdf_value = mixture.value(&scattered.direction);
                    let scattering_pdf = rec.material.scattering_pdf(ray, &rec, scattered);
                    let colour_from_scatter = scattering_pdf / pdf_value
                        * ray_colour(scattered, world, lights, depth - 1, background)
                            .attenuate(&scatter.attenuation);
                    colour_from_emission + colour_from_scatter
                }
            }
        } else {
            colour_from_emission
        }
    } else {
        let (u, v) = sphere_uv(ray.direction.normalize());
        background.value(u, v, Vec3::ZERO)
    }
}

struct ImageBlock {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    buffer: Vec<Colour>,
}

impl ImageBlock {
    fn write(&mut self, x: usize, y: usize, c: Colour) {
        self.buffer[(x - self.xmin) + (y - self.ymin) * (self.xmax - self.xmin)] = c;
    }
    fn new(xmin: usize, xmax: usize, ymin: usize, ymax: usize) -> Self {
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
            buffer: vec![Colour::BLACK; (xmax - xmin) * (ymax - ymin)],
        }
    }
}
