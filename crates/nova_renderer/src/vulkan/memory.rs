use anyhow::{anyhow, Result};
use ash::vk;

pub(crate) fn get_memory_type_index(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    properties: vk::MemoryPropertyFlags,
    requirements: vk::MemoryRequirements) -> Result<u32> {

    unsafe {
        let memory = instance.get_physical_device_memory_properties(physical_device);
        (0..memory.memory_type_count)
            .find(|i| {
                let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
                let memory_type = memory.memory_types[*i as usize];
                suitable && memory_type.property_flags.contains(properties)
            })
            .ok_or_else(|| anyhow!("Failed to find a suitable memory type."))
    }
}

pub(crate) fn get_device_memory(
    instance: &ash::Instance,
    logical_device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    requirements: vk::MemoryRequirements) -> Result<vk::DeviceMemory> {

    let memory_info = vk::MemoryAllocateInfo::default()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            physical_device,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            requirements
        )?);

    unsafe {
        let device_memory = logical_device.allocate_memory(&memory_info, None)?;

        Ok(device_memory)
    }
}