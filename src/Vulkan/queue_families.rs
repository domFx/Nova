use crate::vulkan::errors::*;

use anyhow::anyhow;
use ash::{Instance, vk};
use ash::vk::PhysicalDevice;
#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32
}
impl QueueFamilyIndices {
    pub unsafe fn get(
        instance: &Instance,
        device: PhysicalDevice
    ) -> anyhow::Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let present = 0;

        if let Some(graphics) = graphics {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families")))
        }
    }
}