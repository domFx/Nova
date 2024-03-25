use anyhow::Result;
use ash::vk::Window;
use cgmath::Vector2;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use crate::renderer_backend::RendererBackend;
use crate::vulkan::vulkan_renderer::VulkanRenderer;
use crate::window::NovaWindow;

pub enum RendererType {
    Vulkan
}

pub struct Renderer {
    pub resized: bool,
    renderer_backend: Box<dyn RendererBackend>
}

impl Renderer {
    pub fn new(renderer_type: RendererType, window: &dyn NovaWindow) -> Result<Self> {
        let renderer_backend = match renderer_type {
            RendererType::Vulkan => Box::new(VulkanRenderer::new(window.get_window_handle(), window.get_raw_display_handle(), window.get_window_dimensions())?)
        };

        Ok(Self {
            resized: false,
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