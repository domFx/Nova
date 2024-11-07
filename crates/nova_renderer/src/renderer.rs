use crate::vulkan::vulkan_renderer::VulkanRenderer;
use crate::renderer_backend::RendererBackend;
use crate::transform::Transform;
use crate::vertex::Vertex;

use nova_math::vector::vector2::Vector2;
use nova_math::vector::vector4::Vector4;

use anyhow::Result;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

/// Core Renderable Object
trait Renderable {
    fn transform(&self) -> &Transform;
    fn z_index(&self) -> &i32;
}

#[derive(Debug, Copy, Clone)]
pub struct RectRenderable {
    // Transform data
    transform: Transform, // Model matrix for translation, rotation, scaling

    // Appearance data

    // Color: RGBA
    color: Vector4<f32>,
    // texture_id: u32,        // Texture index in an atlas or texture array

    // Render order (higher = on top)
    z_index: i32,

    // Vulkan-specific data (optional)
    // pipeline_layout: vk::PipelineLayout,
    // descriptor_set: vk::DescriptorSet,

    // Should the object be drawn
    is_visible: bool
}

impl Renderable for RectRenderable {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn z_index(&self) -> &i32 {
        &self.z_index
    }
}

impl RectRenderable {
    pub fn new(transform: Transform, color: Vector4<f32>, z_index: i32, is_visible: bool) -> Self {
        Self {
            transform,
            color,
            z_index,
            is_visible
        }
    }
}

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

pub struct Renderer {
    quads: QuadDrawables,

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

        let quads = QuadDrawables::new();

        Ok(Self {
            quads,

            resized: false,
            renderer_backend,
        })
    }

    pub fn render(&mut self, window_dimensions: Vector2<u32>) -> Result<()> {
        self.renderer_backend.render(window_dimensions)?;

        Ok(())
    }

    pub fn draw_rect(
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
    }

    pub fn destroy(&self) {
        self.renderer_backend.destroy();
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