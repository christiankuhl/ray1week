use std::{io::stderr, sync::Arc};

use ray1week::prelude::*;

use ray1week::{
    material::{Dielectric, Lambertian, Metal},
    objects::Sphere,
};

fn main() -> Result<(), ImageError> {
    // Scene setup
    let mut world = Scene::new();
    let material_ground = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let air_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.1));
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
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, air_bubble));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let camera = Camera {
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        // Zoom out
        vfov: 90.0,
        ..Camera::default()
    };

    // Zoomed in view
    let mut zoom = camera.clone();
    zoom.vfov = 20.0;

    // Render to file
    let renderer = camera.renderer(50, 10);
    renderer
        .render_to_file(&mut world, "examples/output/zoom_out.png", &mut stderr())
        .unwrap();
    let renderer = zoom.renderer(50, 10);
    renderer.render_to_file(&mut world, "examples/output/zoom_in.png", &mut stderr())
}
