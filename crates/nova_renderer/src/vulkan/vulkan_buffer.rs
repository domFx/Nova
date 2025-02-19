use crate::vulkan::memory::get_memory_type_index;
use crate::vulkan::utils::single_time_commands::*;

use anyhow::Result;
use ash::vk;

pub(crate) enum VulkanBufferType { 
    Vertex,
    Index
}

#[derive(Copy, Clone)]
pub(crate) struct VulkanAppBuffer {
    pub(crate) buffer: vk::Buffer,
    pub(crate) buffer_memory: vk::DeviceMemory
}

impl VulkanAppBuffer {
    pub(crate) fn new_staged<T>(
        input_buffer: *const T,
        input_length: usize,
        logical_device: &ash::Device,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        graphics_queue: vk::Queue,
        command_pool: vk::CommandPool,
        buffer_type: VulkanBufferType
    ) -> Result<VulkanAppBuffer> {
        let size = (size_of::<T>() * input_length) as u64;

        let staging_buffer = Self::new_internal(
            instance,
            logical_device,
            physical_device,
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        unsafe {
            let memory = logical_device.map_memory(
                staging_buffer.buffer_memory,
                0,
                size,
                vk::MemoryMapFlags::empty(),
            )?;

            std::ptr::copy_nonoverlapping(input_buffer, memory.cast(), input_length);

            logical_device.unmap_memory(staging_buffer.buffer_memory);
        }

        let additional_destination_properties = match buffer_type {
            VulkanBufferType::Vertex => vk::BufferUsageFlags::VERTEX_BUFFER,
            VulkanBufferType::Index => vk::BufferUsageFlags::INDEX_BUFFER
        };

        let destination_buffer = Self::new_internal(
            instance,
            logical_device,
            physical_device,
            size,
            vk::BufferUsageFlags::TRANSFER_DST |  additional_destination_properties,
            vk::MemoryPropertyFlags::DEVICE_LOCAL
        )?;

        // Copy
        Self::copy_to_destination(logical_device, staging_buffer, destination_buffer, size, graphics_queue, command_pool)?;

        staging_buffer.destroy_buffer(logical_device)?;

        Ok(destination_buffer)
    }

    pub fn new(
        instance: &ash::Instance,
        logical_device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags
    ) -> Result<VulkanAppBuffer> {
        Self::new_internal(
            instance,
            logical_device,
            physical_device,
            size,
            usage,
            properties
        )
    }

    fn new_internal(
        instance: &ash::Instance,
        logical_device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags
    ) -> Result<VulkanAppBuffer> {
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        unsafe {
            let buffer = logical_device.create_buffer(&buffer_info, None)?;

            let requirements = logical_device.get_buffer_memory_requirements(buffer);

            let memory_info = vk::MemoryAllocateInfo::default()
                .allocation_size(requirements.size)
                .memory_type_index(get_memory_type_index(
                    instance,
                    physical_device,
                    properties,
                    requirements
                )?);

            let buffer_memory = logical_device.allocate_memory(&memory_info, None)?;

            logical_device.bind_buffer_memory(buffer, buffer_memory, 0)?;

            Ok(Self {
                buffer,
                buffer_memory
            })
        }
    }

    pub(crate) fn destroy_buffer(&self, logical_device: &ash::Device) -> Result<()> {
        unsafe {
            logical_device.destroy_buffer(self.buffer, None);
            logical_device.free_memory(self.buffer_memory, None);
        }

        Ok(())
    }

    fn copy_to_destination(
        logical_device: &ash::Device,
        source: Self,
        destination: Self,
        size: vk::DeviceSize,
        graphics_queue: vk::Queue,
        command_pool: vk::CommandPool
    ) -> Result<()> {
        let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

        let regions = vk::BufferCopy::default()
            .src_offset(0)
            .dst_offset(0)
            .size(size);

        unsafe {
            logical_device.cmd_copy_buffer(command_buffer, source.buffer, destination.buffer, &[regions]);
        }

        end_single_time_commands(logical_device, graphics_queue, command_pool, command_buffer)?;

        Ok(())
    }

    pub(crate) fn copy_buffer(
        &self,
        logical_device: &ash::Device,
        destination: vk::Buffer,
        size: vk::DeviceSize,
        graphics_queue: vk::Queue,
        command_pool: vk::CommandPool
    ) -> Result<()> {
        let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

        let regions = vk::BufferCopy::default()
            .src_offset(0)
            .dst_offset(0)
            .size(size);

        unsafe {
            logical_device.cmd_copy_buffer(command_buffer, self.buffer, destination, &[regions]);
        }

        end_single_time_commands(logical_device, graphics_queue, command_pool, command_buffer)?;

        Ok(())
    }
}