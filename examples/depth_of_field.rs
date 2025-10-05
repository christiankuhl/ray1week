use std::io::stderr;

use ray1week::prelude::*;

use ray1week::{
    material::{Dielectric, Lambertian, Metal},
    objects::Sphere,
};

fn main() -> Result<(), RenderError> {
    // Scene setup
    let mut world = Scene::new();
    let material_ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Colour::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let air_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.1);
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
        vfov: 20.0,
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        // Set depth of field
        defocus_angle: 5.0,
        focus_dist: 3.4,
        // Rest remains at default values
        ..Camera::default()
    };

    // Render to file
    let renderer = camera.renderer(50, 10);
    renderer.render_to_file(&mut world, "examples/output/dof.png", &mut stderr())
}
