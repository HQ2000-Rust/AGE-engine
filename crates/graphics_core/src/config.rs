use crate::model::Model;
use std::path::PathBuf;
use wgpu::naga::FastHashMap;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Asset {
    Model(PathBuf),
    Image(PathBuf),
}

#[derive(Debug)]
pub struct StateConfig {
    pub color: wgpu::Color,
    pub models: FastHashMap<&'static str, String>,
    pub camera_speed: f32,
}

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            color: wgpu::Color::BLACK,
            models: FastHashMap::default(),
            //TODO: sane camera default
            camera_speed: 1.0,
        }
    }
}

/*impl StateConfig {
    pub fn new() -> StateConfig {
        Self {

        }
    }
}*/
