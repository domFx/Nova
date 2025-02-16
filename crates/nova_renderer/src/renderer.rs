use crate::{renderer_backend, vulkan::vulkan_renderer::VulkanRenderer};
use crate::renderer_backend::RendererBackend;
use crate::vertex::Vertex;

use nova_math::vector::vector2::Vector2;

use anyhow::{anyhow, Result};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use thiserror::Error;

pub enum RendererType {
    Vulkan,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
enum RendererState {
    #[default]
    Initialization,
    Ready,
    Cleanup,
    Complete,
    Error
}

// TODO: Macro? - Pull this out for shared use
impl std::fmt::Display for RendererState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Error, Debug)]
enum RendererError {
    #[error("Invalid renderer state | Expected state: {expected:?} | Actual state: {actual:?}")]
    InvalidRendererState {
        expected: RendererState,
        actual: RendererState
    },
    #[error("Renderer Backend Not Found")]
    RendererBackendNotFound
}

#[derive(Default)]
pub struct Renderer {
    renderer_backend: Option<Box<dyn RendererBackend>>,
    renderer_state: RendererState,
}

impl Renderer {
    pub fn new_vulkan_renderer(
        &mut self, 
        raw_window_handle: RawWindowHandle, 
        raw_display_handle: RawDisplayHandle,
        window_dimensions: Vector2<u32>
    ) -> Result<()> {
        self.new(raw_window_handle, raw_display_handle, window_dimensions, RendererType::Vulkan)?;

        Ok(())
    }

    pub fn new(&mut self,
        raw_window_handle: RawWindowHandle,
        raw_display_handle: RawDisplayHandle,
        window_dimensions: Vector2<u32>,
        renderer_type: RendererType
    ) -> Result<()> {
        let renderer_backend = match renderer_type {
            RendererType::Vulkan => VulkanRenderer::new(raw_window_handle, raw_display_handle, window_dimensions)?,
        };
        
        self.renderer_backend = Some(Box::from(renderer_backend));
        self.renderer_state = RendererState::Ready;

        Ok(())
    }

    pub fn begin_drawing(&self) -> Result<()> {
        Ok(())
    }

    pub fn end_drawing(&self) -> Result<()> {
        Ok(())
    }

    pub fn render(&mut self, window_dimensions: Vector2<u32>) -> Result<()> {
        if self.renderer_state != RendererState::Ready {
            return Err(anyhow!(RendererError::InvalidRendererState {
                expected: RendererState::Ready,
                actual: self.renderer_state
            }))
        }

        if let Some(renderer_backend) = self.renderer_backend.as_mut() {
            renderer_backend.render(window_dimensions)?;
        } else {
            return Err(anyhow!(RendererError::RendererBackendNotFound))
        }

        Ok(())
    }

    pub fn destroy(&mut self) {
        if let Some(ref mut renderer_backend) = self.renderer_backend {
            renderer_backend.destroy();
        }

        self.renderer_backend = None;
    }
}

/*    pub fn draw_rect(
        &mut self,
        position: &Vector2<f32>,
        size: &Vector2<f32>,
        color: &Vector4<f32>,
    ) -> Result<()> {
        let text_coord = Vector2::zero();

        let vertices = vec![
            Vertex::new(Vector2::new(position.x, position.y), color.to_owned(), text_coord),
            Vertex::new(Vector2::new(position.x + size.x, position.y), color.to_owned(), text_coord),
            Vertex::new(Vector2::new(position.x + size.x, position.y + size.y), color.to_owned(), text_coord),
            Vertex::new(Vector2::new(position.x, position.y + size.y), color.to_owned(), text_coord)
        ];

        self.quads.add_quad(vertices);

        Ok(())
    }*/
