use anyhow::anyhow;
use anyhow::Result;
use ash::{Device, Instance, vk};
use ash::vk::PhysicalDevice;

pub(crate) unsafe fn get_memory_type_index(
    instance: &Instance,
    physical_device: PhysicalDevice,
    properties: vk::MemoryPropertyFlags,
    requirements: vk::MemoryRequirements) -> Result<u32> {

    let memory = instance.get_physical_device_memory_properties(physical_device);
    (0..memory.memory_type_count)
        .find(|i| {
            let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];
            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find a suitable memory type."))
}

pub unsafe fn get_device_memory(
    instance: &Instance,
    logical_device: &Device,
    physical_device: PhysicalDevice,
    requirements: vk::MemoryRequirements) -> Result<vk::DeviceMemory> {

    let memory_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            physical_device,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            requirements
        )?)
        .build();

    let device_memory = logical_device.allocate_memory(&memory_info, None)?;

    Ok(device_memory)
}