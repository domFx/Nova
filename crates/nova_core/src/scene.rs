use std::collections::HashMap;

use crate::{node::Node, world::entity::Entity};

pub trait Scene {
    fn get_scene_base(&self) -> &SceneBase;
}

#[derive(Default)]
pub struct SceneBase {
    nodes: HashMap<Entity, Box<dyn Node>>
}

impl SceneBase {
    pub fn new() -> SceneBase {
        let default_capacity = 10;
        SceneBase {
            nodes: HashMap::with_capacity(default_capacity)
        }
    }

    // pub fn add_node(&mut self, nova_node: Box<dyn Node>) {
    //     self.nodes.insert(nova_node.get_game_node().get_id().to_owned(), nova_node);
    // }

    // pub fn contains_node(&self, node_id: &Entity) -> bool {
    //     self.nodes.contains_key(node_id)
    // }

    // pub fn remove_node(&mut self, node_id: &Entity) {
    //     self.nodes.remove(node_id);
    // }

    // pub fn get_nodes(&self) -> &HashMap<Entity, Box<dyn Node>> {
    //     &self.nodes
    // }
}