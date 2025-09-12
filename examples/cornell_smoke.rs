use std::{fs::File, io::stderr, rc::Rc};

use ray1week::{
    colour::Colour,
    material::{DiffuseLight, Lambertian},
    objects::{Collection, Cube, Quad},
    render::Camera,
    texture::SolidColour,
    transform::{Rotate, Translate},
    vec3::{Point3, Vec3},
    volumetrics::ConstantMedium,
};

fn main() {
    let mut world = Collection::new();

    let red = Rc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_colour(Colour::new(7.0, 7.0, 7.0)));

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
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
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

    let box1 = Rc::new(Cube::new(
        Point3::ZERO,
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Rc::new(Rotate::new(box1, 0.0, 15.0, 0.0));
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));

    let box2 = Rc::new(Cube::new(
        Point3::ZERO,
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Rc::new(Rotate::new(box2, 0.0, -18.0, 0.0));
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

    world.add(ConstantMedium::isotropic(
        Rc::new(box1),
        0.01,
        Colour::BLACK,
    ));
    world.add(ConstantMedium::isotropic(
        Rc::new(box2),
        0.01,
        Colour::WHITE,
    ));

    let cam = Camera {
        aspect_ratio: 1.0,
        image_width: 600,
        background: Rc::new(SolidColour::new(Colour::BLACK)),
        vfov: 40.0,
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        ..Camera::default()
    };

    let renderer = cam.renderer(200, 50);
    let mut file = File::create("cornell_box.ppm").unwrap();
    renderer.render(&mut world, &mut file, &mut stderr().lock());
}
