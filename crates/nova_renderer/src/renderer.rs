use crate::core_types::PrimitiveTopologyType;
use crate::vulkan::vulkan_renderer::{VulkanRenderer, VulkanTopology};
use crate::renderer_backend::RendererBackend;
use crate::vertex::Vertex;

use nova_core::image::Image;
use nova_core::color::Color;
use nova_math::vector::{Vector2, Vector4};

use anyhow::{anyhow, Result};
use log::debug;
use raw_window_handle::{DisplayHandle, WindowHandle};
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
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

    pub fn add_to_buffer(&mut self, vertices: &[Vertex], indices: &[u32]) {
        let updated_indices: Vec<u32> = indices.iter().map(|i| self.vertex_buffer.len() as u32 + *i).collect::<Vec<u32>>();

        self.vertex_buffer.extend_from_slice(&vertices);
        self.index_buffer.extend_from_slice(&updated_indices);
    }

    pub fn reset(&mut self) {
        self.index_buffer.clear();
        self.vertex_buffer.clear();
    }
}

pub struct Renderer {
    renderer_backend: Option<Box<dyn RendererBackend>>,
    renderer_state: RendererState,
    line_buffer: RenderDrawBuffer,
    triangle_buffer: RenderDrawBuffer,
    images: Vec<Image>,
    window_dimensions: Vector2<u32>,
    window_scale_factor: f32
}

impl Renderer {
    /// Initializes a renderer with a Vulkan backend
    pub fn new_vulkan_renderer(
        window_handle: WindowHandle, 
        display_handle: DisplayHandle,
        window_dimensions: Vector2<u32>,
        window_scale_factor: f32
    ) -> Self {
        Self::new(window_handle, display_handle, window_dimensions, window_scale_factor, RendererBackendType::Vulkan)
    }

    pub fn new(
        window_handle: WindowHandle, 
        display_handle: DisplayHandle,
        window_dimensions: Vector2<u32>,
        window_scale_factor: f32,
        renderer_type: RendererBackendType
    ) -> Self {
        // Only one backend for now...
        let renderer_backend: Box<dyn RendererBackend> = match renderer_type {
            RendererBackendType::Vulkan => Box::from(
                VulkanRenderer::new(window_handle, display_handle, window_dimensions)
                    .expect("Failed to create Vulkan Renderer")
            )
        };
        
        let renderer_state = RendererState::Ready;

        Self {
            renderer_backend: Some(renderer_backend),
            renderer_state,
            line_buffer: RenderDrawBuffer::new(),
            triangle_buffer: RenderDrawBuffer::new(),
            images: Vec::new(),
            window_dimensions,
            window_scale_factor
        }
    }

    pub fn begin_rendering(&mut self, window_dimensions: Vector2<u32>) {
        self.images.clear();
        self.triangle_buffer.reset();
        self.line_buffer.reset();
        self.window_dimensions = window_dimensions;

        if let Some(renderer_backend) = &mut self.renderer_backend {
            renderer_backend.prepare_window_surface(window_dimensions);
            renderer_backend.begin_commands();
            renderer_backend.begin_render_pass(Color::BLACK);
        }
    }

    pub fn end_rendering(&mut self) {
        if let Some(renderer_backend) = &mut self.renderer_backend {
            // Upload images to GPU 
            if self.images.len() > 0 {
                renderer_backend.load_images(&self.images);
            }
            
            // Triangles
            if self.triangle_buffer.vertex_buffer.len() > 0 && self.triangle_buffer.index_buffer.len() > 0 {    
                renderer_backend.draw(self.triangle_buffer.vertex_buffer.clone(), self.triangle_buffer.index_buffer.clone(), PrimitiveTopologyType::TriangleList);
            }   
            
            // Lines
            if self.line_buffer.vertex_buffer.len() > 0 && self.line_buffer.index_buffer.len() > 0 {
                renderer_backend.draw(self.line_buffer.vertex_buffer.clone(), self.line_buffer.index_buffer.clone(), PrimitiveTopologyType::LineList);
            }
            
            renderer_backend.end_render_pass();
            renderer_backend.end_commands();
            renderer_backend.submit();
            renderer_backend.present();
        }
    }

    /// Draws a rectangle centered at the position
    pub fn draw_filled_rect(&mut self, position: Vector2<f32>, height: f32, width: f32, color: Color) {
        let vertices: [Vertex; 4] = [
            Vertex::new(
                Vector2::new(position.x - width / 2.0, position.y - height / 2.0),
                color.to_vector4(),
                Vector2::zero(),
            ),
            Vertex::new(
                Vector2::new(position.x - width / 2.0, position.y + height / 2.0),
                color.to_vector4(),
                Vector2::zero()
            ),
            Vertex::new(
                Vector2::new(position.x + width / 2.0, position.y + height / 2.0),
                color.to_vector4(),
                Vector2::zero(),
            ),
            Vertex::new(
                Vector2::new(position.x + width / 2.0, position.y - height / 2.0),
                color.to_vector4(),
                Vector2::zero(),
            )
        ]; 

        let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

        self.triangle_buffer.add_to_buffer(&vertices, &indices);
    }

    pub fn draw_rect(&mut self, position: Vector2<f32>, height: f32, width: f32, line_color: Color) {
        let vertices = [
            Vertex::new(Vector2::new(position.x - width / 2.0, position.y + height / 2.0), line_color.to_vector4(), Vector2::zero()),
            Vertex::new(Vector2::new(position.x + width / 2.0, position.y + height / 2.0), line_color.to_vector4(), Vector2::zero()),
            Vertex::new(Vector2::new(position.x + width / 2.0, position.y - height / 2.0), line_color.to_vector4(), Vector2::zero()),
            Vertex::new(Vector2::new(position.x - width / 2.0, position.y - height / 2.0), line_color.to_vector4(), Vector2::zero()),
        ];

        let indices = [0, 1, 1, 2, 2, 3, 3, 0];
        
        self.line_buffer.add_to_buffer(&vertices, &indices);
    }

    pub fn draw_line(&mut self, start_position: Vector2<f32>, end_position: Vector2<f32>, color: Color) {
        let vertices = [
            Vertex::new(end_position, color.to_vector4(), Vector2::zero()),
            Vertex::new(start_position, color.to_vector4(), Vector2::zero()),
        ];

        let indices = [0, 1];
        
        self.line_buffer.add_to_buffer(&vertices, &indices);
    }

    pub fn draw_image(&mut self, position: Vector2<f32>, image: Image) {
        let width = get_ndc_width(image.width() as f32 * self.window_scale_factor, self.window_dimensions.x as f32);
        let height = get_ndc_height(image.height() as f32 * self.window_scale_factor, self.window_dimensions.y as f32);

        let line_color = Color::WHITE;

        self.images.push(image);

        let image_index = self.images.len() as i32 - 1;

        let vertices = [
            Vertex::new_with_texture(Vector2::new(position.x - width / 2.0, position.y + height / 2.0), line_color.to_vector4(), Vector2::new(0f32, 0f32), image_index),
            Vertex::new_with_texture(Vector2::new(position.x + width / 2.0, position.y + height / 2.0), line_color.to_vector4(), Vector2::new(1f32, 0f32), image_index),
            Vertex::new_with_texture(Vector2::new(position.x + width / 2.0, position.y - height / 2.0), line_color.to_vector4(), Vector2::new(1f32, 1f32), image_index),
            Vertex::new_with_texture(Vector2::new(position.x - width / 2.0, position.y - height / 2.0), line_color.to_vector4(), Vector2::new(0f32, 1f32), image_index),
        ];

        let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];
        
        self.triangle_buffer.add_to_buffer(&vertices, &indices);
    }

    pub fn destroy(&mut self) {
        if let Some(ref mut renderer_backend) = self.renderer_backend {
            renderer_backend.destroy();
        }

        self.renderer_backend = None;
    }
}

/// Convert from screen space to NDC coordinates
fn to_ndc_position(position: Vector2<f32>, window_dimensions: Vector2<u32>) -> Vector2<f32> {
    let x = (position.x / window_dimensions.x as f32) * 2.0 - 1.0;
    let y = (position.y / window_dimensions.y as f32) * 2.0 - 1.0;

    Vector2::new(x, y)
}

/// Convert from screen dimensions to NDC dimensions
fn to_ndc_dimensions(dimensions: Vector2<f32>, window_dimensions: Vector2<u32>) -> Vector2<f32> {
    let x = (dimensions.x / window_dimensions.x as f32) * 2.0;
    let y = (dimensions.y / window_dimensions.y as f32) * 2.0;

    Vector2::new(x, y)
}

fn get_ndc_width(width: f32, screen_width: f32) -> f32 {
    (2.0f32 / screen_width as f32) * width
}

fn get_ndc_height(height: f32, screen_height: f32) -> f32 {
    (2.0f32 / screen_height as f32) * height
}