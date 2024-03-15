use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash)]
enum EventType {
    WindowResized,
}

type Callback = fn();

#[derive(Default)]
struct EventHandler {
    events: HashMap<EventType, HashSet<Callback>>
}

impl EventHandler {
    pub fn register_event(&mut self, event_type: EventType, callback: Callback) {
        if !self.events.contains_key(&event_type) {
            self.events.insert(event_type, HashSet::from([callback]));
        } else {
            self.events.get_mut(&event_type).unwrap().insert(callback);
        }
    }

    pub fn unregister_event(&mut self, event_type: EventType, callback: Callback) {
        if self.events.contains_key(&event_type) {
            let mut events = self.events.get_mut(&event_type).unwrap();
            if events.contains(&callback) {
                events.remove(&callback);
            }
        }
    }

    pub fn fire_event(&self, event_type: EventType) {
        if self.events.contains_key(&event_type) {
            let events = self.events.get(&event_type).unwrap();
            events.iter().for_each(|callback| callback() )
        }
    }
}

#[cfg(test)]
mod event_tests {
    use crate::event::{EventHandler, EventType};

    #[test]
    fn test_1() {
        let mut handler = EventHandler::default();

        handler.register_event(EventType::WindowResized, callback_1);
        handler.register_event(EventType::WindowResized, callback_2);

        handler.fire_event(EventType::WindowResized);

        handler.unregister_event(EventType::WindowResized, callback_1);

        handler.fire_event(EventType::WindowResized);
    }

    fn callback_1() {
        assert!(true);
        println!("Callback1")
    }

    fn callback_2() {
        assert!(true);
        println!("Callback2")
    }
}