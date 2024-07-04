use ash::vk::{DeviceSize, Extent3D, Format, Image, ImageCreateFlags, ImageLayout, ImageTiling, ImageType, ImageUsageFlags, MemoryPropertyFlags, PhysicalDevice, SampleCountFlags, SharingMode, StructureType};
use anyhow::Result;
use ash::{Device, Instance, vk};
use crate::vulkan::buffer::VulkanAppBuffer;
use crate::vulkan::memory::get_memory_type_index;

fn create_texture_image(
    instance: &Instance,
    logical_device: &Device,
    physical_device: PhysicalDevice
) -> Result<()> {
    let image = image::io::Reader::open("./src/images/texture.jpg")?.decode()?;

    let device_size: DeviceSize = (image.height() * image.width() * 4) as DeviceSize;

    let buffer = VulkanAppBuffer::new(
        instance,
        logical_device,
        physical_device,
        device_size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
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
        .array_layers(1)
        .mip_levels(1)
        .extent(Extent3D { width: image.width(), height: image.height(), depth: 1 })
        .format(Format::R8G8B8A8_SRGB)
        .tiling(ImageTiling::OPTIMAL)
        .initial_layout(ImageLayout::UNDEFINED)
        .usage(ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED)
        .sharing_mode(SharingMode::EXCLUSIVE)
        .samples(SampleCountFlags::TYPE_1)
        .flags(ImageCreateFlags::empty())
        .build();

    unsafe {
        let image = logical_device.create_image(&image_create_info, None)?;
        let memory_requirements = logical_device.get_image_memory_requirements(image);

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(get_memory_type_index(
                instance,
                physical_device,
                MemoryPropertyFlags::DEVICE_LOCAL,
                memory_requirements
            )?)
            .build();

        logical_device.allocate_memory(&alloc_info, None)?;

        logical_device.bind_image_memory(image, buffer.buffer_memory, device_size)?;
    }

    Ok(())
}

