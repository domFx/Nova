use nova_math::vector::vector4::Vector4;

// Colors: RGBA
pub const WHITE: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
pub const BLACK: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 };

#[derive(Default, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_vector4(color: Vector4<f32>) -> Color {
        Color::new(color.x, color.y, color.z, color.w)
    }

    pub fn to_vector4(&self) -> Vector4<f32> {
        Vector4::new(self.r, self.g, self.b, self.a)
    }
}