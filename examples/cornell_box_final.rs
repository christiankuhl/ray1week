use std::{io::stderr, sync::Arc};

use ray1week::prelude::*;

use ray1week::{
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    objects::{Cube, Quad, Sphere},
    texture::SolidColour,
    transform::{Rotate, Translate},
};

fn main() -> Result<(), ImageError> {
    let mut world = Scene::new();

    let red = Arc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_colour(Colour::new(15.0, 15.0, 15.0)));
    let aluminium = Arc::new(Metal::new(Colour::new(0.8, 0.85, 0.88), 0.0));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::ZERO,
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    ));
    world.add(Sphere::new(Point3::new(100.0, 550.0, -400.0), 30.0, light));
    world.add(Quad::new(
        Point3::ZERO,
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let box1 = Cube::new(Point3::ZERO, Point3::new(165.0, 330.0, 165.0), aluminium);
    let box1 = Rotate::new(box1, 0.0, 15.0, 0.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    world.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, glass));

    let cam = Camera {
        aspect_ratio: 1.0,
        image_width: 600,
        background: Arc::new(SolidColour::new(Colour::BLACK)),
        vfov: 40.0,
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        ..Camera::default()
    };

    let renderer = cam.renderer(1000, 50);
    renderer.render_to_file(
        &mut world,
        "examples/output/cornell_box_final.png",
        &mut stderr(),
    )
}
