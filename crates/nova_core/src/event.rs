use std::any::Any;
use std::collections::{HashMap, HashSet};

use nova_math::vector::vector2::Vector2;

#[derive(Eq, PartialEq, Hash)]
enum NovaSystemEventType {
    WindowResize,
}

#[derive(Clone)]
enum NovaSystemEventContext {
    WindowResizeContext(Vector2)
}

#[derive(Default)]
struct NovaSystemEvent {
    events: HashMap<NovaSystemEventType, HashSet<fn(NovaSystemEventContext)>>
}

impl NovaSystemEvent {
    pub fn register_event(&mut self, event_type: NovaSystemEventType, callback: fn(NovaSystemEventContext)) {
        if !self.events.contains_key(&event_type) {
            self.events.insert(event_type, HashSet::from([callback]));
        } else {
            self.events.get_mut(&event_type).unwrap().insert(callback);
        }
    }

    // pub fn unregister_event(&mut self, event_type: EventType, callback: Callback) {
    //     if self.events.contains_key(&event_type) {
    //         let mut events = self.events.get_mut(&event_type).unwrap();
    //         if events.contains(&callback) {
    //             events.remove(&callback);
    //         }
    //     }
    // }

    pub fn fire_event(&self, event_type: NovaSystemEventType, event_context: NovaSystemEventContext) {
        if self.events.contains_key(&event_type) {
            let events = self.events.get(&event_type).unwrap();
            events.iter().for_each(|callback| callback(event_context.clone()) )
        }
    }
}

#[cfg(test)]
mod event_tests {
    use nova_math::vector::vector2::Vector2;
    use crate::event::{NovaSystemEventType, NovaSystemEvent, NovaSystemEventContext};

    #[test]
    fn test_1() {
        let mut handler = NovaSystemEvent::default();

        handler.register_event(NovaSystemEventType::WindowResize, callback_1);
        handler.register_event(NovaSystemEventType::WindowResize, callback_2);

        let v1 = Vector2::new(1f32, 1f32);
        handler.fire_event(NovaSystemEventType::WindowResize, NovaSystemEventContext::WindowResizeContext(v1));

        // handler.unregister_event(NovaSystemEventType::WindowResize, callback_1);

        let v2 = Vector2::new(5f32, 6f32);
        handler.fire_event(NovaSystemEventType::WindowResize, NovaSystemEventContext::WindowResizeContext(v2));
    }

    fn callback_1(event_context: NovaSystemEventContext) {
        match event_context {
            NovaSystemEventContext::WindowResizeContext(dimensions) => {
                assert!(true);
                println!("Callback1 x: {} y: {}", dimensions.x, dimensions.y);
            }
        }
    }

    fn callback_2(event_context: NovaSystemEventContext) {
        match event_context {
            NovaSystemEventContext::WindowResizeContext(dimensions) => {
                assert!(true);
                println!("Callback2 x: {} y: {}", dimensions.x, dimensions.y);
            }
        }
    }
}