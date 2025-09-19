use std::{io::stderr, sync::Arc};

use ray1week::prelude::*;

use ray1week::{
    material::{DiffuseLight, Lambertian},
    objects::{Cube, Quad},
    texture::SolidColour,
    transform::{Rotate, Translate},
};

fn main() -> Result<(), ImageError> {
    let mut world = Scene::new();

    let red = Lambertian::new(Colour::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Colour::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Colour::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_colour(Colour::new(15.0, 15.0, 15.0));

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

    let box1 = Cube::new(
        Point3::ZERO,
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Rotate::new(box1, 0.0, 15.0, 0.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    world.add(box1);

    let box2 = Cube::new(
        Point3::ZERO,
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Rotate::new(box2, 0.0, -18.0, 0.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));
    world.add(box2);

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
    renderer.render_to_file(&mut world, "examples/output/cornell_box.png", &mut stderr())
}
