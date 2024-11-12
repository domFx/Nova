use crate::vulkan::queue_families::QueueFamilyIndices;

use ash::vk;
use anyhow::Result;

pub(crate) fn create_command_pool(
    device: &ash::Device,
    indices: &QueueFamilyIndices
) -> Result<vk::CommandPool> {
    let info = vk::CommandPoolCreateInfo::default()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(indices.graphics);

    unsafe {
        let command_pool = device.create_command_pool(&info, None)?;

        Ok(command_pool)
    }
}
