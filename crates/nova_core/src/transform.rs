use nova_math::vector3::Vector3;

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