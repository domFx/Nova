use std::sync::{Mutex, LazyLock};

// TODO: Create an Entity management system
// This is a quick and dirty way to track id's generated in the system
static ENTITY_COUNT: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

/// Id wrapper class
#[derive(Default, PartialEq, Eq, Hash, Copy, Clone)]
pub struct EntityId {
    id: u32
}

impl EntityId {
    pub fn new() -> EntityId {
        let mut current_id = ENTITY_COUNT.lock().unwrap();

        *current_id = *current_id + 1;
        
        EntityId { id: *current_id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn to_string(&self) -> String {
        self.id.to_string()
    }
}

#[cfg(test)]
mod vector2_tests {
    use super::*;

    #[test]
    fn test_incremental_id() {

        for i in 1..5 {
            let entity_id = EntityId::new();
            assert_eq!(entity_id.id, i as u32);
        }
    }
}