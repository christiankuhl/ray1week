use std::{fs::File, io::stderr, rc::Rc};

use ray1week::{
    material::Lambertian,
    objects::{Collection, Sphere},
    render::Camera,
    texture::ImageTexture,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = Collection::new();

    let map = Rc::new(ImageTexture::new("examples/resources/earthmap.jpg").unwrap());
    let material = Rc::new(Lambertian::from_texture(map));

    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, material));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vfov: 20.0,
        lookfrom: Point3::new(0.0, 0.0, 12.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Camera::default()
    };

    let renderer = cam.renderer(100, 50);

    let mut f = File::create("globe.ppm").unwrap();
    renderer.render(&mut world, &mut f, &mut stderr().lock());
}
