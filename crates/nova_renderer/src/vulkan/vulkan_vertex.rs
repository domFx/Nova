use crate::vulkan::vulkan_buffer::VulkanAppBuffer;
use crate::vertex::Vertex;

use nova_math::vector::vector2::Vector2;
use nova_math::vector::vector3::Vector3;
use nova_math::vector::vector4::Vector4;

use anyhow::Result;
use ash::vk;
use std::mem::size_of;

impl Vertex {
    pub(crate) fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
    }

    pub(crate) fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        let pos = vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0);

        let color = vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<Vector2<f32>>() as u32);

        let text_coord = vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(2)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(size_of::<Vector2<f32>>() as u32 + size_of::<Vector3<f32>>() as u32);

        [pos, color, text_coord]
    }
}

pub(crate) fn create_vertex_buffer(
    instance: &ash::Instance,
    logical_device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    command_pool: vk::CommandPool,
) -> Result<VulkanAppBuffer> {
    let vertex_buffer = VulkanAppBuffer::new_staged(
        VERTICES.as_ptr(),
        VERTICES.len(),
        logical_device,
        instance,
        physical_device,
        graphics_queue,
        command_pool,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    Ok(vertex_buffer)
}

pub(crate) static VERTICES: [Vertex; 4] = [
    Vertex::new(
        Vector2::new(-0.5, -0.5),
        Vector4::new(1.0, 0.0, 0.0, 1.0),
        Vector2::new(1.0, 0.0),
    ),
    Vertex::new(
        Vector2::new(0.5, -0.5),
        Vector4::new(1.0, 0.0, 0.0, 1.0),
        Vector2::new(0.0, 0.0),
    ),
    Vertex::new(
        Vector2::new(0.5, 0.5),
        Vector4::new(1.0, 0.0, 0.0, 1.0),
        Vector2::new(0.0, 1.0),
    ),
    Vertex::new(
        Vector2::new(-0.5, 0.5),
        Vector4::new(1.0, 0.0, 0.0, 1.0),
        Vector2::new(1.0, 1.0),
    ),
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
