use std::io::stderr;

use ray1week::prelude::*;

use ray1week::{material::Lambertian, objects::Sphere, texture::NoiseTexture};

fn main() -> Result<(), RenderError> {
    let mut world = Scene::new();

    let ground = NoiseTexture::plain(4.0);
    let ground = Lambertian::from_texture(ground);

    let marble = NoiseTexture::marble(4.0);

    let marble = Lambertian::from_texture(marble);
    let turbulence = NoiseTexture::turbulence(1.0, 7);
    let turbulence = Lambertian::from_texture(turbulence);

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 2.0, -2.5), 2.0, marble));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 2.5), 2.0, turbulence));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Camera::default()
    };

    let renderer = cam.renderer(100, 50);

    renderer.render_to_file(
        &mut world,
        "examples/output/perlin_spheres.png",
        &mut stderr(),
    )
}
