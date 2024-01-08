use crate::utils::convert_vec_to_string;
use crate::vulkan::constants::DEVICE_EXTENSIONS;
use crate::vulkan::errors::SuitabilityError;
use crate::vulkan::queue_families::QueueFamilyIndices;
use crate::vulkan::swapchain::SwapchainSupport;

use std::collections::HashSet;
use std::ffi::CStr;

use anyhow::{Result, anyhow};
use ash::extensions::khr::Surface;
use ash::{Instance};
use ash::vk::{PhysicalDevice, SurfaceKHR};
use log::{info, warn};

pub unsafe fn pick_physical_device(instance: &Instance, surface_loader: &Surface, surface: SurfaceKHR) -> Result<PhysicalDevice> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        let device_name = convert_vec_to_string(&properties.device_name.to_vec());

        if let Err(error) = check_physical_device(instance, surface_loader, surface, physical_device) {
            warn!("Skipping physical device (`{:?}`): {}", device_name, error);
        } else {
            info!("Selected physical device (`{:?}`).", device_name);
            return Ok(physical_device)
        }
    }

    Err(anyhow!(SuitabilityError("Missing valid physical device.")))
}

unsafe fn check_physical_device(instance: &Instance, surface_loader: &Surface, surface: SurfaceKHR, physical_device: PhysicalDevice) -> Result<()> {
    QueueFamilyIndices::get(instance, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(surface_loader, physical_device, surface)?;

    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")))
    }

    return Ok(())
}

unsafe fn check_physical_device_extensions(instance: &Instance, physical_device: PhysicalDevice) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device)?
        .iter()
        .map(|e| unsafe { CStr::from_ptr(e.extension_name.as_ptr()) })
        .collect::<HashSet<_>>();

    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError("Missing required device extensions")))
    }
}