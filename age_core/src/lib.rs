pub mod events;
mod game_fn;
mod info;
mod scheduler;

use crate::events::{EventHandle, EventQueueMode};
use crate::info::GameInfo;
use crate::scheduler::Scheduler;
//btw, I didn't choose the AnyMap because of Bevy but because it is just extremely convenient :-)
use crate::events::special_events::{ExitEvent};
use anymap::AnyMap;
use graphics_core::state::State;



pub(crate) mod aliases {
    use super::events::EventHandle;
    use super::info::GameInfo;
    use anymap::AnyMap;
    
    pub(crate) type BoxedGameFn=Box<dyn FnMut(&mut AnyMap,&mut EventHandle)+'static>;
    pub(crate) type _MaxParamGameFn = Box<dyn FnMut(&mut AnyMap, &mut EventHandle, &GameInfo) + 'static>;
}

pub struct Game {
    resources: AnyMap,
    scheduler: Scheduler,
    events: EventHandle,
    graphics_state: Option<State>,
    //?
}

impl Default for Game {
    fn default() -> Self {
        Self {
            resources: AnyMap::new(),
            scheduler: Scheduler::new_empty(),
            //make the mode customizable later!!
            events: EventHandle::new(EventQueueMode::StoreOnce),
            graphics_state: None,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_once(mut self, once: impl FnMut(&mut AnyMap,&mut EventHandle)+'static) -> Self {
        self.scheduler.add_once(Box::new(once));
        self
    }

    pub fn add_update(mut self, update: impl FnMut(&mut AnyMap,&mut EventHandle)+'static) -> Self {
        self.scheduler.add_update(Box::new(update));
        self
    }

    pub fn run(mut self) {
        self.scheduler
            .setup(&mut self.resources, &mut self.events);
        self.events.setup();

        while !self.events.contains(&ExitEvent) {
            self.scheduler
                .update(&mut self.resources, &mut self.events);
            self.events.update();
        }
    }
}
