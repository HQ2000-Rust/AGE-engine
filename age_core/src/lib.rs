pub mod custom_events;
pub mod events;
pub mod game;
mod game_fn;
mod info;
mod scheduler;

use crate::events::EventHandle;
use crate::info::GameInfo;
use crate::scheduler::Scheduler;
//btw, I didn't choose the AnyMap because of Bevy but because it is just extremely convenient :-)
use crate::custom_events::builtin::ExitEvent;
use anymap::AnyMap;
use graphics_core::state::State;

pub(crate) mod aliases {
    use super::events::EventHandle;
    use super::info::GameInfo;
    use anymap::AnyMap;

    pub(crate) type BoxedGameFn = Box<dyn FnMut(&mut AnyMap, &mut EventHandle) + 'static>;
    pub(crate) type _MaxParamGameFn =
        Box<dyn FnMut(&mut AnyMap, &mut EventHandle, &GameInfo) + 'static>;
}
