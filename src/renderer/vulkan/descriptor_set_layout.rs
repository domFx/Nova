use ash::{Device, vk};
use ash::vk::DescriptorSetLayout;

pub fn create_descriptor_set_layout(
    logical_device: &Device
) -> anyhow::Result<DescriptorSetLayout> {
    let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .build();

    let descriptor_set_layout_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(&[descriptor_set_layout_binding])
        .build();

    unsafe {
        let descriptor_set_layout = logical_device.create_descriptor_set_layout(&descriptor_set_layout_info, None)?;

        Ok(descriptor_set_layout)
    }
}