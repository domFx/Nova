use nova_math::vector::Vector3;

#[derive(Clone, Copy)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>
}

impl Transform {
    pub(crate) fn new(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Transform {
        Transform {
            position,
            rotation,
            scale
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }
}

impl Default for Transform {
    fn default() -> Self {
        // TODO: May not be needed
        Self {
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::zero()
        }
    }
}