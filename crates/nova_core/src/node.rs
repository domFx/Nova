use nova_math::vector3::Vector3;
use crate::color::Color;
use crate::transform::Transform;

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

#[allow(unused)]
#[derive(Default, Copy, Clone)]
pub struct Node {
    id: NodeId,

    // Transform data
    transform: Transform,

    // Color: RGBA
    color: Color,

    // Render order (higher = on top)
    z_index: i32,

    // Should the object be drawn
    is_visible: bool
}

impl Node {
    pub fn new(transform: Transform) -> Node {
        let mut game_object = Node::default();
        game_object.transform = transform;
        game_object
    }
    
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn get_id(&self) -> &NodeId {
        &self.id
    }
}

pub fn draw_triangle_from_transform(transform: Transform) -> Node {
    Node::new(transform)
}

pub fn draw_triangle_from_vec3(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Node {
    Node::new(Transform::new(position, rotation, scale))
}