use crate::vulkan::buffer::VulkanAppBuffer;

use anyhow::Result;
use ash::vk;
use cgmath::vec3;
use nova_math::vector::vector2::Vector2;
use std::mem::size_of;

type Vec2 = nova_math::vector::vector2::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: Vec2,
    color: Vec3,
    tex_coord: Vec2,
}

impl Vertex {
    pub const fn new(pos: Vec2, color: Vec3, tex_coord: Vec2) -> Self {
        Self {
            pos,
            color,
            tex_coord,
        }
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
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

        let text_coord = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(size_of::<Vec2>() as u32 + size_of::<Vec3>() as u32)
            .build();

        [pos, color, text_coord]
    }
}

pub unsafe fn create_vertex_buffer(
    instance: &ash::Instance,
    logical_device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    command_pool: vk::CommandPool,
) -> Result<VulkanAppBuffer> {
    let size = (size_of::<Vertex>() * VERTICES.len()) as u64;

    let staging_buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = logical_device.map_memory(
        staging_buffer.buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    std::ptr::copy_nonoverlapping(VERTICES.as_ptr(), memory.cast(), VERTICES.len());

    logical_device.unmap_memory(staging_buffer.buffer_memory);

    let vertex_buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    staging_buffer.copy_buffer(
        logical_device,
        vertex_buffer.buffer,
        size,
        graphics_queue,
        command_pool,
    )?;

    logical_device.destroy_buffer(staging_buffer.buffer, None);
    logical_device.free_memory(staging_buffer.buffer_memory, None);

    Ok(vertex_buffer)
}

pub(crate) static VERTICES: [Vertex; 4] = [
    Vertex::new(
        Vector2::new(-0.5, -0.5),
        vec3(1.0, 0.0, 0.0),
        Vector2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vector2::new(0.5, -0.5),
        vec3(0.0, 1.0, 0.0),
        Vector2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vector2::new(0.5, 0.5),
        vec3(0.0, 0.0, 1.0),
        Vector2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vector2::new(-0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        Vector2::new(1.0, 1.0),
    ),
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
