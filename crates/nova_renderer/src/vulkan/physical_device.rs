use crate::vulkan::constants::DEVICE_EXTENSIONS;
use crate::vulkan::errors::VulkanSuitabilityError;
use crate::vulkan::queue_families::QueueFamilyIndices;
use crate::vulkan::swapchain::SwapchainSupport;

use nova_core::utils::convert_vec_to_string;

use std::collections::HashSet;
use std::ffi::CStr;

use anyhow::{Result, anyhow};
use ash::{khr,vk};
use log::{info, warn};

pub(crate) fn pick_physical_device(
    instance: &ash::Instance,
    surface_loader: &khr::surface::Instance,
    surface: vk::SurfaceKHR
) -> Result<vk::PhysicalDevice> {
    unsafe {
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
    }

    Err(anyhow!(VulkanSuitabilityError("Missing valid physical device.")))
}

fn check_physical_device(
    instance: &ash::Instance,
    surface_loader: &khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice
) -> Result<()> {
    unsafe {
        QueueFamilyIndices::get(instance, physical_device, surface, &surface_loader)?;
        check_physical_device_extensions(instance, physical_device)?;

        let support = SwapchainSupport::get(surface_loader, physical_device, surface)?;

        if support.formats.is_empty() || support.present_modes.is_empty() {
            return Err(anyhow!(VulkanSuitabilityError("Insufficient swapchain support.")))
        }

        let features = instance.get_physical_device_features(physical_device);
        if features.sampler_anisotropy != vk::TRUE {
            return Err(anyhow!(VulkanSuitabilityError("No sampler anisotropy.")))
        }

        Ok(())
    }
}

fn check_physical_device_extensions(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice
) -> Result<()> {
    unsafe {
        let extensions = instance
            .enumerate_device_extension_properties(physical_device)?
            .iter()
            .map(|e| CStr::from_ptr(e.extension_name.as_ptr()) )
            .collect::<HashSet<_>>();

        if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
            Ok(())
        } else {
            Err(anyhow!(VulkanSuitabilityError("Missing required device extensions")))
        }
    }
}