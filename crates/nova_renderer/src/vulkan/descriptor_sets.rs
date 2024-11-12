use crate::vulkan::vulkan_buffer::VulkanAppBuffer;
use crate::vulkan::uniform_buffers::UniformBufferObject;
use crate::vulkan::constants::MAX_FRAMES_IN_FLIGHT;

use anyhow::Result;
use ash::vk;
use std::mem::size_of;

pub(crate) fn create_descriptor_sets(
    logical_device: &ash::Device,
    descriptor_pool: vk::DescriptorPool,
    descriptor_set_layout: vk::DescriptorSetLayout,
    swapchain_images: &Vec<vk::Image>,
    uniform_buffers: &Vec<VulkanAppBuffer>,
    texture_image_view: vk::ImageView,
    sampler: vk::Sampler
) -> Result<Vec<vk::DescriptorSet>> {
    let set_layouts = vec![descriptor_set_layout; swapchain_images.len()];
    let descriptor_set_allocator_info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(descriptor_pool)
        .set_layouts(&set_layouts);

    unsafe {
        let descriptor_sets = logical_device.allocate_descriptor_sets(&descriptor_set_allocator_info)?;

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            let uniform_buffer_info = &[vk::DescriptorBufferInfo::default()
                .buffer(uniform_buffers[i].buffer)
                .offset(0)
                .range(size_of::<UniformBufferObject>() as u64)];

            let image_info = &[vk::DescriptorImageInfo::default()
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image_view(texture_image_view)
                .sampler(sampler)];

            let uniform_buffer_write = vk::WriteDescriptorSet::default()
                .dst_set(descriptor_sets[i])
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(uniform_buffer_info);

            let sampler_write = vk::WriteDescriptorSet::default()
                .dst_set(descriptor_sets[i])
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(image_info);

            logical_device.update_descriptor_sets(&[uniform_buffer_write, sampler_write], &[] as &[vk::CopyDescriptorSet]);
        }

        Ok(descriptor_sets)
    }
}