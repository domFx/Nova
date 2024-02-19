use crate::renderer::vulkan::errors::*;

use anyhow::anyhow;
use ash::{Instance, vk};
use ash::extensions::khr::Surface;
use ash::vk::{PhysicalDevice, SurfaceKHR};

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32
}
impl QueueFamilyIndices {
    pub unsafe fn get(
        instance: &Instance,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_loader: &Surface
    ) -> anyhow::Result<Self> {
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
            Err(anyhow!(SuitabilityError("Missing required queue families")))
        }
    }
}