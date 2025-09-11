use std::{fs::File, io::stderr, rc::Rc};

use ray1week::{
    material::Lambertian,
    objects::{Collection, Sphere},
    render::Camera,
    texture::NoiseTexture,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = Collection::new();

    let ground = Rc::new(NoiseTexture::plain(4.0));
    let ground = Rc::new(Lambertian::from_texture(ground));

    let marble = Rc::new(NoiseTexture::marble(4.0));
    let marble = Rc::new(Lambertian::from_texture(marble));

    let turbulence = Rc::new(NoiseTexture::turbulence(1.0, 7));
    let turbulence = Rc::new(Lambertian::from_texture(turbulence));

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 2.0, -2.5), 2.0, marble));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 2.5), 2.0, turbulence));

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

    let mut f = File::create("perlin_spheres.ppm").unwrap();
    renderer.render(&mut world, &mut f, &mut stderr().lock());
}
