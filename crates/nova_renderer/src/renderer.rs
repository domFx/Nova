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

struct QuadDrawables {
    quad_count: usize,
    vertex_buffer: Vec<Option<Vertex>>,
    index_buffer: Vec<u32>,
}

impl QuadDrawables {
    const INDICES: &'static [u16] = &[0, 1, 2, 2, 3, 0];
    const MAX_QUADS: usize = 200;
    const NUM_VERTICES: usize = 4;
    const NUM_INDICES: usize = 6;

    pub fn new_with_max(max_quads: usize) -> Self {
        let mut index_buffer = vec![0; max_quads * Self::NUM_INDICES];

        let mut indices_idx = 0;
        for i in 0 .. max_quads * Self::NUM_INDICES {
            index_buffer[i] = Self::INDICES[indices_idx];

            indices_idx += 1;
            if indices_idx == Self::NUM_INDICES {
                indices_idx = 0
            }
        }

        Self {
            quad_count: 0,
            vertex_buffer: vec![None; max_quads * Self::NUM_VERTICES],
            index_buffer: vec![0; max_quads * Self::NUM_INDICES]
        }
    }

    pub fn new() -> Self {
        Self::new_with_max(Self::MAX_QUADS)
    }

    pub fn add_quad(&mut self, vertices: Vec<Vertex>) {
        assert!(self.quad_count < Self::MAX_QUADS - 1, "Expected quad count {} to be less than max quads {}", self.quad_count, Self::MAX_QUADS);
        assert_eq!(vertices.len(), Self::NUM_VERTICES, "Expected {} vertices", Self::NUM_VERTICES);

        // Push to graphics card
        if self.quad_count > self.get_max_quads() {
            return;
        }

        let vertex_buffer_idx = self.quad_count * Self::NUM_VERTICES;

        vertices
            .iter()
            .enumerate()
            .for_each(|(i, vertex)| { self.vertex_buffer[vertex_buffer_idx + i] = Some(*vertex); });
        self.quad_count += 1;
    }

    pub fn get_max_quads(&self) -> usize {
        self.vertex_buffer.capacity() / Self::NUM_VERTICES
    }

    pub fn get_quad_count(&self) -> usize {
        self.quad_count
    }
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

#[cfg(test)]
mod quad_drawable_tests {
    use nova_math::vector2::Vector2;
    use nova_math::vector4::Vector4;
    use crate::renderer::QuadDrawables;
    use crate::vertex::Vertex;

    #[test]
    fn add_quad_tests() {
        const MAX_QUADS: usize = 2;
        let mut quads = QuadDrawables::new_with_max(MAX_QUADS);

        let quad = vec![
            Vertex::new(Vector2::zero(), Vector4::zero(), Vector2::zero()),
            Vertex::new(Vector2::zero(), Vector4::zero(), Vector2::zero()),
            Vertex::new(Vector2::zero(), Vector4::zero(), Vector2::zero()),
            Vertex::new(Vector2::zero(), Vector4::zero(), Vector2::zero())
        ];

        quads.add_quad(quad.clone());

        assert_eq!(quads.get_max_quads(), MAX_QUADS);
        assert_eq!(quads.get_quad_count(), 1);

        quads.add_quad(quad.clone());
        assert_eq!(quads.get_quad_count(), MAX_QUADS);
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
