use anyhow::Result;
use ash::{Device, Instance, vk};
use ash::vk::{Buffer, CommandPool, DeviceMemory, PhysicalDevice};
use crate::renderer::vulkan::memory::get_memory_type_index;

pub struct VulkanAppBuffer {
    pub buffer: Buffer,
    pub buffer_memory: DeviceMemory
}

impl VulkanAppBuffer {
    pub fn new(
        instance: &Instance,
        logical_device: &Device,
        physical_device: PhysicalDevice,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags
    ) -> Result<VulkanAppBuffer> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .build();

        unsafe {
            let buffer = logical_device.create_buffer(&buffer_info, None)?;

            let requirements = logical_device.get_buffer_memory_requirements(buffer);

            let memory_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(requirements.size)
                .memory_type_index(get_memory_type_index(
                    instance,
                    physical_device,
                    properties,
                    requirements
                )?)
                .build();

            let buffer_memory = logical_device.allocate_memory(&memory_info, None)?;

            logical_device.bind_buffer_memory(buffer, buffer_memory, 0)?;

            Ok(Self {
                buffer,
                buffer_memory
            })
        }
    }

    pub fn copy_buffer(
        &self,
        logical_device: &Device,
        destination: Buffer,
        size: vk::DeviceSize,
        graphics_queue: vk::Queue,
        command_pool: CommandPool
    ) -> Result<()> {
        unsafe {
            let buffer_allocation_info = vk::CommandBufferAllocateInfo::builder()
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_pool(command_pool)
                .command_buffer_count(1)
                .build();

            let command_buffer = logical_device.allocate_command_buffers(&buffer_allocation_info)?[0];

            let buffer_begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
                .build();

            logical_device.begin_command_buffer(command_buffer, &buffer_begin_info)?;

            let regions = vk::BufferCopy::builder()
                .size(size)
                .build();

            logical_device.cmd_copy_buffer(command_buffer, self.buffer, destination, &[regions]);

            logical_device.end_command_buffer(command_buffer)?;

            let submit_info = vk::SubmitInfo::builder()
                .command_buffers(&[command_buffer])
                .build();

            logical_device.queue_submit(graphics_queue, &[submit_info], vk::Fence::null())?;
            logical_device.queue_wait_idle(graphics_queue)?;

            logical_device.free_command_buffers(command_pool, &[command_buffer]);

            Ok(())
        }
    }
}

