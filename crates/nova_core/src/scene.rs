use std::collections::HashMap;

use crate::{node::Node, world::Entity};

pub trait Scene {
    fn get_scene_base(&self) -> &SceneBase;
}

#[derive(Default)]
pub struct SceneBase {
    nodes: Vec<Box<dyn Node>>
}

impl SceneBase {
    pub fn new() -> SceneBase {
        let initial_capacity = 10;
        SceneBase {
            nodes: Vec::with_capacity(initial_capacity)
        }
    }

    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }
}