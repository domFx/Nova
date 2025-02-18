use crate::vulkan::vulkan_buffer::{VulkanAppBuffer,VulkanBufferType};
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