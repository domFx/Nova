use anyhow::Result;
use cgmath::Vector2;

pub trait RendererBackend {
    fn render(&mut self, window_dimensions: Vector2<f32>) -> Result<()>;

    fn destroy(&mut self);
}