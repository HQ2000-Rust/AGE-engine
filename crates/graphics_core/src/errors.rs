use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum StateCreationError {
    RequestAdapterError(wgpu::RequestAdapterError),
    RequestDeviceError(wgpu::RequestDeviceError),
    ModelError(ModelError),
}

#[derive(Debug)]
pub enum ModelError {
    ImageError(image::ImageError),
    IoError(std::io::Error),
    LoadError(tobj::LoadError),
    TextureError(TextureError),
}

#[derive(Debug)]
pub enum TextureError {
    ImageError(image::ImageError),
    IoError(std::io::Error),
}

impl Error for StateCreationError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(match *self {
            StateCreationError::RequestAdapterError(ref err) => err,
            StateCreationError::RequestDeviceError(ref err) => err,
            StateCreationError::ModelError(ref err) => err,
        })
    }
}

impl Display for StateCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //TODO!: proper Display implementation later
        write!(f, "{:?}", self)
    }
}

impl Error for ModelError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(match self {
            ModelError::LoadError(err) => err,
            ModelError::IoError(err) => err,
            ModelError::TextureError(err) => err,
            ModelError::ImageError(err) => err,
        })
    }
}

impl Display for ModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //TODO!: proper Display implementation later
        write!(f, "{:?}", self)
    }
}

impl Error for TextureError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(match self {
            TextureError::ImageError(err) => err,
            TextureError::IoError(err) => err,
        })
    }
}

impl Display for TextureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //TODO!: proper Display implementation later
        write!(f, "{:?}", self)
    }
}
