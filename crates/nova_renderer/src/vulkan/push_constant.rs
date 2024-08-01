use anyhow::Result;
use ash::vk;
use cgmath::Matrix4;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct PushConstant {
    model_matrix: Matrix4<f32>
}

pub(crate) fn create_push_constant() -> Result<()> {
    let push_constant_range = vk::PushConstantRange::builder()
        .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
        .offset(0)
        .size(size_of::<PushConstant>() as u32)
        .build();

    Ok(())
}