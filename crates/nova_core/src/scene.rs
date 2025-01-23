use std::collections::HashMap;

use crate::node::{ Node, NodeId };

#[allow(unused)]
#[derive(Default)]
pub struct Scene {
    nodes: HashMap<NodeId, Box<dyn Node>>
}

impl Scene {
    pub fn new() -> Scene {
        let default_capacity = 10;
        Scene {
            nodes: HashMap::with_capacity(default_capacity)
        }
    }

    pub fn add_node(&mut self, nova_node: Box<dyn Node>) {
        self.nodes.insert(nova_node.get_game_node().get_id().to_owned(), nova_node);
    }

    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        self.nodes.contains_key(node_id)
    }

    pub fn remove_node(&mut self, node_id: &NodeId) {
        self.nodes.remove(node_id);
    }

    pub fn get_nodes(&self) -> &HashMap<NodeId, Box<dyn Node>> {
        &self.nodes
    }
}