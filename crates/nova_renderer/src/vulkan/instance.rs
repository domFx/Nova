use crate::vulkan::constants::{API_VERSION, APPLICATION_VERSION, ENGINE_NAME, ENGINE_VERSION, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYERS, WINDOW_TITLE};
use crate::vulkan::platform::required_extension_names;

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use anyhow::{anyhow, Result};
use ash::vk;
use log::info;

pub(crate) fn create_instance(
    entry: &ash::Entry,
    debug_info: &mut vk::DebugUtilsMessengerCreateInfoEXT
) -> Result<ash::Instance> {
    let window_title = CString::new(WINDOW_TITLE)?;
    let engine_name = CString::new(ENGINE_NAME)?;

    let app_info = vk::ApplicationInfo::default()
        .application_name(&window_title)
        .application_version(APPLICATION_VERSION)
        .engine_name(&engine_name)
        .api_version(API_VERSION)
        .engine_version(ENGINE_VERSION);

    let mut extensions = required_extension_names();

    if VALIDATION_ENABLED {
        unsafe {
            let available_layers = entry
                .enumerate_instance_layer_properties()?
                .iter()
                .map(|l| CStr::from_ptr(l.layer_name.as_ptr()))
                .collect::<HashSet<_>>();

            let mut valid_layers = true;
            VALIDATION_LAYERS
                .iter()
                .for_each(|l| {
                    if !available_layers.contains(l) {
                        valid_layers = false;
                    }
                });

            if !valid_layers {
                return Err(anyhow!("Validation layer requested but not supported."));
            }

            extensions.push(ash::ext::debug_utils::NAME.as_ptr())
        }
    }

    let layers = if VALIDATION_ENABLED {
        VALIDATION_LAYERS
            .iter()
            .map(|v| v.as_ptr())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    unsafe {
        let app_version = entry.try_enumerate_instance_version()?.unwrap();

        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_NAME.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_NAME.as_ptr());
        // Required by Vulkan SDK on macOS since 1.3.216.
        let flags = vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;

        if cfg!(target_os = "macos") && app_version >= PORTABILITY_MACOS_VERSION {
            info!("Enabling extensions for macOS portability.");
            extensions.push(vk::EXT_METAL_SURFACE_NAME.as_ptr());
            extensions.push(vk::MVK_MACOS_SURFACE_NAME.as_ptr());
        };

        let mut instance_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .flags(flags);

        if VALIDATION_ENABLED {
            instance_info = instance_info.push_next(debug_info);
        }

        let instance: ash::Instance = entry
            .create_instance(&instance_info, None)?;

        Ok(instance)
    }
}