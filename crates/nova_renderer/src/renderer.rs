use crate::{renderer_backend, vulkan::vulkan_renderer::VulkanRenderer};
use crate::renderer_backend::RendererBackend;
use crate::vertex::Vertex;
use nova_core::color::Color;

use log::debug;
use nova_math::vector::vector2::Vector2;
use nova_math::vector::vector4::Vector4;

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
    index_buffer: Vec<u32>,
    vertex_buffer: Vec<Vertex>
}

impl Renderer {
    const INITIAL_CAPACITY: usize = 1000;
    
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
            index_buffer: Vec::with_capacity(Self::INITIAL_CAPACITY),
            vertex_buffer: Vec::with_capacity(Self::INITIAL_CAPACITY)
        }
    }

    pub fn begin_rendering(&mut self) {
        if let Some(renderer_backend) = &mut self.renderer_backend {
            renderer_backend.prepare_window_surface(Vector2::new(800, 600));
            renderer_backend.begin_commands();
            renderer_backend.begin_render_pass();
        }
    }

    pub fn end_rendering(&mut self) {
        if let Some(renderer_backend) = &mut self.renderer_backend {
            renderer_backend.draw();
            renderer_backend.end_render_pass();
            renderer_backend.end_commands();
            renderer_backend.submit();
            renderer_backend.present();
        }
    }
    
    pub fn draw_line_2d(&mut self, start_position: Vector2<f32>, end_position: Vector2<f32>, color: Color) {
        let vertices = [
            Vertex::new(Vector2::new(start_position.x, end_position.y), color.to_vector4(), Vector2::zero()),
            Vertex::new(Vector2::new(end_position.x, end_position.y), color.to_vector4(), Vector2::zero())
        ];

        let indices = [0, 1];
        
        self.vertex_buffer.extend_from_slice(&vertices);
        self.index_buffer.extend_from_slice(&indices);
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