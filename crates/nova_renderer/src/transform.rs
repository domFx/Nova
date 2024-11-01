use nova_math::vector::vector2::Vector2;

#[derive(Debug, Default, Copy, Clone)]
pub struct Transform {
    position: Vector2<f32>,
    rotation: f32,
    scale: Vector2<f32>
}

impl Transform {
    /// Create an empty transform
    pub fn zero() -> Self {
        Transform::default()
    }

    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Self {
        Self {
            position,
            rotation,
            scale
        }
    }

    // /// Computes the model matrix for the transform.
    // pub fn to_model_matrix(&self) -> Mat4 {
    //     let translation = Mat4::from_translation(self.position.extend(0.0));
    //     let rotation = Mat4::from_rotation_z(self.rotation); // Rotate around Z-axis
    //     let scaling = Mat4::from_scale(self.scale.extend(1.0));
    //     translation * rotation * scaling
    // }
}

