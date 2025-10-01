use crate::commands::CommandHandle;
use crate::events::EventHandle;
use anymap::{AnyMap, Map};
pub(crate) type BoxedGameFn =
    Box<dyn FnMut(&mut Commands, &mut SendAnyMap, &mut EventHandle) + 'static + Send>;
pub(crate) type _MaxParamGameFn = Box<dyn FnMut(&mut Commands, &mut AnyMap, &mut Events) + 'static>;
pub type Commands = CommandHandle;
pub type Events = EventHandle;

pub(crate) type SendAnyMap = Map<dyn anymap::any::Any + Send>;
