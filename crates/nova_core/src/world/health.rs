use nova_macros::Component;

#[derive(Component, Default)]
pub struct Health {
    pub value: usize
}

impl Health {
    pub fn new(value: usize) -> Self {
        Self {
            value
        }
    }
}