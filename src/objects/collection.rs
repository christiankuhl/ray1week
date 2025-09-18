use super::{Hittable, IntoPrimitives, Object};
use crate::bounding_box::AaBb;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct Collection {
    pub objects: Vec<Object>,
    pub(crate) bbox: AaBb,
}

impl Collection {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AaBb::default(),
        }
    }

    pub fn with_objects(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self {
            objects: objects.iter().map(|o| Object(Arc::clone(o))).collect(),
            bbox: AaBb::default(),
        }
    }

    pub fn add(&mut self, object: impl IntoPrimitives) {
        for obj in object.primitives() {
            self.bbox = AaBb::enclosing(&self.bbox, &obj.bbox());
            self.objects.push(obj.clone());
        }
    }
}

impl IntoPrimitives for Collection {
    fn primitives(&self) -> Vec<Object> {
        self.objects.to_vec()
    }
}
