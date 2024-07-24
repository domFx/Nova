use std::any::Any;
use nova_math::vector::vector2::Vector2;

use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Event {
    WindowResize,
}

#[derive(Debug, Clone, Copy)]
pub enum EventContext {
    // WindowResizeContext(Vector2)
}

#[derive(Default)]
pub struct SystemEvents {
    events: HashMap<Event, HashSet<fn(EventContext)>>
}

static mut SYS_EVENTS: Lazy<Mutex<SystemEvents>> = Lazy::new(|| Mutex::new(SystemEvents::default()));

impl SystemEvents {
    pub fn register_event(&mut self, event: Event, callback: fn(EventContext)) {
        if !self.events.contains_key(&event) {
            self.events.insert(event, HashSet::from([callback]));
        } else {
            self.events.get_mut(&event).unwrap().insert(callback);
        }
    }

    pub fn unregister_event(&mut self, event: &Event, callback: fn(EventContext)) {
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


pub struct EventListener {
    event: Event,
    callback: fn(EventContext)
}

impl EventListener {
    /// Register the listener with the event handler
    pub fn new(event: Event, callback: fn(EventContext)) -> Self {
        unsafe {
            SYS_EVENTS.deref_mut().lock().unwrap().register_event(event.clone(), callback)
        }

        Self {
            event,
            callback
        }
    }

    /// Remove the listener from the event handler
    pub fn remove_listener(&self) {
        unsafe {
            SYS_EVENTS.deref_mut().lock().unwrap().unregister_event(&self.event, self.callback)
        }
    }
}

impl Drop for EventListener {
    /// Ensure that listener is removed from event handler when dropped
    fn drop(&mut self) {
        self.remove_listener();
    }
}


pub fn fire_event(event: Event, event_context: EventContext) {
    println!("Event fired: {:?}, {:?}", event, event_context);
    unsafe {
        SYS_EVENTS.deref_mut().lock().unwrap().fire_event(event, event_context)
    }
}

#[cfg(test)]
mod event_handler_tests {
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

        handler.unregister_event(&Event::WindowResize, callback);
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

        handler.unregister_event(&Event::WindowResize, callback_2);
        assert_eq!(handler.get_event_registration_count(Event::WindowResize), 0);

        let v2 = Vector2::new(5f32, 6f32);
        handler.fire_event(Event::WindowResize, EventContext::WindowResizeContext(v2));
    }
}