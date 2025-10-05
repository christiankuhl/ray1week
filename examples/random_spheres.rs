use std::io::stderr;

use ray1week::prelude::*;

use ray1week::{
    material::{Dielectric, Lambertian, Material, Metal},
    objects::Sphere,
};

fn main() -> Result<(), RenderError> {
    let mut world = Scene::new();
    let mut materials: Vec<Material> = Vec::new();
    let ground_material = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Colour::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut centers = Vec::new();

    make_random_spheres(&mut materials, &mut centers);

    for (idx, &center) in centers.iter().enumerate() {
        world.add(Sphere::new(center, 0.2, materials[idx].clone()));
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

    let renderer = cam.renderer(50, 50);
    renderer.render_to_file(
        &mut world,
        "examples/output/random_spheres.png",
        &mut stderr(),
    )
}

fn make_random_spheres(materials: &mut Vec<Material>, centers: &mut Vec<Point3>) {
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
                    materials.push(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::new(
                        fastrand::f64() / 2.0 + 0.5,
                        fastrand::f64() / 2.0 + 0.5,
                        fastrand::f64() / 2.0 + 0.5,
                    );
                    let fuzz = 0.5 * fastrand::f64();
                    materials.push(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    materials.push(Dielectric::new(1.5));
                }
                centers.push(center);
            }
        }
    }
}
