use crate::vertex::Vertex;

use anyhow::Result;

pub(crate) trait RenderCommand {
    fn bind_vertex_buffer(&self, vertex_buffer: Vec<Vertex>) -> Result<()>;
    fn bind_index_buffer(&self, index_buffer: Vec<u32>) -> Result<()>;
    fn draw(&self) -> Result<()>;
}