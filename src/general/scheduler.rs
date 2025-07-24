use wgpu::naga::FastHashMap;
use crate::general::data::{DataTrait, GameData};

pub(crate) struct Scheduler {
    once: Vec<Box<dyn FnOnce(&mut FastHashMap<&'static str, GameData>)>>,
    update: Vec<Box<dyn FnMut(&mut FastHashMap<&'static str, GameData>, EventHandle>)>>,
}

impl Scheduler {
    fn new(once: Vec<Box<dyn FnOnce(&mut FastHashMap<&'static str, GameData>)>>,
           update: Vec<Box<dyn FnMut(&mut FastHashMap<&'static str, GameData>)>>) -> Self {
        Self {
            once,
            update,
        }
    }
    fn setup(&self, resources: &mut FastHashMap<&'static str, GameData>) {
        self.once.iter().for_each(|f| f(resources));
    }
    fn update(&self, resources: &mut FastHashMap<&'static str, GameData>) {
        self.update.iter().for_each(|f| f(resources));
    }
}
