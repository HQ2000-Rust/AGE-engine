use cgmath::Point3;
use cgmath::prelude::*;
use std::marker::PhantomData;

//marker structs later

struct Sprite<T> {
    position: Point3<f32>,
    _marker: PhantomData<T>,
}
