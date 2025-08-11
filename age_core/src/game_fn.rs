//unused for later use



/*
//all the dirty impl work
use crate::events::EventHandle;
use crate::info::GameInfo;
use anymap::AnyMap;
use crate::Game;
//2^2=4 possibilties
pub trait GameFn: 'static {
    fn exec(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    );
}

/*

impl GameFn for for<'a> fn(&'a EventHandle) {
    fn exec(
        &mut self,
        _resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        self(event_handle);
    }
}



//#1

impl GameFn for fn(&mut AnyMap, &mut EventHandle) {
    fn exec(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        self(resources, event_handle)
    }
}

impl GameFn for Box<dyn FnMut(&mut AnyMap, &mut EventHandle)> {
    fn exec(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        self(resources, event_handle)
    }
}

impl From<fn(&mut AnyMap, &mut EventHandle)> for Box<dyn GameFn> {
    fn from(value: fn(&mut AnyMap, &mut EventHandle)) -> Self {
        Box::new(value)
    }
}


//#2

impl GameFn for fn(&mut AnyMap) {
    fn exec(
        &mut self,
        resources: &mut AnyMap,
        _event_handle: &mut EventHandle,
    ) {
        self(resources)
    }
}

impl GameFn for Box<dyn FnMut(&mut AnyMap)> {
    fn exec(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        self(resources)
    }
}

impl From<fn(&mut AnyMap)> for Box<dyn GameFn> {
    fn from(value: fn(&mut AnyMap)) -> Self {
        Box::new(value)
    }
}

//#3

impl GameFn for  fn(&mut EventHandle) {
    fn exec(
        &mut self,
        _resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        self(event_handle)
    }
}

impl GameFn for  Box<dyn FnMut(&mut EventHandle)> {
    fn exec(
        &mut self,
        _resources: &mut AnyMap,
        event_handle: &mut EventHandle,
    ) {
        self(event_handle)
    }
}

impl From<fn(&mut EventHandle)> for Box<dyn GameFn> {
    fn from(value: fn(&mut EventHandle)) -> Self {
        Box::new(value)
    }
}

//#4

impl GameFn for fn() {
    fn exec(
        &mut self,
        _resources: &mut AnyMap,
        _event_handle: &mut EventHandle,
    ) {
        self()
    }
}

impl GameFn for Box<dyn FnMut()> {
    fn exec(
        &mut self,
        _resources: &mut AnyMap,
        _event_handle: &mut EventHandle,
    ) {
        self()
    }
}

impl From<fn()> for Box<dyn GameFn> {
    fn from(value: fn()) -> Self {
        Box::new(value)
    }
}
*/
*/