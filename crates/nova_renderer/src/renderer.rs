use anyhow::Result;
use cgmath::Vector2;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use crate::renderer_backend::RendererBackend;
use crate::vulkan::vulkan_renderer::VulkanRenderer;

pub enum RendererType {
    Vulkan
}

pub struct Renderer {
    pub resized: bool,
    raw_window_handle: RawWindowHandle,
    raw_display_handle: RawDisplayHandle,
    window_dimensions: Vector2<f32>,
    renderer_backend: Box<dyn RendererBackend>
}

impl Renderer {
    pub fn new(renderer_type: RendererType, raw_window_handle: RawWindowHandle, raw_display_handle: RawDisplayHandle, window_dimensions: Vector2<f32>) -> Result<Self> {
        let renderer_backend = match renderer_type {
            RendererType::Vulkan => Box::new(VulkanRenderer::new(raw_window_handle, raw_display_handle, window_dimensions)?)
        };

        Ok(Self {
            resized: false,
            raw_window_handle,
            raw_display_handle,
            window_dimensions,
            renderer_backend
        })
    }

    pub fn render(&mut self, window_dimensions: Vector2<f32>) -> Result<()> {
        self.renderer_backend.render(window_dimensions)?;

        Ok(())
    }

    pub fn destroy(&mut self) {
        self.renderer_backend.destroy();
    }
}