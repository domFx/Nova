use crate::winit_window::WinitWindow;
use nova_math::vector::vector2::Vector2;
use nova_renderer::renderer::Renderer;

use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

/// Window functionality
pub trait NovaWindow {
    /// Window Loop
    fn run(&mut self, renderer: &mut Renderer);

    /// Window Cleanup
    fn destroy(&self);

    fn get_raw_display_handle(&self) -> RawDisplayHandle;

    fn get_window_handle(&self) -> RawWindowHandle;

    fn get_window_dimensions(&self) -> Vector2<u32>;

    fn get_window(&self) -> &winit::window::Window;

    fn set_title(&self, title: &str);
}

pub fn get_window() -> impl NovaWindow {
    WinitWindow::new()
}
