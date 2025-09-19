use std::io::stderr;

use ray1week::prelude::*;

use ray1week::material::{Dielectric, Lambertian, Metal};
use ray1week::objects::Sphere;

fn main() -> Result<(), ImageError> {
    // Scene setup
    let material_ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Colour::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_right = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.1);

    let mut world = Scene::new();
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));
    //    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 1.0, light));
    let camera = Camera::default();

    // Render to file
    let renderer = camera.renderer(100, 10);
    renderer.render_to_file(&mut world, "examples/output/basic.png", &mut stderr())
}
