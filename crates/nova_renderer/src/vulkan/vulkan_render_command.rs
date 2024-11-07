use std::process::Command;
use crate::render_command::RenderCommand;
use crate::vertex::Vertex;
use crate::vulkan::vulkan_buffer::VulkanAppBuffer;

use anyhow::Result;
use ash::vk;

pub(crate) struct VulkanRenderCommand<'a> {
    logical_device: &'a ash::Device,
    instance: &'a ash::Instance,
    physical_device: vk::PhysicalDevice,
    // command_buffer: vk::CommandBuffer,
    command_pool: vk::CommandPool,
    graphics_queue: vk::Queue
}

impl<'a> VulkanRenderCommand<'a> {
    pub(crate) fn new(
        logical_device: &'a ash::Device,
        instance: &'a ash::Instance,
        physical_device: vk::PhysicalDevice,
        command_pool: vk::CommandPool,
        graphics_queue: vk::Queue
    ) -> Self {
        Self {
            logical_device,
            instance,
            physical_device,
            command_pool,
            graphics_queue
        }
    }
}

impl RenderCommand for VulkanRenderCommand<'_> {
    fn bind_vertex_buffer(&self, vertex_buffer: Vec<Vertex>) -> Result<()> {
        // let buffer = VulkanAppBuffer::new_buffer()?;
        Ok(())
    }

    fn bind_index_buffer(&self, index_buffer: Vec<u32>) -> Result<()> {
        unsafe {
            // self.logical_device.cmd_bind_index_buffer(self.command_buffer, vk::Buffer::null(), 0, vk::IndexType::UINT16);
        }

        Ok(())
    }

    fn draw(&self) -> Result<()> {
        todo!("Not implemented ")
    }
}