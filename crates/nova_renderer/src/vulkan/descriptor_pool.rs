use anyhow::Result;
use ash::vk;

pub(crate) fn create_descriptor_pool(
    logical_device: &ash::Device,
    swapchain_images: &Vec<vk::Image>
) -> Result<vk::DescriptorPool> {
    let uniform_buffer_size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(swapchain_images.len() as u32);

    let sampler_size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(swapchain_images.len() as u32);

    let pool_sizes = &[uniform_buffer_size, sampler_size];
    let descriptor_pool_info = vk::DescriptorPoolCreateInfo::default()
        .pool_sizes(pool_sizes)
        .max_sets(swapchain_images.len() as u32);

    unsafe {
        let descriptor_pool = logical_device.create_descriptor_pool(&descriptor_pool_info, None)?;
        Ok(descriptor_pool)
    }
}