use crate::world::entity::Entity;

use std::collections::HashMap;
use std::any::{Any, TypeId};

use anyhow::{Result, anyhow};
use log::debug;

pub trait AnyImpl : Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Component : AnyImpl { }

impl<T: Component> AnyImpl for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Represents the entire game space
/// Responsible for global game systems
struct World {
    components: HashMap<TypeId, Vec<Option<Box<dyn Component>>>>,
    entities: Vec<Option<Entity>>,
    max_entity_count: usize,
    current_entity_count: usize
}

impl World {
    fn new(max_entity_count: usize) -> Self {
        World {
            components: HashMap::new(),
            entities: vec![None; max_entity_count],
            max_entity_count,
            current_entity_count: 0
        }
    }

    fn add_component<T: Component>(&mut self, entity: &Entity, component: T) {
        let type_id = component.type_id();

        let components_option = self.components.get_mut(&type_id);

        if let Some(components) = components_option {
            components[entity.id() as usize] = Some(Box::from(component));
        } else {
            let mut components: Vec<Option<Box<dyn Component>>> = std::iter::repeat_with(|| None)
                .take(self.max_entity_count)
                .collect();

            components[entity.id() as usize] = Some(Box::from(component));
            
            self.components.insert(type_id, components);
        }

        return;
    }

    fn get_component_mut<T: Component>(&mut self, entity: &Entity) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>()) // Get the Vec<Option<Box<dyn Component>>>
            .and_then(|vec| vec.get_mut(entity.id() as usize)) // Get the specific index in the Vec
            .and_then(|opt_box| opt_box.as_mut()) // Unwrap the Option
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<T>())
    }

    fn get_component<T: Component>(&mut self, entity: &Entity) -> Option<&T> {
        self.components
            .get_mut(&TypeId::of::<T>()) // Get the Vec<Option<Box<dyn Component>>>
            .and_then(|vec| vec.get_mut(entity.id() as usize)) // Get the specific index in the Vec
            .and_then(|opt_box| opt_box.as_mut()) // Unwrap the Option
            .and_then(|boxed| boxed.as_any().downcast_ref::<T>())
    }

    pub fn create_entity(&mut self) -> Result<Entity> {
        if self.current_entity_count >= self.max_entity_count {
            return Err(anyhow!("Reached maximum entities allowed"));
        }

        // Find the next empty slot
        for (i, entity) in self.entities.iter().enumerate() { 
            if self.entities[i].is_some() {
                continue;
            }

            let empty_slot = i;
            let mut entity = Entity::new(empty_slot + 1);
            self.entities[empty_slot] = Some(entity);
            self.current_entity_count += 1;

            let reference = self.entities[empty_slot].as_mut().unwrap();
            return Ok(entity);
        }

        Err(anyhow!("No empty slots found"))
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if let Some(slot) = self.entities.get_mut(entity.id() - 1) {
            *slot = None;
            self.current_entity_count -= 1;
        }
    }
    
    pub fn check_entitiy_exists(&self, entity: Entity) -> bool {
        if self.entities.capacity() == 0 {
            return false;
        }

        self.entities[(entity.id() - 1)].is_some()
    }
}

mod tests {
    use crate::world;

    use super::*;

    struct Health(u32);
    impl Component for Health { }

    #[test]
    fn add_component_success() {
        let mut manager = World::new(5);

        let e1 = Entity::new(1);
        let e2 = Entity::new(2);
        let e3= Entity::new(3);

        manager.add_component::<Health>(&e1, Health(2));
        manager.add_component::<Health>(&e2, Health(5));

        let component = manager.get_component::<Health>(&e1);
        
        assert!(component.is_some());
        assert_eq!(component.unwrap().0, 2);

        let component = manager.get_component::<Health>(&e2);
        
        assert!(component.is_some());
        assert_eq!(component.unwrap().0, 5);

        let component = manager.get_component::<Health>(&e3);
        
        assert!(component.is_none());
    }

    #[test]
    fn create_single_entity_success() {
        let mut world = World::new(1);
        let entity1 = world.create_entity().unwrap();
        assert!(entity1.id() == 1);
        assert_eq!(world.current_entity_count, 1);
    }

    #[test]
    fn create_multiple_entities_success() {
        let mut world = World::new(2);

        let entity1 = world.create_entity().unwrap();
        let entity2 = world.create_entity().unwrap();
        assert_eq!(entity1.id(), 1); 
        assert_eq!(entity2.id(), 2); 
        assert_eq!(world.current_entity_count, 2);
    }

    #[test]
    fn create_entitiy_max_reached() {
        let mut world = World:: new(0);
        let entity = world.create_entity();
        assert_eq!(entity.is_err(), true);
        assert_eq!(world.current_entity_count, 0);
    }

    #[test]
    fn check_entity_exists_true() {
        let mut world = World::new(1);
        let entity = world.create_entity().unwrap();
        assert_eq!(world.check_entitiy_exists(entity), true);
    }

    #[test]
    fn check_entity_exists_false() { 
        let mut world = World::new(1);
        let entity = Entity::new(1);
        assert_eq!(world.check_entitiy_exists(entity), false);
    }

    #[test]
    fn destroy_valid_entity_success() {
        let mut world = World::new(1);
        let entity = world.create_entity().unwrap();
        assert_eq!(world.current_entity_count, 1);

        world.destroy_entity(entity);
        assert_eq!(world.current_entity_count, 0);
        assert_eq!(world.check_entitiy_exists(entity), false)
    }

    #[test]
    fn destroy_entity_when_empty_success() {
        let mut world = World::new(0);
        let entity = Entity::new(1);
        world.destroy_entity(entity);
        assert_eq!(world.current_entity_count, 0);
        assert_eq!(world.check_entitiy_exists(entity), false)
    }
}