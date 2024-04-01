use crate::renderer_backend::RendererBackend;
use crate::vulkan::vulkan_renderer::VulkanRenderer;
use nova_math::vector::vector2::Vector2;

use anyhow::Result;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub enum RendererType {
    Vulkan
}

pub struct Renderer {
    pub resized: bool,
    renderer_backend: Box<dyn RendererBackend>
}

impl Renderer {
    pub fn new(
        renderer_type: RendererType,
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle,
        window_dimensions: Vector2
    ) -> Result<Self> {
        let renderer_backend = match renderer_type {
            RendererType::Vulkan => Box::new(VulkanRenderer::new(window_handle, display_handle, window_dimensions)?)
        };

        Ok(Self {
            resized: false,
            renderer_backend
        })
    }

    pub fn render(&mut self, window_dimensions: Vector2) -> Result<()> {
        self.renderer_backend.render(window_dimensions)?;

        Ok(())
    }

    pub fn destroy(&self) {
        self.renderer_backend.destroy();
    }
}