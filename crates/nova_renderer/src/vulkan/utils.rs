/// Vulkan Utility Functions

/// Single Time Commands Utils
/// Begin and end single time command
pub(crate) mod single_time_commands {
    use ash::vk;

    pub(crate) fn begin_single_time_commands(
        logical_device: &ash::Device,
        command_pool: vk::CommandPool
    ) -> anyhow::Result<vk::CommandBuffer> {
        let buffer_allocation_info = vk::CommandBufferAllocateInfo::builder()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1)
            .build();

        let buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        unsafe {
            let command_buffer = logical_device.allocate_command_buffers(&buffer_allocation_info)?[0];

            logical_device.begin_command_buffer(command_buffer, &buffer_begin_info)?;
            Ok(command_buffer)
        }
    }

    pub(crate) fn end_single_time_commands(
        logical_device: &ash::Device,
        graphics_queue: vk::Queue,
        command_pool: vk::CommandPool,
        command_buffer: vk::CommandBuffer
    ) -> anyhow::Result<()> {
        unsafe {

            logical_device.end_command_buffer(command_buffer)?;

            let command_buffers = &[command_buffer];
            let submit_info = vk::SubmitInfo::builder()
                .command_buffers(command_buffers)
                .build();

            logical_device.queue_submit(graphics_queue, &[submit_info], vk::Fence::null())?;
            logical_device.queue_wait_idle(graphics_queue)?;

            logical_device.free_command_buffers(command_pool, command_buffers);
        }

        Ok(())
    }
}

/// Shader Utils
pub(crate) mod shader {
    use ash::vk;

    pub(crate) fn create_shader_module(
        device: &ash::Device,
        bytecode: &[u32]
    ) -> anyhow::Result<vk::ShaderModule> {
        let info = vk::ShaderModuleCreateInfo::builder()
            .code(bytecode)
            .build();

        unsafe {
            let shader_module = device.create_shader_module(&info, None)?;

            Ok(shader_module)
        }
    }
}