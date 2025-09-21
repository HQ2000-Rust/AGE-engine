use cgmath::Point3;
use cgmath::prelude::*;

mod impls;

//TODO: other commands also here!!
pub struct SetPos(pub u32, pub Point3<f32>);

pub struct SetPosRel(pub u32, pub Point3<f32>);

//invalid id => no change
pub struct SetModel(pub u32, pub &'static str);
