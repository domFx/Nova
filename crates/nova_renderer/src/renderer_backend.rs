use anyhow::Result;

use nova_core::image::Image;
use nova_core::color::Color;
use nova_math::vector::vector2::Vector2;

use crate::{core_types::PrimitiveTopologyType, pipeline_config::PipelineType, vertex::Vertex};

/// RendererBackend trait defines the interface for rendering backends in the engine.
pub trait RendererBackend {
    /// Verifies swapchain, gets image index, etc.
    fn prepare_window_surface(&mut self, window_dimensions: Vector2<u32>) -> Result<()>;

    /// Begins command list/buffer
    fn begin_commands(&mut self) -> Result<()>;

    /// Begins render pass with the specified clear color
    fn begin_render_pass(&mut self, clear_color: Color) -> Result<()>;

    /// Binds pipelines and issues draw commands
    fn draw(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, pipeline_type: PipelineType) -> Result<()>;

    /// Ends current render pass
    fn end_render_pass(&mut self) -> Result<()>;

    /// Ends command list/buffer
    fn end_commands(&mut self) -> Result<()>;

    /// Submits command list/buffer for execution.
    fn submit(&mut self) -> Result<()>;

    /// Presents the swapchain image to the window.
    fn present(&mut self) -> Result<()>;

    /// Loads images to the GPU
    fn load_images(&mut self, images: &Vec<Image>) -> Result<()>;

    /// Cleans up resources used by the renderer backend
    fn destroy(&self);
}