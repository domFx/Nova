use crate::vulkan::errors::*;

use anyhow::anyhow;
use ash::vk;
use ash::extensions::khr::Surface;

#[derive(Copy, Clone, Debug)]
pub(crate) struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32
}
impl QueueFamilyIndices {
    pub(crate) fn get(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_loader: &Surface
    ) -> anyhow::Result<Self> {
        unsafe {
            let properties = instance.get_physical_device_queue_family_properties(physical_device);

            let graphics = properties
                .iter()
                .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
                .map(|i| i as u32);

            let mut present = None;
            for (index, _properties) in properties.iter().enumerate() {
                if surface_loader.get_physical_device_surface_support(physical_device, index as u32, surface)? {
                    present = Some(index as u32);
                    break;
                }
            }

            if let (Some(graphics), Some(present)) = (graphics, present) {
                Ok(Self { graphics, present })
            } else {
                Err(anyhow!(VulkanSuitabilityError("Missing required queue families")))
            }
        }
    }
}