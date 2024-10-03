use crate::vulkan::vulkan_renderer::VulkanRenderer;
use crate::{renderer_backend::RendererBackend, vulkan::vertex::Vertex};

use nova_math::vector::vector2::Vector2;
use nova_math::vector::vector4::Vector4;

use anyhow::Result;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub enum RendererType {
    Vulkan,
}

pub struct Renderer {
    pub resized: bool,
    renderer_backend: Box<dyn RendererBackend>,
}

impl Renderer {
    pub fn new(
        renderer_type: RendererType,
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle,
        window_dimensions: Vector2<u32>,
    ) -> Result<Self> {
        let renderer_backend = match renderer_type {
            RendererType::Vulkan => Box::new(VulkanRenderer::new(
                window_handle,
                display_handle,
                window_dimensions,
            )?),
        };

        Ok(Self {
            resized: false,
            renderer_backend,
        })
    }

    pub fn render(&mut self, window_dimensions: Vector2<u32>) -> Result<()> {
        self.renderer_backend.render(window_dimensions)?;

        Ok(())
    }

    pub fn draw_rect(
        &self,
        position: Vector2<f32>,
        dimensions: Vector2<f32>,
        color: Vector4<f32>,
    ) -> Result<()> {
        let color = Vector4::new(1.0, 0.0, 0.0, 1.0);

        let vertices: [Vertex; 4] = [
            Vertex::new(position, color, Vector2::zero()),
            Vertex::new(position, color, Vector2::zero()),
            Vertex::new(position, color, Vector2::zero()),
            Vertex::new(position, color, Vector2::zero()),
        ];

        let indices = [0, 1, 2, 2, 3, 0];

        Ok(())
    }

    pub fn destroy(&self) {
        self.renderer_backend.destroy();
    }
}
