use super::{IntoPrimitives, Object};
use crate::bounding_box::AaBb;

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

    pub fn with_objects(objects: Vec<Object>) -> Self {
        Self {
            objects,
            bbox: AaBb::default(),
        }
    }

    pub fn add(&mut self, object: impl IntoPrimitives) {
        for obj in object.primitives() {
            self.bbox = AaBb::enclosing(&self.bbox, &obj.bbox());
            self.objects.push(obj.clone());
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.objects.extend(other.objects);
    }
}

impl IntoPrimitives for Collection {
    fn primitives(&self) -> Vec<Object> {
        self.objects.to_vec()
    }
}

impl IntoIterator for Collection {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.into_iter()
    }
}
