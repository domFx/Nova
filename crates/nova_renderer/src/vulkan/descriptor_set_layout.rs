use ash::vk;

pub(crate) fn create_descriptor_set_layout(
    logical_device: &ash::Device
) -> anyhow::Result<vk::DescriptorSetLayout> {
    let uniform_buffer_set_layout_binding = vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX);

    let sampler_set_layout_binding = vk::DescriptorSetLayoutBinding::default()
        .binding(1)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT);

    let bindings = &[uniform_buffer_set_layout_binding, sampler_set_layout_binding];
    let descriptor_set_layout_info = vk::DescriptorSetLayoutCreateInfo::default()
        .bindings(bindings);

    unsafe {
        let descriptor_set_layout = logical_device.create_descriptor_set_layout(&descriptor_set_layout_info, None)?;

        Ok(descriptor_set_layout)
    }
}