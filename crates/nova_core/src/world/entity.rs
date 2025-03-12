use anyhow::{anyhow, Result};

const MAX_ENTITIES: usize = 5000;

#[derive(Default, Debug, Clone, Copy, Hash)]
pub struct Entity {
    id: usize
}

impl Entity {
    pub(crate) fn new(id: usize) -> Entity {
        Entity { 
            id
        }
    }
    
    pub(crate) fn id(&self) -> usize {
        self.id
    }
}