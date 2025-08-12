#[derive(Debug)]
pub enum StateCreationError {
    RequestAdapterError(wgpu::RequestAdapterError),
    RequestDeviceError(wgpu::RequestDeviceError),
    ModelError(ModelError),
}

#[derive(Debug)]
pub enum TextureError {
    ImageError(image::ImageError),
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum ModelError {
    ImageError(image::ImageError),
    IoError(std::io::Error),
    LoadError(tobj::LoadError),
    TextureError(TextureError),
}
