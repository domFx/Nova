use crate::vertex::Vertex;
use crate::vulkan::vulkan_buffer::VulkanBuffer;

use anyhow::Result;
use ash::vk;
use nova_core::color::{Color};
use nova_core::scene::SceneBase;
use crate::vulkan::utils::single_time_commands::*;

// TODO: Potentially introduce state (init/recording/submitted)
pub(crate) struct VulkanRenderCommand<'a> {
    logical_device: &'a ash::Device,
    instance: &'a ash::Instance,
    physical_device: vk::PhysicalDevice,
    // Command buffer to record to - created on init
    command_buffer: vk::CommandBuffer,
    // Command pool - cleanup command buffer that was created after use
    command_pool: vk::CommandPool,
    // Graphics queue to submit to
    graphics_queue: vk::Queue,


    clear_color: Color
}

impl<'a> VulkanRenderCommand<'a> {
    pub(crate) fn new(
        logical_device: &'a ash::Device,
        instance: &'a ash::Instance,
        physical_device: vk::PhysicalDevice,
        command_pool: vk::CommandPool,
        graphics_queue: vk::Queue
    ) -> Result<Self> {
        let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

        let render_command = Self {
            logical_device,
            instance,
            physical_device,
            command_pool,
            command_buffer,
            graphics_queue,
            clear_color: Color::WHITE
        };

        Ok(render_command)
    }
}

// impl RenderCommand for VulkanRenderCommand<'_> {
//     // Could maybe move the VulkanRenderCommand::new code into here, or call begin_single_time_commands here
//     // Could be enforced that this is called first with state using phantom data.
//     fn begin_drawing(&self, scene: &SceneBase) -> Result<()> {
//         Ok(())
//     }

//     fn end_drawing(&self, scene: &SceneBase) -> Result<()> {
//         end_single_time_commands(
//             self.logical_device,
//             self.graphics_queue,
//             self.command_pool,
//             self.command_buffer
//         )?;

//         Ok(())
//     }
    
//     fn clear_background(&mut self, color: nova_core::color::Color) -> Result<()> {
//         self.clear_color = color;

//         return Ok(())
//     }

//     fn bind_vertex_buffer(&self, vertex_buffer: Vec<Vertex>) -> Result<()> {
//         let buffer = VulkanAppBuffer::new_staged(
//             vertex_buffer.as_ptr(),
//             vertex_buffer.len(),
//             self.logical_device,
//             self.instance,
//             self.physical_device,
//             self.graphics_queue,
//             self.command_pool,
//             vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
//             vk::MemoryPropertyFlags::DEVICE_LOCAL
//         )?;

//         unsafe {
//             self.logical_device.cmd_bind_vertex_buffers(self.command_buffer, 0, &[buffer.buffer], &[0])
//         }

//         Ok(())
//     }

//     fn bind_index_buffer(&self, _index_buffer: Vec<u32>) -> Result<()> {
//         // unsafe {
//             // self.logical_device.cmd_bind_index_buffer(self.command_buffer, vk::Buffer::null(), 0, vk::IndexType::UINT16);
//         // }

//         Ok(())
//     }

//     fn draw(&self) -> Result<()> {
//         todo!("Not implemented ")
//     }
// }

impl Drop for VulkanRenderCommand<'_> {
    fn drop(&mut self) {
        end_single_time_commands(self.logical_device, self.graphics_queue, self.command_pool, self.command_buffer)
            .expect("Unable to clean up Vulkan Render Command");
    }
}