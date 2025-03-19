use std::any::Any;

pub trait AnyImpl : Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Component : AnyImpl { 
    fn get_size(&self) -> usize;
}

impl<T: Component> AnyImpl for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}