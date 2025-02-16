use crate::{renderer_backend, vulkan::vulkan_renderer::VulkanRenderer};
use crate::renderer_backend::RendererBackend;
use crate::vertex::Vertex;

use log::debug;
use nova_math::vector::vector2::Vector2;

use anyhow::{anyhow, Result};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use thiserror::Error;

pub enum RendererBackendType {
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

pub struct Renderer {
    renderer_backend: Option<Box<dyn RendererBackend>>,
    renderer_state: RendererState,
    index_buffer: RenderBuffer,
    vertex_buffer: RenderBuffer
}

impl Renderer {
    /// Initializes a renderer with a Vulkan backend
    pub fn new_vulkan_renderer(
        raw_window_handle: RawWindowHandle, 
        raw_display_handle: RawDisplayHandle,
        window_dimensions: Vector2<u32>
    ) -> Self {
        Self::new(raw_window_handle, raw_display_handle, window_dimensions, RendererBackendType::Vulkan)
    }

    pub fn new(
        raw_window_handle: RawWindowHandle,
        raw_display_handle: RawDisplayHandle,
        window_dimensions: Vector2<u32>,
        renderer_type: RendererBackendType
    ) -> Self {
        // Only one backend for now...
        let renderer_backend: Box<dyn RendererBackend> = match renderer_type {
            RendererBackendType::Vulkan => Box::from(
                VulkanRenderer::new(raw_window_handle, raw_display_handle, window_dimensions)
                    .expect("Failed to create Vulkan Renderer")
                )
        };
        
        let renderer_state = RendererState::Ready;

        Self {
            renderer_backend: Some(renderer_backend),
            renderer_state,
            index_buffer: RenderBuffer::new(RenderBufferType::Index),
            vertex_buffer: RenderBuffer::new(RenderBufferType::Vertex)
        }
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


enum RenderBufferType {
    Vertex,
    Index
}

struct RenderBuffer {
    internal_buffer_data: Vec<u32>,
    buffer_type: RenderBufferType
}

impl RenderBuffer {
    const INITIAL_CAPACITY: usize = 1000;

    fn new(buffer_type: RenderBufferType) -> Self {
        Self {
            internal_buffer_data: Vec::with_capacity(Self::INITIAL_CAPACITY),
            buffer_type
        }
    }

    fn get_buffer_type(&self) -> &RenderBufferType {
        &self.buffer_type
    }
    
    fn get_buffer_count(&self) -> usize {
        self.internal_buffer_data.len()
    }

    fn get_capacity(&self) -> usize {
        self.internal_buffer_data.capacity()
    }

    fn load_data(&mut self, data: &[u32]) {
        if data.len() > self.internal_buffer_data.capacity() {
            debug!("Buffer resize needed, current capacity: {}, new capacity: {}", self.internal_buffer_data.capacity(), self.internal_buffer_data.capacity() + data.len());
        }
        self.internal_buffer_data.extend_from_slice(data);
    }
}