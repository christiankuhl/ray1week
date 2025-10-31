use std::io::stderr;

use ray1week::effects::BackFaceCulling;
use ray1week::prelude::*;
use ray1week::{objects::WavefrontObj, prelude::Scene, render::Camera};

fn main() -> Result<(), RenderError> {
    let mut world = Scene::new();
    let penger = WavefrontObj::from_file("examples/resources/penger.obj")?;
    let penger = penger.triangulate()?;
    //    let penger = BackFaceCulling::new(penger.triangulate()?);
    world.add(penger);
    let cam = Camera {
        image_width: 800,
        aspect_ratio: 1.0,
        lookfrom: Point3::new(4.0, 4.0, 6.0),
        lookat: Point3::new(0.0, 0.5, 0.0),
        vfov: 20.0,
        ..Camera::default()
    };
    let renderer = cam.renderer(50, 50);
    let img = renderer.render_with_filter(&mut world, BackFaceCulling, &mut stderr());
    img.save("examples/output/penger.png")?;
    Ok(())
}
