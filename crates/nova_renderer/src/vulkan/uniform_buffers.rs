use crate::vulkan::vulkan_buffer::VulkanAppBuffer;
use crate::vulkan::constants::MAX_FRAMES_IN_FLIGHT;

use anyhow::Result;
use ash::vk;
use cgmath::{Matrix4, SquareMatrix};
use std::mem::size_of;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UniformBufferObject {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>,
}

pub fn create_uniform_buffers(
    instance: &ash::Instance,
    logical_device: &ash::Device,
    physical_device: vk::PhysicalDevice,
) -> Result<Vec<VulkanAppBuffer>> {
    let mut uniform_buffers = Vec::new();

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        let vulkan_app_buffer = VulkanAppBuffer::new(
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
    uniform_buffers: &Vec<VulkanAppBuffer>,
    _extent: vk::Extent2D,
    logical_device: &ash::Device,
    _start: &std::time::Instant,
) -> Result<()> {
    // let time = start.elapsed().as_secs_f32();

    let model = Matrix4::identity();

    // from_axis_angle(
    //     vec3(0.0, 0.0, 1.0),
    //     Deg(90.0)
    // );
    //
    // let view = Matrix4::look_at_rh(
    //     point3(0.0, 0.0, 1.0),
    //     point3(0.0, 0.0, 0.0),
    //     vec3(0.0, -1.0, 0.0)
    // );
    //
    let view = Matrix4::identity();

    // let left = 0.0;
    // let right = extent.width as f32;
    // let bottom = 0.0;
    // let top = extent.height as f32;
    // let near = 0.0;
    // let far = 1.0;

    // let mut proj = cgmath::ortho(left, right, bottom, top, near, far);
    let proj = Matrix4::identity();
    // let mut proj = cgmath::perspective(
    //     Deg(45.0),
    //     extent.width as f32 / extent.height as f32,
    //     0.1,
    //     10.0
    // );

    // proj[1][1] *= -1.0;

    let ubo = UniformBufferObject { model, view, proj };

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
