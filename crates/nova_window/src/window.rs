use crate::winit_window::WinitWindow;
use nova_math::vector::vector2::Vector2;
use nova_renderer::renderer::Renderer;

use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

/// Window functionality
pub trait NovaWindow {
    /// Window Loop
    fn run(&self);

    /// Window Cleanup
    fn destroy(&self);

    fn get_raw_display_handle(&self) -> RawDisplayHandle;

    fn get_window_handle(&self) -> RawWindowHandle;

    fn get_window_dimensions(&self) -> Vector2<u32>;

    fn set_title(&mut self, title: &str);

    fn is_running(&self) -> bool;

    fn redraw(&self) -> bool;
}

pub fn get_nova_window(title: &str) -> Box<dyn NovaWindow> {
    Box::new(WinitWindow::new(title))
}