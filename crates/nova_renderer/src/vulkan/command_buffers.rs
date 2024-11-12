use anyhow::Result;
use ash::vk;

pub(crate) fn create_command_buffers(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    framebuffers: &Vec<vk::Framebuffer>,
) -> Result<Vec<vk::CommandBuffer>> {
    let info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(framebuffers.len() as u32);

    unsafe {
        let command_buffers = device.allocate_command_buffers(&info)?;
        Ok(command_buffers)
    }
}