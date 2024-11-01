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
        let size = (size_of::<Vertex>() * vertex_buffer.len()) as u64;

        unsafe {
            let staging_buffer = VulkanAppBuffer::new_staging_buffer(
                self.instance,
                self.logical_device,
                self.physical_device,
                size
            )?;

            let memory = self.logical_device.map_memory(
                staging_buffer.buffer_memory,
                0,
                size,
                vk::MemoryMapFlags::empty(),
            )?;

            std::ptr::copy_nonoverlapping(vertex_buffer.as_ptr(), memory.cast(), vertex_buffer.len());

            self.logical_device.unmap_memory(staging_buffer.buffer_memory);

            let vertex_buffer = VulkanAppBuffer::new(
                self.instance,
                self.logical_device,
                self.physical_device,
                size,
                vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )?;

            staging_buffer.copy_buffer(
                self.logical_device,
                vertex_buffer.buffer,
                size,
                self.graphics_queue,
                self.command_pool,
            )?;

            self.logical_device.destroy_buffer(staging_buffer.buffer, None);
            self.logical_device.free_memory(staging_buffer.buffer_memory, None);

            // self.logical_device.cmd_bind_vertex_buffers(self.command_buffer, 0, &vec![], &vec![]);
        }

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