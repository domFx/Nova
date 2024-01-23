use crate::vulkan::queue_families::QueueFamilyIndices;

use ash::{Device,vk};
use ash::vk::{CommandPool};
use anyhow::Result;

pub unsafe fn create_command_pool(device: &Device, indices: &QueueFamilyIndices) -> Result<CommandPool> {
    let info = vk::CommandPoolCreateInfo::builder()
        // .flags(vk::CommandPoolCreateFlags::empty())
        .queue_family_index(indices.graphics)
        .build();

    let command_pool = device.create_command_pool(&info, None)?;

    Ok(command_pool)
}
