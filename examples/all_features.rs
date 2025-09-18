use std::{io::stderr, sync::Arc};

use ray1week::prelude::*;

use ray1week::{
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    objects::{Cube, MovingSphere, Quad, Sphere},
    texture::{ImageTexture, NoiseTexture, SolidColour},
    transform::{Rotate, Translate},
    volumetrics::ConstantMedium,
};

fn main() -> Result<(), ImageError> {
    let mut boxes1 = Collection::new();
    let ground = Arc::new(Lambertian::new(Colour::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = fastrand::f64() * 100.0 + 1.0;
            let z1 = z0 + w;
            boxes1.add(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    let mut world = Scene::new();
    world.add(boxes1);

    let light = Arc::new(DiffuseLight::from_colour(7.0 * Colour::WHITE));
    world.add(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        300.0 * Vec3::EX,
        265.0 * Vec3::EZ,
        light,
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + 30.0 * Vec3::EX;
    let sphere_material = Arc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.1)));
    world.add(MovingSphere::new(center1, center2, 50.0, sphere_material));

    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0)),
    ));

    let boundary = Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.add(boundary.clone());
    world.add(ConstantMedium::isotropic(
        boundary.clone(),
        0.2,
        Colour::new(0.2, 0.4, 0.9),
    ));
    let boundary = Sphere::new(Point3::ZERO, 5000.0, Arc::new(Dielectric::new(1.5)));
    world.add(ConstantMedium::isotropic(
        boundary.clone(),
        0.0001,
        Colour::WHITE,
    ));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(
        ImageTexture::new("examples/resources/earthmap.jpg").unwrap(),
    )));
    world.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = Arc::new(NoiseTexture::plain(0.2));
    world.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(pertext)),
    ));

    let mut boxes2 = Collection::new();
    let white = Arc::new(Lambertian::new(0.73 * Colour::WHITE));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Sphere::new(Point3::random(0.0, 165.0), 10.0, white.clone()));
    }

    world.add(Translate::new(
        Rotate::new(boxes2, 0.0, 15.0, 0.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let cam = Camera {
        aspect_ratio: 1.0,
        image_width: 800,
        background: Arc::new(SolidColour::new(Colour::BLACK)),
        vfov: 40.0,
        lookfrom: Point3::new(478.0, 278.0, -600.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        defocus_angle: 0.0,
        ..Camera::default()
    };

    let renderer = cam.renderer(10, 50);
    renderer.render(&mut world, "all_features_test.png", &mut stderr())
}
