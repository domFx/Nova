/// Id wrapper class
#[derive(Default, PartialEq, Eq, Hash, Copy, Clone)]
pub struct EntityId {
    id: u32
}

impl EntityId {
    pub fn new(id: u32) -> EntityId {
        EntityId { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn to_string(&self) -> String {
        self.id.to_string()
    }
}