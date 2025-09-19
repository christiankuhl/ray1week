use std::{io::stderr, sync::Arc};

use ray1week::prelude::*;

use ray1week::{
    material::{DiffuseLight, Lambertian},
    objects::{Quad, Sphere},
    texture::{NoiseTexture, SolidColour},
};

fn main() -> Result<(), ImageError> {
    let mut world = Scene::new();
    let marble = Arc::new(NoiseTexture::marble(4.0));
    let marble = Lambertian::from_texture(marble);
    let ground = Arc::new(NoiseTexture::plain(1.0));
    let ground = Lambertian::from_texture(ground);

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, marble));

    let difflight = DiffuseLight::from_colour(Colour::new(4.0, 4.0, 4.0));
    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        2.0 * Vec3::EX,
        2.0 * Vec3::EY,
        difflight.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        background: Arc::new(SolidColour::new(Colour::BLACK)),
        vfov: 20.0,
        lookfrom: Point3::new(26.0, 3.0, 6.0),
        lookat: Point3::new(0.0, 2.0, 0.0),
        ..Camera::default()
    };

    let renderer = cam.renderer(100, 50);
    renderer.render_to_file(
        &mut world,
        "examples/output/diffuse_light.png",
        &mut stderr(),
    )
}
