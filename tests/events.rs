#![cfg(test)]

use anymap::AnyMap;
use age_core::{Game};
use age_core::events::{Event, EventHandle};
use age_core::events::special_events::{ExitEvent};

fn resource_alloc(resources: &mut AnyMap, _: &mut EventHandle) {
    resources.insert("hi");
}

fn check(resources: &mut AnyMap, event_handle: &mut EventHandle ) {
    assert!(resources.get::<&str>().is_some());
assert!(event_handle.contains(&CustomEvent));
    event_handle.write(ExitEvent);
}

fn event( _: &mut AnyMap,events: &mut EventHandle) {
    events.write(CustomEvent);
}

struct CustomEvent;
impl Event for CustomEvent {
    fn id(&self) -> &'static str {
        "custom"
    }
}

#[test]
fn event_test() {
Game::new().add_once(resource_alloc).add_update(check).add_once(event).run();
}
