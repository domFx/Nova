use crate::vulkan::vulkan_buffer::VulkanBuffer;
use crate::vulkan::constants::MAX_FRAMES_IN_FLIGHT;

use anyhow::Result;
use ash::vk;
use cgmath::{Matrix4, SquareMatrix};
use std::mem::size_of;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UniformBufferObject {
    view: Matrix4<f32>,
    proj: Matrix4<f32>
}

pub fn create_uniform_buffers(
    instance: &ash::Instance,
    logical_device: &ash::Device,
    physical_device: vk::PhysicalDevice,
) -> Result<Vec<VulkanBuffer>> {
    let mut uniform_buffers = Vec::new();

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        let vulkan_app_buffer = VulkanBuffer::new(
            instance,
            logical_device,
            physical_device,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        uniform_buffers.push(vulkan_app_buffer);
    }

    Ok(uniform_buffers)
}

pub fn update_uniform_buffer(
    image_index: usize,
    uniform_buffers: &Vec<VulkanBuffer>,
    _extent: vk::Extent2D,
    logical_device: &ash::Device,
    _start: &std::time::Instant,
) -> Result<()> {
    // let time = start.elapsed().as_secs_f32();

    let model: Matrix4<f32> = Matrix4::identity();
    let view = Matrix4::identity();
    let proj = Matrix4::identity();

    let ubo = UniformBufferObject { view, proj };

    unsafe {
        let memory = logical_device.map_memory(
            uniform_buffers[image_index].buffer_memory,
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        std::ptr::copy_nonoverlapping(&ubo as *const UniformBufferObject, memory.cast(), 1);

        logical_device.unmap_memory(uniform_buffers[image_index].buffer_memory);
    }

    Ok(())
}
