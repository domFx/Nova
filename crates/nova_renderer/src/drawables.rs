struct QuadDrawables {
    quad_count: usize,
    vertex_buffer: Vec<Option<Vertex>>,
    index_buffer: Vec<u32>,
}

impl QuadDrawables {
    const INDICES: &[u16; 6] = &[0, 1, 2, 2, 3, 0];
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
