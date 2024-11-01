use nova_math::vector2::Vector2;
use nova_math::vector4::Vector4;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    pos: Vector2<f32>,
    color: Vector4<f32>,
    tex_coord: Vector2<f32>,
}

impl Vertex {
    pub(crate) const fn new(pos: Vector2<f32>, color: Vector4<f32>, tex_coord: Vector2<f32>) -> Self {
        Self {
            pos,
            color,
            tex_coord,
        }
    }
}
