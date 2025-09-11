use std::{fs::File, io::stderr, rc::Rc};

use ray1week::{
    colour::Colour,
    material::Lambertian,
    objects::{Collection, Quad},
    render::Camera,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = Collection::new();

    // Materials
    let left_red = Rc::new(Lambertian::new(Colour::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::new(Colour::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::new(Colour::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new(Colour::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::new(Colour::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let cam = Camera {
        aspect_ratio: 1.0,
        image_width: 400,
        vfov: 80.0,
        lookfrom: Point3::new(0.0, 0.0, 9.0),
        lookat: Point3::ZERO,
        up: Vec3::EY,
        defocus_angle: 0.0,
        ..Camera::default()
    };

    let renderer = cam.renderer(100, 50);
    let mut file = File::create("quads.ppm").unwrap();
    renderer.render(&mut world, &mut file, &mut stderr().lock());
}
