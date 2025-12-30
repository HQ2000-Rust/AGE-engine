use anymap::AnyMap;
use anymap::any::{IntoBox, UncheckedAnyExt};
use std::any::{Any, TypeId};
use std::fmt::Debug;

use pyo3::prelude::*;

#[pyclass]
pub struct EventHandle {
    events: Option<AnyMap>,
    queue: Option<AnyMap>,
}

#[pymethods]
impl EventHandle {
    #[new]
    pub fn new() -> Self {
        Self {
            events: Some(AnyMap::new()),
            queue: Some(AnyMap::new()),
        }
    }
    pub fn update(&mut self) {
        self.events = self.queue.take();
        self.queue = Some(AnyMap::new());
    }

    pub fn get<T: 'static>(&mut self) -> Option<&T> {
        //the turbofish is for showing that this method always uses the corresponding AnyMap::insert method
        self.events.as_ref().expect("Always Some(_)").get::<T>()
    }

    pub fn contains<T: 'static>(&mut self) -> bool {
        //same
        self.events
            .as_ref()
            .expect("Always Some(_)")
            .contains::<T>()
    }

    pub fn add<T: 'static>(&mut self, event: T) -> Option<T> {
        //same
        self.queue
            .as_mut()
            .expect("Always Some(_)")
            .insert::<T>(event)
    }
}
