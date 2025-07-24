use std::collections::VecDeque;

#[derive(Debug)]
pub struct EventHandle {
    events: VecDeque<Box<dyn Event>>,
    queue: VecDeque<Box<dyn Event>>,
    mode: EventQueueMode,
}

impl EventHandle {
    fn new(mode: EventQueueMode) -> Self {
        Self {
            events: VecDeque::new(),
            queue: VecDeque::new(),
            mode,
        }
    }
}

#[derive(Default, Debug)]
pub enum EventQueueMode {
    //if this mode is activated, the developer is responsible for consuming the events
    //so they don't accumulate until an overflow happens
    StoreUntilConsume,
    //default later for quickstart
    #[default]
    StoreOnce,
}

pub trait Event {

}
