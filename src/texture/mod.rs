mod perlin;
mod textures;

pub use textures::*;

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{colour::Colour, linalg::Point3};

pub trait Textured: std::fmt::Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour;
}

#[derive(Debug)]
pub struct Texture(Arc<dyn Textured>);

impl Texture {
    pub(crate) fn new(texture: Arc<dyn Textured>) -> Self {
        Self(texture)
    }
}

impl Deref for Texture {
    type Target = Arc<dyn Textured>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Texture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
