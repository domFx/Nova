use nova_math::vector3::Vector3;

use crate::color::Color;
use crate::entity_id::EntityId;
use crate::transform::Transform;

/// Indicates how many indices to use and how they are laid out in the renderer
#[derive(Default, Copy, Clone)]
pub enum NodeType {
    #[default]
    Rectangle,
    Triangle,
}

#[derive(Default, Copy, Clone)]
pub struct NodeBase {
    id: EntityId,

    // Transform data
    transform: Transform,

    // Color: RGBA
    color: Option<Color>,

    // Render order (higher = on top)
    z_index: i32,

    // Should the object be drawn
    is_visible: bool,

    // Type of the node
    node_type: NodeType
}

impl NodeBase {
    pub fn new(transform: Transform) -> Self {
        let mut node = Self::default();
        node.transform = transform;
        node
    }
    
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn get_id(&self) -> &EntityId {
        &self.id
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.transform.set_position(position);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }
}

pub trait Scriptable : Node {
    fn update(&self);

    fn fixed_update(&self);
}

pub trait Node {
    fn get_game_node(&self) -> &NodeBase;
}

pub fn draw_triangle_from_transform(transform: Transform) -> NodeBase {
   let mut node = NodeBase::new(transform);
   node.node_type = NodeType::Triangle;
   node
}

pub fn draw_triangle_from_vec3(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> NodeBase {
    NodeBase::new(Transform::new(position, rotation, scale))
}

pub struct RectangleNode { 
    node_base: NodeBase,
    width: f32,
    height: f32
}

impl Scriptable for RectangleNode {
    fn update(&self) {
        // Implement the update logic here
    }

    fn fixed_update(&self) {
        // Implement the fixed_update logic here
    }
}

impl Node for RectangleNode {
    fn get_game_node(&self) -> &NodeBase {
        &self.node_base
    }
}

impl RectangleNode {
    pub fn new(width: f32, height: f32, position: Vector3<f32>, color: Option<Color>) -> Self {
        let mut node_base = NodeBase::default();
        node_base.set_position(position);

        if let Some(c) = color {
            node_base.set_color(c);
        }

        Self {
            node_base: NodeBase::default(),
            width: width,
            height: height
        }
    }
}