use std::{io::stderr, sync::Arc};

use ray1week::prelude::*;

use ray1week::{material::Lambertian, objects::Quad};

fn main() -> Result<(), ImageError> {
    let mut world = Scene::new();

    // Materials
    let left_red = Arc::new(Lambertian::new(Colour::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Colour::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Colour::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Colour::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Colour::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let cam = Camera {
        aspect_ratio: 1.0,
        image_width: 400,
        vfov: 80.0,
        lookfrom: Point3::new(0.0, 0.0, 9.0),
        lookat: Point3::ZERO,
        up: Vec3::EY,
        defocus_angle: 0.0,
        ..Camera::default()
    };

    let renderer = cam.renderer(100, 50);
    renderer.render(&mut world, "examples/output/quads.png", &mut stderr())
}
