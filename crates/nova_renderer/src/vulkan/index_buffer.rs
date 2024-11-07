use crate::vulkan::vulkan_buffer::VulkanAppBuffer;
use crate::vulkan::vulkan_vertex::INDICES;

use anyhow::Result;
use ash::vk;

pub(crate) fn create_index_buffer(
    instance: &ash::Instance,
    logical_device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    command_pool: vk::CommandPool,
) -> Result<VulkanAppBuffer> {
    let index_buffer = VulkanAppBuffer::new_staged(
        INDICES.as_ptr(),
        INDICES.len(),
        logical_device,
        instance,
        physical_device,
        graphics_queue,
        command_pool,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL
    )?;

    Ok(index_buffer)
}
