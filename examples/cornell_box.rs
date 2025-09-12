use std::{fs::File, io::stderr, rc::Rc};

use ray1week::{
    colour::Colour,
    material::{DiffuseLight, Lambertian},
    objects::{Collection, Cube, Quad},
    render::Camera,
    texture::SolidColour,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = Collection::new();

    let red = Rc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_colour(Colour::new(15.0, 15.0, 15.0)));

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

    world.add(Cube::new(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        white.clone(),
    ));
    world.add(Cube::new(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        white,
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
