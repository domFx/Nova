use crate::vulkan::constants::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYERS};
use crate::vulkan::queue_families::QueueFamilyIndices;

use ash::vk;
use ash::vk::KHR_PORTABILITY_SUBSET_NAME;

pub(crate) fn create_logical_device(
    entry: &ash::Entry,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    indices: &QueueFamilyIndices
) -> anyhow::Result<ash::Device> {
    let mut unique_indices = std::collections::HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|i| {
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();

    let mut extensions: Vec<*const i8> = DEVICE_EXTENSIONS
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    unsafe {
        if cfg!(target_os = "macos") && entry.try_enumerate_instance_version()?.unwrap() >= PORTABILITY_MACOS_VERSION {
            let portability = KHR_PORTABILITY_SUBSET_NAME;
            extensions.push(portability.as_ptr());
        }
    }

    let layers: Vec<*const i8> = if VALIDATION_ENABLED {
        VALIDATION_LAYERS
            .iter()
            .map(|v| v.as_ptr())
            .collect()
    } else  {
        vec![]
    };

    let features = vk::PhysicalDeviceFeatures::default()
        .sampler_anisotropy(true);

    let info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    unsafe {
        let device = instance.create_device(physical_device, &info, None)?;

        Ok(device)
    }
}