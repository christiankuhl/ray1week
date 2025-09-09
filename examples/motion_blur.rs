use ray1week::colour::Colour;
use ray1week::material::Lambertian;
use ray1week::material::{Dielectric, Metal, Scatter};
use ray1week::objects::{Collection, MovingSphere, Sphere};
use ray1week::render::Camera;
use ray1week::vec3::{Point3, Vec3};

use std::collections::HashMap;
use std::fs::File;
use std::io::stderr;
use std::rc::Rc;

const BOUNDARY: i32 = 11;

fn main() {
    let mut materials: Vec<Rc<dyn Scatter>> = Vec::new();
    let ground_material = Rc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    let material1 = Rc::new(Dielectric::new(1.5));
    let material2 = Rc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    let material3 = Rc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    let mut world = Collection::new();
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut centers = Vec::new();
    let mut moving = HashMap::new();

    make_random_spheres(&mut materials, &mut centers, &mut moving);

    for (idx, &center) in centers.iter().enumerate() {
        let a = center.x.floor() as i32;
        let b = center.z.floor() as i32;
        if let Some(point) = moving.get(&(a, b)) {
            world.add(MovingSphere::new(
                center,
                *point,
                0.2,
                materials[idx].clone(),
            ));
        } else {
            world.add(Sphere::new(center, 0.2, materials[idx].clone()));
        }
    }

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

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
    renderer.render(&mut world, &mut f, &mut stderr().lock());
}

fn make_random_spheres(
    materials: &mut Vec<Rc<dyn Scatter>>,
    centers: &mut Vec<Point3>,
    moving: &mut HashMap<(i32, i32), Point3>,
) {
    for a in -BOUNDARY..BOUNDARY {
        for b in -BOUNDARY..BOUNDARY {
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
                    materials.push(Rc::new(Lambertian::new(albedo)));
                    moving.insert((a, b), center + Vec3::new(0.0, 0.5 * fastrand::f64(), 0.0));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::new(
                        fastrand::f64() / 2.0 + 0.5,
                        fastrand::f64() / 2.0 + 0.5,
                        fastrand::f64() / 2.0 + 0.5,
                    );
                    let fuzz = 0.5 * fastrand::f64();
                    materials.push(Rc::new(Metal::new(albedo, fuzz)));
                } else {
                    // glass
                    materials.push(Rc::new(Dielectric::new(1.5)));
                }
                centers.push(center);
            }
        }
    }
}
