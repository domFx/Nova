use anyhow::Result;
use ash::{Device, vk};
use ash::vk::{DescriptorPool, Image};

pub fn create_descriptor_pool(
    logical_device: &Device,
    swapchain_images: &Vec<Image>
) -> Result<DescriptorPool> {
    let ubo_size = vk::DescriptorPoolSize::builder()
        .ty(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(swapchain_images.len() as u32)
        .build();

    let descriptor_pool_info = vk::DescriptorPoolCreateInfo::builder()
        .pool_sizes(&[ubo_size])
        .max_sets(swapchain_images.len() as u32)
        .build();

    unsafe {
        let descriptor_pool = logical_device.create_descriptor_pool(&descriptor_pool_info, None)?;
        Ok(descriptor_pool)
    }
}