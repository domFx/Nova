use std::thread::AccessError;
use ash::vk::{AccessFlags, Buffer, BufferImageCopy, BufferMemoryBarrier, CommandPool, DependencyFlags, DeviceMemory, DeviceSize, Extent3D, Format, Image, ImageAspectFlags, ImageCreateFlags, ImageLayout, ImageMemoryBarrier, ImageSubresource, ImageSubresourceLayers, ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, MemoryBarrier, MemoryPropertyFlags, Offset3D, PhysicalDevice, PipelineStageFlags, Queue, QUEUE_FAMILY_IGNORED, SampleCountFlags, SharingMode};
use anyhow::Result;
use ash::{Device, Instance, vk};
use log::LevelFilter::Off;
use crate::vulkan::buffer::{begin_single_time_commands, end_single_time_commands, VulkanAppBuffer};
use crate::vulkan::memory::get_memory_type_index;

pub(crate) struct VulkanImage {
    pub(crate) texture_image: Image,
    pub(crate) texture_image_memory: DeviceMemory
}

pub(crate) fn create_texture_image(
    instance: &Instance,
    logical_device: &Device,
    physical_device: PhysicalDevice
) -> Result<VulkanImage> {
    let image = image::io::Reader::open("./src/images/texture.jpg")?.decode()?;

    let device_size: DeviceSize = (image.height() * image.width() * 4) as DeviceSize;

    let buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        device_size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT
    )?;

    unsafe {
        let memory = logical_device.map_memory(
            buffer.buffer_memory,
            0,
            device_size,
            vk::MemoryMapFlags::empty()
        )?;

        std::ptr::copy_nonoverlapping(image.as_bytes().as_ptr(), memory.cast(), image.as_bytes().len());

        logical_device.unmap_memory(buffer.buffer_memory);
    }

    let image_create_info = vk::ImageCreateInfo::builder()
        .image_type(ImageType::TYPE_2D)
        .extent(Extent3D { width: image.width(), height: image.height(), depth: 1 })
        .array_layers(1)
        .mip_levels(1)
        .format(Format::R8G8B8A8_SRGB)
        .tiling(ImageTiling::OPTIMAL)
        .initial_layout(ImageLayout::UNDEFINED)
        .usage(ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED)
        .sharing_mode(SharingMode::EXCLUSIVE)
        .samples(SampleCountFlags::TYPE_1)
        .flags(ImageCreateFlags::empty())
        .build();

    unsafe {
        let texture_image = logical_device.create_image(&image_create_info, None)?;
        let memory_requirements = logical_device.get_image_memory_requirements(texture_image);

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(get_memory_type_index(
                instance,
                physical_device,
                MemoryPropertyFlags::DEVICE_LOCAL,
                memory_requirements
            )?)
            .build();

        let texture_image_memory = logical_device.allocate_memory(&alloc_info, None)?;

        logical_device.bind_image_memory(texture_image, texture_image_memory, 0)?;

        buffer.destroy_buffer(logical_device);

        Ok(VulkanImage {
            texture_image,
            texture_image_memory
        })
    }
}

pub(crate) fn transition_image_layout(
    logical_device: &Device,
    command_pool: CommandPool,
    graphics_queue: Queue,
    image: Image,
    format: Format,
    old_layout: ImageLayout,
    new_layout: ImageLayout
) -> Result<()> {
    let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

    let subresource = ImageSubresourceRange::builder()
        .aspect_mask(ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let barrier = ImageMemoryBarrier::builder()
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(subresource)
        .src_access_mask(AccessFlags::empty())
        .dst_access_mask(AccessFlags::empty())
        .build();

    unsafe {
        logical_device.cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::empty(),
            PipelineStageFlags::empty(),
            DependencyFlags::empty(),
            &[] as &[MemoryBarrier],
            &[] as &[BufferMemoryBarrier],
            &[barrier]
        );
    }

    end_single_time_commands(logical_device, graphics_queue, command_pool, command_buffer)?;

    Ok(())
}

pub(crate) fn copy_buffer_to_image(
    logical_device: &Device,
    command_pool: CommandPool,
    graphics_queue: Queue,
    buffer: Buffer,
    image: Image,
    width: u32,
    height: u32
) -> Result<()> {
    let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

    let subresource = ImageSubresourceLayers::builder()
        .aspect_mask(ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let region = BufferImageCopy::builder()
        .buffer_offset(0)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(subresource)
        .image_offset(Offset3D { x: 0, y: 0, z: 0 })
        .image_extent(Extent3D { width, height, depth: 1 })
        .build();

    unsafe {
        logical_device.cmd_copy_buffer_to_image(
            command_buffer,
            buffer,
            image,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region]
        );
    }

    end_single_time_commands(logical_device, graphics_queue, command_pool, command_buffer)?;

    Ok(())
}