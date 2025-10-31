use std::sync::Arc;

use crate::{
    objects::{Hittable, Interval},
    ray::Ray,
    render::Renderer,
};

#[derive(Debug)]
pub struct BackFaceCulling;

pub trait RenderFilter {
    fn filter(&self, renderer: &Renderer, objects: &mut Vec<Arc<dyn Hittable>>);
}

pub(crate) struct TrivialFilter;

impl RenderFilter for TrivialFilter {
    fn filter(&self, _renderer: &Renderer, _objects: &mut Vec<Arc<dyn Hittable>>) {}
}

impl RenderFilter for BackFaceCulling {
    fn filter(&self, renderer: &Renderer, objects: &mut Vec<Arc<dyn Hittable>>) {
        objects.retain(|obj| {
            let mut shows_front = false;
            let bbox = obj.bbox();
            let test_points = bbox.test_points(8);
            for p in test_points {
                let r = Ray::new(renderer.center, p - renderer.center);
                if let Some(rec) = obj.hit(&r, Interval::new(0.001, f64::INFINITY)) {
                    shows_front |= rec.front_face;
                }
            }
            shows_front
        });
    }
}
