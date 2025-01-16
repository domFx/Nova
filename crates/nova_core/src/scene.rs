use std::collections::HashMap;

use crate::node::{ Node, NodeId };

#[allow(unused)]
#[derive(Default)]
pub struct Scene {
    nodes: HashMap<NodeId, Node>
}

impl Scene {
    pub fn new() -> Scene {
        let default_capacity = 10;
        Scene {
            nodes: HashMap::with_capacity(default_capacity)
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.get_id().to_owned(), node);
    }

    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        self.nodes.contains_key(node_id)
    }

    pub fn remove_node(&mut self, node_id: &NodeId) {
        self.nodes.remove(node_id);
    }

    pub fn get_nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }
}