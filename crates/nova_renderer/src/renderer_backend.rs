use anyhow::Result;
use nova_math::vector::vector2::Vector2;

// frame_prepare_window_surface - verifies swapchain, gets image index, etc.
// frame_commands_begin - begins command list/buffer
// <insert renderpasses, draws, etc. here>
// frame_commands_end - ends command list/buffer
// frame_submit - submits command list/buffer for execution.
// frame_present - once frame execution is complete, presents swapchain image.
pub trait RendererBackend {
    fn prepare_window_surface(&mut self, window_dimensions: Vector2<u32>) -> Result<()>;

    fn begin_commands(&mut self) -> Result<()>;

    fn begin_render_pass(&mut self) -> Result<()>;

    fn draw(&mut self) -> Result<()>;

    fn end_render_pass(&mut self) -> Result<()>;

    fn end_commands(&mut self) -> Result<()>;

    fn submit(&mut self) -> Result<()>;

    fn present(&mut self) -> Result<()>;

    fn destroy(&self);
}