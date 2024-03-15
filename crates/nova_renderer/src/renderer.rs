use anyhow::Result;
use cgmath::Vector2;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use crate::vulkan::vulkan_app::VulkanApp;

pub struct Renderer {
    pub resized: bool,
    raw_window_handle: RawWindowHandle,
    raw_display_handle: RawDisplayHandle,
    window_dimensions: Vector2<f32>,
    vulkan_renderer: VulkanApp
}

impl Renderer {
    pub fn new(raw_window_handle: RawWindowHandle, raw_display_handle: RawDisplayHandle, window_dimensions: Vector2<f32>) -> Result<Self> {
        let vulkan_renderer = VulkanApp::new(raw_window_handle, raw_display_handle, window_dimensions)?;

        Ok(Self {
            resized: false,
            raw_window_handle,
            raw_display_handle,
            window_dimensions,
            vulkan_renderer
        })
    }

    pub fn render(&mut self, window_dimensions: Vector2<f32>) -> Result<()> {
        unsafe {
            self.vulkan_renderer.render(window_dimensions)?;
        }
        Ok(())
    }

    pub fn destroy(&mut self) {
        self.vulkan_renderer.destroy();
    }
}