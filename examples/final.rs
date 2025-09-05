use ray1week::colour::Colour;
use ray1week::material::Lambertian;
use ray1week::material::{Dielectric, Metal, Scatter};
use ray1week::objects::{Collection, Sphere};
use ray1week::render::Camera;
use ray1week::vec3::Point3;

use std::fs::File;
use std::io::stderr;

fn main() {
    let mut materials: Vec<Box<dyn Scatter>> = Vec::new();
    let ground_material = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Colour::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);
    let mut world = Collection::new();
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ));

    let mut centers = Vec::new();

    make_random_spheres(&mut materials, &mut centers);

    for (idx, &center) in centers.iter().enumerate() {
        world.add(Sphere::new(center, 0.2, materials[idx].as_ref()));
    }

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &material1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &material2));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &material3));

    let cam = Camera {
        image_width: 800,
        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
        ..Camera::default()
    };

    let mut f = File::create("img.ppm").unwrap();
    let renderer = cam.renderer(50, 50);
    renderer.render(&world, &mut f, &mut stderr().lock());
}

fn make_random_spheres(materials: &mut Vec<Box<dyn Scatter>>, centers: &mut Vec<Point3>) {
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f64();
            let center = Point3::new(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::random().attenuate(&Colour::random());
                    materials.push(Box::new(Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::new(
                        fastrand::f64() / 2.0 + 0.5,
                        fastrand::f64() / 2.0 + 0.5,
                        fastrand::f64() / 2.0 + 0.5,
                    );
                    let fuzz = 0.5 * fastrand::f64();
                    materials.push(Box::new(Metal::new(albedo, fuzz)));
                } else {
                    // glass
                    materials.push(Box::new(Dielectric::new(1.5)));
                }
                centers.push(center);
            }
        }
    }
}
