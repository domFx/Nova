use crate::vertex::Vertex;
use nova_core::scene::Scene;

use anyhow::Result;

pub(crate) trait RenderCommand {
    fn begin_scene(&self, scene: &Scene) -> Result<()>;
    fn end_scene(&self, scene: &Scene) -> Result<()>;
    fn bind_vertex_buffer(&self, vertex_buffer: Vec<Vertex>) -> Result<()>;
    fn bind_index_buffer(&self, index_buffer: Vec<u32>) -> Result<()>;
    fn draw(&self) -> Result<()>;
}