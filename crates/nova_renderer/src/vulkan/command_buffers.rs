use crate::vulkan::vertex::INDICES;

use anyhow::Result;
use ash::vk;

pub fn create_command_buffers(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    framebuffers: &Vec<vk::Framebuffer>,
) -> Result<Vec<vk::CommandBuffer>> {
    let info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(framebuffers.len() as u32)
        .build();

    unsafe {
        let command_buffers = device.allocate_command_buffers(&info)?;
        Ok(command_buffers)
    }
}