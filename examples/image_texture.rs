use std::io::stderr;

use ray1week::prelude::*;

use ray1week::{material::Lambertian, objects::Sphere, texture::ImageTexture};

fn main() -> Result<(), ImageError> {
    let mut world = Scene::new();

    let map = ImageTexture::new("examples/resources/earthmap.jpg").unwrap();
    let material = Lambertian::from_texture(map);

    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, material));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vfov: 20.0,
        lookfrom: Point3::new(0.0, 0.0, 12.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Camera::default()
    };

    let renderer = cam.renderer(100, 50);

    renderer.render_to_file(&mut world, "examples/output/globe.png", &mut stderr())
}
