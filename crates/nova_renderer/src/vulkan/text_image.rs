use ash::vk::{DeviceSize, PhysicalDevice};
use anyhow::Result;
use ash::{Device, Instance, vk};
use crate::vulkan::buffer::VulkanAppBuffer;

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

    Ok(())
}