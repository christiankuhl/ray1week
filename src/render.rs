use std::f64;
use std::io::Write;
use std::rc::Rc;

use crate::bounding_box::BVHNode;
use crate::colour::Colour;
use crate::objects::{Collection, Hittable, Interval, sphere_uv};
use crate::random::{random_unit_disk, sample_square};
use crate::ray::Ray;
use crate::texture::{SkyTexture, Texture};
use crate::vec3::{Point3, Vec3};

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
    pub background: Rc<dyn Texture>,
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
    background: Rc<dyn Texture>,
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
            background: Rc::new(SkyTexture::default()),
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

    pub fn render(&self, world: &mut Collection, f: &mut impl Write, p: &mut impl Write) {
        let bvh = BVHNode::new(&mut world.objects);
        let pixel_samples_scale = 1.0 / (self.sqrt_spp * self.sqrt_spp) as f64;
        writeln!(f, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();
        for y in 0..self.image_height {
            let remaining = self.image_height - y;
            write!(
                p,
                "\rScanlines remaining: {remaining} / {}...",
                self.image_height
            )
            .unwrap();
            for x in 0..self.image_width {
                let mut c = Colour::new(0.0, 0.0, 0.0);
                for sj in 0..self.sqrt_spp {
                    for si in 0..self.sqrt_spp {
                        let r = self.get_ray(x, y, si, sj);
                        c += ray_colour(r, &bvh, self.max_depth, self.background.clone());
                    }
                }
                c = pixel_samples_scale * c;
                writeln!(f, "{}", c.ppm()).unwrap();
            }
        }
        writeln!(
            p,
            "\rDone.                                                                      "
        )
        .unwrap();
    }
}

fn ray_colour<'a>(
    ray: Ray,
    world: &'a BVHNode,
    depth: usize,
    background: Rc<dyn Texture + 'a>,
) -> Colour {
    if depth == 0 {
        return Colour::BLACK;
    }
    if let Some(rec) = world.hit(&ray, Interval::new(0.001, f64::INFINITY)) {
        let colour_from_emission = rec.material.emit(rec.u, rec.v, rec.p);
        if let Some(scatter) = rec.material.scatter(ray, &rec) {
            let colour_from_scatter = ray_colour(scatter.ray, world, depth - 1, background)
                .attenuate(&scatter.attenuation);
            colour_from_emission + colour_from_scatter
        } else {
            colour_from_emission
        }
    } else {
        let (u, v) = sphere_uv(ray.direction.normalize());
        background.value(u, v, Vec3::ZERO)
    }
}
