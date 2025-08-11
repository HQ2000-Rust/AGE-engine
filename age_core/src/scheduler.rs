use super::events::EventHandle;
use super::info::GameInfo;
use anymap::AnyMap;
use std::collections::{HashMap, VecDeque};
use crate::aliases::BoxedGameFn;

#[derive(Default)]
pub(crate) struct Scheduler {
    once: VecDeque<BoxedGameFn>,
    update: VecDeque<BoxedGameFn>,
}

impl Scheduler {
    pub(crate) fn new_from(
        once: VecDeque<BoxedGameFn>,
        update: VecDeque<BoxedGameFn>,
    ) -> Self {
        Self { once, update }
    }
    pub(crate) fn new_empty() -> Self {
        Default::default()
    }
    pub(crate) fn add_once(&mut self, once: BoxedGameFn) {
        self.once.push_back(once);
    }
    pub(crate) fn add_update(&mut self, update: BoxedGameFn) {
        self.update.push_back(update);
    }
    pub(crate) fn setup(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        for f in self.once.iter_mut() {
            f(resources, event_handle);
        }
    }
    pub(crate) fn update(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        for f in self.update.iter_mut() {
            f(resources, event_handle);
        }
    }
}
