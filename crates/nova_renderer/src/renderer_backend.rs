use anyhow::Result;
use nova_math::vector::vector2::Vector2;

pub trait RendererBackend {
    fn render(&mut self, window_dimensions: Vector2<u32>) -> Result<()>;

    fn destroy(&self);
}