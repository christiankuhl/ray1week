use std::io::stderr;

use ray1week::material::Metal;
use ray1week::prelude::*;
use ray1week::{objects::WavefrontObj, prelude::Scene, render::Camera};

fn main() -> Result<(), RenderError> {
    let mut world = Scene::new();
    let blue = Metal::new(Colour::new(0.15, 0.15, 0.73), 0.1);
    let teapot = WavefrontObj::from_file("examples/resources/teapot.obj")?;
    let teapot = teapot.triangulate_with_material(blue);
    world.add(teapot);
    let cam = Camera {
        image_width: 800,
        lookfrom: Point3::new(4.0, 4.0, 6.0),
        lookat: Point3::new(0.0, 1.0, 0.0),
        vfov: 45.0,
        ..Camera::default()
    };
    let renderer = cam.renderer(50, 50);
    renderer.render_to_file(&mut world, "examples/output/utah_teapot.png", &mut stderr())
}
