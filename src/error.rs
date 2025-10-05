use std::error::Error;

use image::ImageError;

use crate::objects::WavefrontObjError;

pub enum RenderError {
    ImageError(ImageError),
    ObjectConstruction(WavefrontObjError),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ImageError(ref err) => write!(f, "{err}"),
            Self::ObjectConstruction(ref err) => write!(f, "{err}"),
        }
    }
}

impl std::fmt::Debug for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl Error for RenderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::ImageError(ref err) => Some(err),
            Self::ObjectConstruction(ref err) => Some(err),
        }
    }
}

impl From<ImageError> for RenderError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<WavefrontObjError> for RenderError {
    fn from(value: WavefrontObjError) -> Self {
        Self::ObjectConstruction(value)
    }
}
