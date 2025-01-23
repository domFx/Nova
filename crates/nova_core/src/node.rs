use nova_math::vector3::Vector3;
use crate::color::Color;
use crate::transform::Transform;

/// Id wrapper
#[derive(Default, PartialEq, Eq, Hash, Copy, Clone)]
pub struct NodeId {
    id: u32
}

impl NodeId {
    pub fn new(id: u32) -> NodeId {
        NodeId { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn to_string(&self) -> String {
        self.id.to_string()
    }
}

/// Indicates how many indices to use and how they are laid out in the renderer
#[derive(Default, Copy, Clone)]
pub enum NodeType {
    #[default]
    Rectangle,
    Triangle,
}

#[derive(Default, Copy, Clone)]
pub struct NodeBase {
    id: NodeId,

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

    pub fn get_id(&self) -> &NodeId {
        &self.id
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
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
    NodeBase::new(transform)
}

pub fn draw_triangle_from_vec3(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> NodeBase {
    NodeBase::new(Transform::new(position, rotation, scale))
}

pub struct RectangleNode { 
    game_node: NodeBase,
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
        &self.game_node
    }
}