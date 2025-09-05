use ray1week::colour::Colour;
use ray1week::material::{Dielectric, Lambertian, Metal};
use ray1week::objects::{Collection, Sphere};
use ray1week::render::Camera;
use ray1week::vec3::Point3;

use std::fs::File;
use std::io::stderr;

fn main() {
    // Scene setup
    let material_ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Colour::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let air_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.1);
    let mut world = Collection::new();
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        &material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    ));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, &air_bubble));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    ));

    let camera = Camera {
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        // Zoom out
        vfov: 90.0,
        ..Camera::default()
    };

    // Zoomed in view
    let mut zoom = camera;
    zoom.vfov = 20.0;

    // Render to file
    let renderer = camera.renderer(50, 10);
    let mut f = File::create("zoom_out.ppm").unwrap();
    renderer.render(&world, &mut f, &mut stderr().lock());
    let renderer = zoom.renderer(50, 10);
    let mut f = File::create("zoom_in.ppm").unwrap();
    renderer.render(&world, &mut f, &mut stderr().lock());
}
