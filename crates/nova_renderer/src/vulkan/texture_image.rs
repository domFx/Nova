use crate::vulkan::memory::get_memory_type_index;
use crate::vulkan::utils::single_time_commands::*;
use crate::vulkan::vulkan_buffer::VulkanAppBuffer;

use ash::vk;
use anyhow::{anyhow, Result};

pub(crate) struct VulkanImage {
    pub(crate) texture_image: vk::Image,
    pub(crate) texture_image_memory: vk::DeviceMemory,
    pub(crate) texture_image_view: vk::ImageView
}

impl VulkanImage {
    pub(crate) fn destroy_vulkan_image(&self, logical_device: &ash::Device) {
        unsafe {
            logical_device.destroy_image_view(self.texture_image_view, None);
            logical_device.free_memory(self.texture_image_memory, None);
            logical_device.destroy_image(self.texture_image, None);
        }
    }
}

pub(crate) fn create_texture_image(
    instance: &ash::Instance,
    logical_device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    command_pool: vk::CommandPool,
    graphics_queue: vk::Queue,
) -> Result<VulkanImage> {
    let image = std::fs::File::open("./src/images/texture.png")?;

    let decoder = png::Decoder::new(image);
    let mut reader = decoder.read_info()?;

    let mut pixels = vec![0; reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let size = reader.info().raw_bytes() as u64;
    let (width, height) = reader.info().size();

    // let image = image::ImageReader::open("./src/images/texture.png")?.decode()?;
    //
    // let width = image.width().clone();
    // let height = image.height().clone();
    // let size = image.clone().into_bytes().len() as u64;

    // let device_size: DeviceSize = (width * height * 4) as DeviceSize;

    let buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE
    )?;

    unsafe {
        let memory = logical_device.map_memory(
            buffer.buffer_memory,
            0,
            size,
            vk::MemoryMapFlags::empty()
        )?;

        std::ptr::copy_nonoverlapping(pixels.as_ptr(), memory.cast(), pixels.len());

        logical_device.unmap_memory(buffer.buffer_memory);
    }

    let image_create_info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::TYPE_2D)
        .extent(vk::Extent3D { width, height, depth: 1 })
        .array_layers(1)
        .mip_levels(1)
        .format(vk::Format::R8G8B8A8_SRGB)
        .tiling(vk::ImageTiling::OPTIMAL)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .samples(vk::SampleCountFlags::TYPE_1)
        .flags(vk::ImageCreateFlags::empty())
        .build();

    unsafe {
        let texture_image = logical_device.create_image(&image_create_info, None)?;
        let memory_requirements = logical_device.get_image_memory_requirements(texture_image);

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(get_memory_type_index(
                instance,
                physical_device,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                memory_requirements
            )?)
            .build();

        let texture_image_memory = logical_device.allocate_memory(&alloc_info, None)?;

        logical_device.bind_image_memory(texture_image, texture_image_memory, 0)?;

        let _ = transition_image_layout(
            &logical_device,
            command_pool,
            graphics_queue,
            texture_image,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL
        );

        let _ = copy_buffer_to_image(
            &logical_device,
            command_pool,
            graphics_queue,
            buffer.buffer,
            texture_image,
            width,
            height);

        let _ = transition_image_layout(
            &logical_device,
            command_pool,
            graphics_queue,
            texture_image,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        );

        buffer.destroy_buffer(logical_device);

        let texture_image_view = create_image_view(logical_device, texture_image, vk::Format::R8G8B8A8_SRGB)?;

        Ok(VulkanImage {
            texture_image,
            texture_image_memory,
            texture_image_view
        })
    }
}

pub(crate) fn create_image_view(
    logical_device: &ash::Device,
    image: vk::Image,
    format: vk::Format
) -> Result<vk::ImageView> {
    let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let info = vk::ImageViewCreateInfo::builder()
        .image(image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .subresource_range(subresource_range)
        .build();

    unsafe {
        let image_view = logical_device.create_image_view(&info, None)?;

        Ok(image_view)
    }
}

pub(crate) fn create_texture_sampler(device: &ash::Device) -> Result<vk::Sampler> {
    let sampler_info = vk::SamplerCreateInfo::builder()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::REPEAT)
        .address_mode_v(vk::SamplerAddressMode::REPEAT)
        .address_mode_w(vk::SamplerAddressMode::REPEAT)
        .anisotropy_enable(true)
        .max_anisotropy(16.0)
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false)
        .compare_enable(false)
        .compare_op(vk::CompareOp::ALWAYS)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .mip_lod_bias(0.0)
        .min_lod(0.0)
        .max_lod(0.0)
        .build();

    unsafe {
        let sampler = device.create_sampler(&sampler_info, None)?;

        Ok(sampler)
    }
}

pub(crate) fn transition_image_layout(
    logical_device: &ash::Device,
    command_pool: vk::CommandPool,
    graphics_queue: vk::Queue,
    image: vk::Image,
    format: vk::Format,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout
) -> Result<()> {
    let (src_access_mask, dst_access_mask, src_stage_mask, dst_stage_mask) = match (old_layout, new_layout) {
        (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
            vk::AccessFlags::empty(),
            vk::AccessFlags::TRANSFER_WRITE,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER
        ),
        (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => (
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::SHADER_READ,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER
        ),
        _ => return Err(anyhow!("Unsupported image layout transition!"))
    };

    let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

    let subresource = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let barrier = vk::ImageMemoryBarrier::builder()
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(subresource)
        .src_access_mask(src_access_mask)
        .dst_access_mask(dst_access_mask)
        .build();

    unsafe {
        logical_device.cmd_pipeline_barrier(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            vk::DependencyFlags::empty(),
            &[] as &[vk::MemoryBarrier],
            &[] as &[vk::BufferMemoryBarrier],
            &[barrier]
        );
    }

    end_single_time_commands(logical_device, graphics_queue, command_pool, command_buffer)?;

    Ok(())
}

pub(crate) fn copy_buffer_to_image(
    logical_device: &ash::Device,
    command_pool: vk::CommandPool,
    graphics_queue: vk::Queue,
    buffer: vk::Buffer,
    image: vk::Image,
    width: u32,
    height: u32
) -> Result<()> {
    let command_buffer = begin_single_time_commands(logical_device, command_pool)?;

    let subresource = vk::ImageSubresourceLayers::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let region = vk::BufferImageCopy::builder()
        .buffer_offset(0)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(subresource)
        .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
        .image_extent(vk::Extent3D { width, height, depth: 1 })
        .build();

    unsafe {
        logical_device.cmd_copy_buffer_to_image(
            command_buffer,
            buffer,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region]
        );
    }

    end_single_time_commands(logical_device, graphics_queue, command_pool, command_buffer)?;

    Ok(())
}