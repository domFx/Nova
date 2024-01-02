use crate::vulkan::constants::{DEVICE_EXTENSIONS, ENABLE_VALIDATION_LAYERS, PORTABILITY_MACOS_VERSION, VALIDATION_LAYERS};
use crate::vulkan::queue_families::QueueFamilyIndices;

use std::ffi::CString;

use ash::{Device, Entry, Instance, vk};
use ash::vk::{KhrPortabilitySubsetFn, PhysicalDevice};

pub unsafe fn create_logical_device(entry: &Entry, instance: &Instance, physical_device: PhysicalDevice, indices: &QueueFamilyIndices) -> anyhow::Result<Device> {
    let queue_priorities = &[1.0];
    let queue_info = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(indices.graphics)
        .queue_priorities(queue_priorities)
        .build();

    let mut extensions: Vec<*const i8> = DEVICE_EXTENSIONS
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    if cfg!(target_os = "macos") && entry.try_enumerate_instance_version()?.unwrap() >= PORTABILITY_MACOS_VERSION {
        let portability = KhrPortabilitySubsetFn::name();
        extensions.push(portability.as_ptr());
    }

    let layers: Vec<*const i8> = if ENABLE_VALIDATION_LAYERS {
        VALIDATION_LAYERS
            .iter()
            .map(|v| CString::new(*v).unwrap())
            .map(|v| v.as_ptr())
            .collect()
    } else  {
        vec![]
    };

    let features = vk::PhysicalDeviceFeatures::builder();

    let queue_infos = &[queue_info];
    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(physical_device, &info, None)?;

    Ok(device)
}

