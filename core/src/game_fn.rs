//all the dirty impl work
use anymap::AnyMap;
use crate::events::EventHandle;
use crate::info::GameInfo;

//2^3=8 possibilties

pub trait GameFn: 'static {
    fn exec(&mut self, resources: &mut AnyMap, event_handle: &mut EventHandle, game_info: &GameInfo);
}
//#1

impl GameFn for Box<dyn FnMut(&mut AnyMap, &mut EventHandle, &GameInfo) + 'static> {
    fn exec(&mut self, resources: &mut AnyMap, event_handle: &mut EventHandle, game_info: &GameInfo) {
        self(resources,event_handle,game_info)
    }
}

//#2

impl GameFn for Box<dyn FnMut(&mut AnyMap, &mut EventHandle) + 'static> {
    fn exec(&mut self, resources: &mut AnyMap, event_handle: &mut EventHandle, _game_info: &GameInfo) {
        self(resources,event_handle)
    }
}

//#3

impl GameFn for Box<dyn FnMut(&mut AnyMap) + 'static> {
    fn exec(&mut self, resources: &mut AnyMap, _event_handle: &mut EventHandle, _game_info: &GameInfo) {
        self(resources)
    }
}

//#4

impl GameFn for Box<dyn FnMut() + 'static> {
    fn exec(&mut self, _resources: &mut AnyMap, _event_handle: &mut EventHandle, _game_info: &GameInfo) {
        self()
    }
}

//#5

impl GameFn for Box<dyn FnMut(&mut EventHandle) + 'static> {
    fn exec(&mut self, _resources: &mut AnyMap, event_handle: &mut EventHandle, _game_info: &GameInfo) {
        self(event_handle)
    }
}

//#6

impl GameFn for Box<dyn FnMut(&GameInfo) + 'static> {
    fn exec(&mut self, _resources: &mut AnyMap, _event_handle: &mut EventHandle, game_info: &GameInfo) {
        self(game_info)
    }
}

//#7

impl GameFn for Box<dyn FnMut(&mut AnyMap, &GameInfo) + 'static> {
    fn exec(&mut self, resources: &mut AnyMap, _event_handle: &mut EventHandle, game_info: &GameInfo) {
        self(resources,game_info)
    }
}

//#8

impl GameFn for Box<dyn FnMut(&GameInfo, &mut EventHandle) + 'static> {
    fn exec(&mut self, _resources: &mut AnyMap, event_handle: &mut EventHandle, game_info: &GameInfo) {
        self(game_info,event_handle)
    }
}
