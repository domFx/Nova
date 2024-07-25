use crate::vulkan::memory::get_memory_type_index;

use anyhow::Result;
use ash::{Device, Instance, vk};
use ash::vk::{Buffer, BufferUsageFlags, CommandBuffer, CommandPool, DeviceMemory, DeviceSize, MemoryPropertyFlags, PhysicalDevice, Queue};

pub(crate) struct VulkanAppBuffer {
    pub(crate) buffer: Buffer,
    pub(crate) buffer_memory: DeviceMemory
}

impl VulkanAppBuffer {
    pub(crate) fn new(
        instance: &Instance,
        logical_device: &Device,
        physical_device: PhysicalDevice,
        size: DeviceSize,
        usage: BufferUsageFlags,
        properties: MemoryPropertyFlags
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

    pub(crate) fn destroy_buffer(&self, logical_device: &Device) {
        unsafe {
            logical_device.destroy_buffer(self.buffer, None);
            logical_device.free_memory(self.buffer_memory, None);
        }
    }

    pub(crate) fn copy_buffer(
        &self,
        logical_device: &Device,
        destination: Buffer,
        size: DeviceSize,
        graphics_queue: Queue,
        command_pool: CommandPool
    ) -> Result<()> {
        let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

        let regions = vk::BufferCopy::builder()
            .size(size)
            .build();

        unsafe {
            logical_device.cmd_copy_buffer(command_buffer, self.buffer, destination, &[regions]);
        }

        end_single_time_commands(logical_device, graphics_queue, command_pool, command_buffer)?;

        Ok(())
    }
}

pub(crate) fn begin_single_time_commands(
    logical_device: &Device,
    command_pool: CommandPool
) -> Result<CommandBuffer> {
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
    logical_device: &Device,
    graphics_queue: Queue,
    command_pool: CommandPool,
    command_buffer: CommandBuffer
) -> Result<()> {
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

