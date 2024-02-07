use std::mem::size_of;
use anyhow::{Result};
use ash::{Instance, vk};
use ash::Device;
use ash::vk::{CommandPool, PhysicalDevice};

use cgmath::{vec2, vec3};
use crate::vulkan::buffer::VulkanAppBuffer;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: Vec2,
    color: Vec3
}

impl Vertex {
    pub const fn new(pos: Vec2, color: Vec3) -> Self {
        Self {pos, color}
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)
            .build();

        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<Vec2>() as u32)
            .build();

        [pos, color]
    }
}

pub unsafe fn create_vertex_buffer(
    instance: &Instance,
    logical_device: &Device,
    physical_device: PhysicalDevice,
    graphics_queue: vk::Queue,
    command_pool: CommandPool
) -> Result<VulkanAppBuffer> {
    let size = (size_of::<Vertex>() * VERTICES.len()) as u64;

    let staging_buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE
    )?;

    let memory = logical_device.map_memory(
        staging_buffer.buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty()
    )?;

    std::ptr::copy_nonoverlapping(VERTICES.as_ptr(), memory.cast(), VERTICES.len());

    logical_device.unmap_memory(staging_buffer.buffer_memory);

    let vertex_buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL
    )?;

    staging_buffer.copy_buffer(logical_device, vertex_buffer.buffer, size, graphics_queue, command_pool)?;

    logical_device.destroy_buffer(staging_buffer.buffer, None);
    logical_device.free_memory(staging_buffer.buffer_memory, None);

    Ok(vertex_buffer)
}

pub static VERTICES: [Vertex; 4] = [
    Vertex::new(vec2(-0.5, -0.5), vec3(1.0, 0.0, 0.0)),
    Vertex::new(vec2(0.5, -0.5), vec3(0.0, 1.0, 0.0)),
    Vertex::new(vec2(0.5, 0.5), vec3(0.0, 0.0, 1.0)),
    Vertex::new(vec2(-0.5, 0.5), vec3(1.0, 1.0, 1.0))
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];