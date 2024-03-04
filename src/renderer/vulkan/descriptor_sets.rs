use std::mem::size_of;
use anyhow::Result;
use ash::{Device, vk};
use ash::vk::{DescriptorPool, DescriptorSet, DescriptorSetLayout, Image};
use crate::renderer::vulkan::buffer::VulkanAppBuffer;
use crate::renderer::vulkan::uniform_buffers::UniformBufferObject;

pub fn create_descriptor_sets(
    logical_device: &Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_layout: DescriptorSetLayout,
    swapchain_images: &Vec<Image>,
    uniform_buffers: &Vec<VulkanAppBuffer>
) -> Result<Vec<DescriptorSet>> {
    let set_layouts = vec![descriptor_set_layout; swapchain_images.len()];
    let descriptor_set_allocator_info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(descriptor_pool)
        .set_layouts(&set_layouts)
        .build();

    unsafe {
        let descriptor_sets = logical_device.allocate_descriptor_sets(&descriptor_set_allocator_info)?;

        for i in 0..swapchain_images.len() {
            let info = vk::DescriptorBufferInfo::builder()
                .buffer(uniform_buffers[i].buffer)
                .offset(0)
                .range(size_of::<UniformBufferObject>() as u64)
                .build();

            let ubo_write = vk::WriteDescriptorSet::builder()
                .dst_set(descriptor_sets[i])
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&[info])
                .build();

            logical_device.update_descriptor_sets(&[ubo_write], &[] as &[vk::CopyDescriptorSet]);
        }

        Ok(descriptor_sets)
    }
}