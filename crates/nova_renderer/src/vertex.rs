use nova_math::vector::{Vector2, Vector4};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    pos: Vector2<f32>,
    color: Vector4<f32>,
    tex_coord: Vector2<f32>,
    tex_idx: i32
}

impl Vertex {
    pub(crate) const fn new(pos: Vector2<f32>, color: Vector4<f32>, tex_coord: Vector2<f32>) -> Self {
        Self {
            pos,
            color,
            tex_coord,
            tex_idx: -1
        }
    }
    
    pub(crate) const fn new_with_texture(pos: Vector2<f32>, color: Vector4<f32>, tex_coord: Vector2<f32>, tex_idx: i32) -> Self {
        Self {
            pos,
            color,
            tex_coord,
            tex_idx
        }
    }
}
