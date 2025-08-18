use crate::commands::CommandHandle;
use crate::events::EventHandle;
use anymap::AnyMap;
pub(crate) type BoxedGameFn =
    Box<dyn FnMut(&mut Commands, &mut AnyMap, &mut EventHandle) + 'static>;
pub(crate) type _MaxParamGameFn = Box<dyn FnMut(&mut Commands, &mut AnyMap, &mut Events) + 'static>;
pub type Commands = CommandHandle;
pub type Events = EventHandle;
