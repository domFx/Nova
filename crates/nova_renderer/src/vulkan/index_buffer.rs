use std::mem::size_of;
use anyhow::Result;
use ash::{Device, vk, Instance};
use ash::vk::{CommandPool, PhysicalDevice, Queue};
use crate::vulkan::buffer::VulkanAppBuffer;
use crate::vulkan::vertex::{INDICES};

pub fn create_index_buffer(
    instance: &Instance,
    logical_device: &Device,
    physical_device: PhysicalDevice,
    graphics_queue: Queue,
    command_pool: CommandPool
) -> Result<VulkanAppBuffer> {
    let size = (size_of::<u16>() * INDICES.len()) as u64;

    let staging_buffer = VulkanAppBuffer::new(
        &instance,
        logical_device,
        physical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE
    )?;

    unsafe {
        let memory = logical_device.map_memory(
            staging_buffer.buffer_memory,
            0,
            size,
            vk::MemoryMapFlags::empty()
        )?;

        std::ptr::copy_nonoverlapping(INDICES.as_ptr(), memory.cast(), INDICES.len());

        logical_device.unmap_memory(staging_buffer.buffer_memory);
    }

    let index_buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL
    )?;

    staging_buffer.copy_buffer(logical_device, index_buffer.buffer, size, graphics_queue, command_pool)?;

    unsafe {
        logical_device.destroy_buffer(staging_buffer.buffer, None);
        logical_device.free_memory(staging_buffer.buffer_memory, None);
    }

    Ok(index_buffer)
}