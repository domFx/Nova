use nova_math::vector::vector2::Vector2;

use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash)]
enum Event {
    WindowResize,
}

#[derive(Clone, Copy)]
enum EventContext {
    WindowResizeContext(Vector2)
}

#[derive(Default)]
pub struct SystemEvents {
    events: HashMap<Event, HashSet<fn(EventContext)>>
}

impl SystemEvents {
    pub fn register_event(&mut self, event: Event, callback: fn(EventContext)) {
        if !self.events.contains_key(&event) {
            self.events.insert(event, HashSet::from([callback]));
        } else {
            self.events.get_mut(&event).unwrap().insert(callback);
        }
    }

    pub fn unregister_event(&mut self, event: Event, callback: fn(EventContext)) {
        if self.events.contains_key(&event) {
            let events = self.events.get_mut(&event).unwrap();
            if events.contains(&callback) {
                events.remove(&callback);
            }
        }
    }

    pub fn fire_event(&self, event: Event, event_context: EventContext) {
        if self.events.contains_key(&event) {
            let events = self.events.get(&event).unwrap();
            events.iter().for_each(|callback| {
                callback(event_context.clone())
            });
        }
    }

    pub(crate) fn get_event_registration_count(&self, event: Event) -> i32 {
        self.events[&event].iter().count() as i32
    }
}

#[cfg(test)]
mod event_tests {
    use nova_math::vector::vector2::Vector2;
    use crate::event::{Event, SystemEvents, EventContext};

    /// Assert that registering a handler, firing an event and removing the handler works as expected
    #[test]
    fn register_handler() {
        let mut handler = SystemEvents::default();

        fn callback(context: EventContext) {
            match context {
                EventContext::WindowResizeContext(dimensions) => {
                    assert_eq!(dimensions.x, 1f32);
                    assert_eq!(dimensions.y, 1f32);
                }
            }
        }

        handler.register_event(Event::WindowResize, callback);
        assert_eq!(handler.get_event_registration_count(Event::WindowResize), 1);

        let v1 = Vector2::new(1f32, 1f32);
        handler.fire_event(Event::WindowResize, EventContext::WindowResizeContext(v1));

        handler.unregister_event(Event::WindowResize, callback);
        assert_eq!(handler.get_event_registration_count(Event::WindowResize), 0);

        let v2 = Vector2::new(5f32, 6f32);
        handler.fire_event(Event::WindowResize, EventContext::WindowResizeContext(v2));
    }

    /// Assert that registering a handler twice only registers once, firing an event and removing the handler works as expected
    #[test]
    fn register_handler_twice() {
        let mut handler = SystemEvents::default();

        fn callback_2(context: EventContext) {
            match context {
                EventContext::WindowResizeContext(dimensions) => {
                    assert_eq!(dimensions.x, 1f32);
                    assert_eq!(dimensions.y, 1f32);
                }
            }
        }

        handler.register_event(Event::WindowResize, callback_2);
        handler.register_event(Event::WindowResize, callback_2);

        assert_eq!(handler.get_event_registration_count(Event::WindowResize), 1);

        let v1 = Vector2::new(1f32, 1f32);
        handler.fire_event(Event::WindowResize, EventContext::WindowResizeContext(v1));

        handler.unregister_event(Event::WindowResize, callback_2);
        assert_eq!(handler.get_event_registration_count(Event::WindowResize), 0);

        let v2 = Vector2::new(5f32, 6f32);
        handler.fire_event(Event::WindowResize, EventContext::WindowResizeContext(v2));
    }
}