use crate::vulkan::vulkan_renderer::VulkanTopology;
use crate::{renderer_backend, vulkan::vulkan_renderer::VulkanRenderer};
use crate::renderer_backend::RendererBackend;
use crate::vertex::{self, Vertex};
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

struct RenderDrawBuffer {
    index_buffer: Vec<u32>,
    vertex_buffer: Vec<Vertex>
}

impl RenderDrawBuffer {
    const INITIAL_CAPACITY: usize = 1000;

    pub fn new() -> Self {
        Self {
            index_buffer: Vec::with_capacity(Self::INITIAL_CAPACITY),
            vertex_buffer: Vec::with_capacity(Self::INITIAL_CAPACITY)
        }
    }

    pub fn extend_vertex_buffer(&mut self, vertices: &[Vertex]) {
        self.vertex_buffer.extend_from_slice(&vertices);
    }

    pub fn extend_index_buffer(&mut self, indices: &[u32]) {
        self.index_buffer.extend_from_slice(&indices);
    }
}

pub struct Renderer {
    renderer_backend: Option<Box<dyn RendererBackend>>,
    renderer_state: RendererState,
    line_buffer: RenderDrawBuffer,
    triangle_buffer: RenderDrawBuffer
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
            line_buffer: RenderDrawBuffer::new(),
            triangle_buffer: RenderDrawBuffer::new()
        }
    }

    pub fn begin_rendering(&mut self, window_dimensions: Vector2<u32>) {
        if let Some(renderer_backend) = &mut self.renderer_backend {
            renderer_backend.prepare_window_surface(window_dimensions);
            renderer_backend.begin_commands();
            renderer_backend.begin_render_pass(Color::BLACK);
        }
    }

    pub fn end_rendering(&mut self) {
        if let Some(renderer_backend) = &mut self.renderer_backend {
            
            renderer_backend.draw(self.triangle_buffer.vertex_buffer.clone(),  self.triangle_buffer.index_buffer.clone(), crate::core_types::TopologyType::TriangleList);
            
            renderer_backend.draw(self.line_buffer.vertex_buffer.clone(),  self.line_buffer.index_buffer.clone(), crate::core_types::TopologyType::LineList);

            
            renderer_backend.end_render_pass();
            renderer_backend.end_commands();
            renderer_backend.submit();
            renderer_backend.present();
        }
    }

    // pub fn draw_rect(&mut self, position: Vector2<f32>, height: f32, width: f32, color: Color) {
    pub fn draw_rect(&mut self) {
        let vertices: [Vertex; 4] = [
            Vertex::new(
                Vector2::new(-0.5, -0.5),
                Vector4::new(1.0, 0.0, 0.0, 1.0),
                Vector2::zero(),
            ),
            Vertex::new(
                Vector2::new(-0.5, 0.5),
                Vector4::new(1.0, 0.0, 0.0, 1.0),
                Vector2::zero()
            ),
            Vertex::new(
                Vector2::new(0.5, 0.5),
                Vector4::new(1.0, 0.0, 0.0, 1.0),
                Vector2::zero(),
            ),
            Vertex::new(
                Vector2::new(0.5, -0.5),
                Vector4::new(1.0, 0.0, 0.0, 1.0),
                Vector2::zero(),
            )
        ];
        
        let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

        self.triangle_buffer.extend_vertex_buffer(&vertices);
        self.triangle_buffer.extend_index_buffer(&indices);
    }
    
    pub fn draw_line_2d(&mut self, start_position: Vector2<f32>, end_position: Vector2<f32>, color: Color) {
        let vertices = [
            Vertex::new(end_position, color.to_vector4(), Vector2::zero()),
            Vertex::new(start_position, color.to_vector4(), Vector2::zero()),
        ];

        let indices = [0, 1, 1];
        
        self.line_buffer.extend_vertex_buffer(&vertices);
        self.line_buffer.extend_index_buffer(&indices);
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