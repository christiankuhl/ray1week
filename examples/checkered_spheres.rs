use std::{fs::File, io::stderr, rc::Rc};

use ray1week::{
    colour::Colour,
    material::Lambertian,
    objects::{Collection, Sphere},
    render::Camera,
    texture::{CheckerTexture, UVSlice},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = Collection::new();

    let spatial_checker = Rc::new(CheckerTexture::solid(
        0.032,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let uv_checker = Rc::new(UVSlice::new(spatial_checker, 0.0));
    let material = Rc::new(Lambertian::from_texture(uv_checker));

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, material));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Camera::default()
    };

    let renderer = cam.renderer(100, 50);

    let mut f = File::create("checkered_spheres.ppm").unwrap();
    renderer.render(&mut world, &mut f, &mut stderr().lock());
}
