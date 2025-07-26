use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use crate::general::data::GameData;
use crate::general::events::EventHandle;
use crate::general::aliases::*;


pub(crate) struct Scheduler {
    once: VecDeque<SetupFn>,
    update: VecDeque<UpdateFn>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            once: VecDeque::new(),
            update: VecDeque::new(),
        }
    }
}

impl Scheduler {
    pub(crate) fn new(once: VecDeque<SetupFn>,
           update: VecDeque<UpdateFn>) -> Self {
        Self {
            once,
            update,
        }
    }
    pub(crate) fn new_empty() -> Self {
        Default::default()
    }
    pub(crate) fn add_once(&mut self,once: impl FnMut(&mut Resources, &mut EventHandle) + 'static) {
        self.once.push_back(Box::new(once));
    }
    pub(crate) fn add_update(&mut self,once: impl FnMut(&mut Resources, &mut EventHandle) + 'static) {
        self.update.push_back(Box::new(once));
    }
    pub(crate) fn setup(&mut self, resources: &mut HashMap<&'static str, GameData>, event_handle: &mut EventHandle ) {
        //I hope I can remove that .collect() later...just have no idea how
        for f in self.once.iter_mut() {
            f(resources, event_handle);
        }
    }
    pub(crate) fn update(&mut self, resources: &mut HashMap<&'static str, GameData>, event_handle: &mut EventHandle) {
        for f in self.update.iter_mut() {
            f(resources, event_handle);
        }
    }
}
