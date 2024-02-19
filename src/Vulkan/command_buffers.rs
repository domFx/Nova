use anyhow::Result;
use ash::{Device, vk};
use ash::vk::{Buffer, CommandBuffer, CommandPool, DescriptorSet, Extent2D, Framebuffer, RenderPass};
use crate::vulkan::pipeline::VulkanAppPipeline;
use crate::vulkan::vertex::{INDICES};

pub unsafe fn create_command_buffers(
    device: &Device,
    command_pool: CommandPool,
    extent: Extent2D,
    render_pass: RenderPass,
    framebuffers: &Vec<Framebuffer>,
    pipeline: &VulkanAppPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    descriptor_sets: &Vec<DescriptorSet>
) -> Result<Vec<CommandBuffer>> {

    let info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(framebuffers.len() as u32)
        .build();

    let command_buffers = device.allocate_command_buffers(&info)?;

    for (i, command_buffer) in command_buffers.iter().enumerate() {
        let info = vk::CommandBufferBeginInfo::builder()
            .build();

        device.begin_command_buffer(*command_buffer, &info)?;

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(extent)
            .build();

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };

        let clear_values = &[color_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .framebuffer(framebuffers[i])
            .render_area(render_area)
            .clear_values(clear_values)
            .build();

        device.cmd_begin_render_pass(*command_buffer, &info, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline);
        device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertex_buffer], &[0]);
        device.cmd_bind_index_buffer(*command_buffer, index_buffer, 0, vk::IndexType::UINT16);
        device.cmd_bind_descriptor_sets(
            *command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline.pipeline_layout,
            0,
            &[descriptor_sets[i]],
            &[]);

        device.cmd_draw_indexed(*command_buffer, INDICES.len() as u32, 1, 0, 0, 0);
        device.cmd_end_render_pass(*command_buffer);

        device.end_command_buffer(*command_buffer)?;
    }

    Ok(command_buffers)
}