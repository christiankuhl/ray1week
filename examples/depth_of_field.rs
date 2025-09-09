use ray1week::colour::Colour;
use ray1week::material::{Dielectric, Lambertian, Metal};
use ray1week::objects::{Collection, Sphere};
use ray1week::render::Camera;
use ray1week::vec3::Point3;

use std::fs::File;
use std::io::stderr;
use std::rc::Rc;

fn main() {
    // Scene setup
    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let air_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.1));
    let mut world = Collection::new();
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
    let mut f = File::create("dof.ppm").unwrap();
    renderer.render(&mut world, &mut f, &mut stderr().lock());
}
