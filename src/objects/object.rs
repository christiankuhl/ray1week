use std::{ops::Deref, sync::Arc};

use super::Hittable;

#[derive(Debug)]
pub struct Object(pub Arc<dyn Hittable>);

impl Object {
    pub fn new(object: Arc<dyn Hittable>) -> Self {
        Self(Arc::clone(&object))
    }
}

impl Deref for Object {
    type Target = Arc<dyn Hittable>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Self(Arc::clone(self))
    }
}

pub trait IntoPrimitives {
    fn primitives(&self) -> Vec<Object>;
}

impl IntoPrimitives for Object {
    fn primitives(&self) -> Vec<Object> {
        vec![self.clone()]
    }
}
