use std::io::stderr;

use ray1week::material::Metal;
use ray1week::prelude::*;
use ray1week::{objects::WavefrontObj, prelude::Scene, render::Camera};

fn main() -> Result<(), RenderError> {
    let mut world = Scene::new();
    let blue = Metal::new(Colour::new(0.15, 0.15, 0.73), 0.1);
    let penger = WavefrontObj::from_file("examples/resources/penger.obj")?;
    let penger = penger.triangulate(blue);
    world.add(penger);
    let cam = Camera {
        image_width: 800,
        aspect_ratio: 1.0,
        lookfrom: Point3::new(4.0, 4.0, 6.0),
        lookat: Point3::new(0.0, 1.0, 0.0),
        vfov: 45.0,
        ..Camera::default()
    };
    let renderer = cam.renderer(50, 50);
    renderer.render_to_file(&mut world, "examples/output/penger.png", &mut stderr())
}
