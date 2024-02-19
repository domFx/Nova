use crate::renderer::vulkan::constants::{API_VERSION, APPLICATION_VERSION, ENGINE_NAME, ENGINE_VERSION, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYERS, WINDOW_TITLE};
use crate::renderer::vulkan::platform::required_extension_names;

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use anyhow::anyhow;
use anyhow::Result;
use ash::{Entry, Instance, vk};
use log::info;

pub unsafe fn create_instance(entry: &Entry, debug_info: &mut vk::DebugUtilsMessengerCreateInfoEXT) -> Result<Instance> {
    let app_info = vk::ApplicationInfo::builder()
        .application_name(CString::new(WINDOW_TITLE).unwrap().as_c_str())
        .application_version(APPLICATION_VERSION)
        .engine_name(CString::new(ENGINE_NAME).unwrap().as_c_str())
        .api_version(API_VERSION)
        .engine_version(ENGINE_VERSION)
        .build();

    let mut extensions = required_extension_names();

    if VALIDATION_ENABLED {
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

        extensions.push(vk::ExtDebugUtilsFn::name().as_ptr())
    }

    let layers = if VALIDATION_ENABLED {
        VALIDATION_LAYERS
            .iter()
            .map(|v| v.as_ptr())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let app_version = entry.try_enumerate_instance_version()?.unwrap();

    // Required by Vulkan SDK on macOS since 1.3.216.
    let flags = if cfg!(target_os = "macos") && app_version >= PORTABILITY_MACOS_VERSION {
        info!("Enabling extensions for macOS portability.");
        extensions.push(vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        extensions.push(vk::KhrPortabilityEnumerationFn::name().as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        extensions.push(vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        extensions.push(vk::KhrPortabilityEnumerationFn::name().as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        // vk::InstanceCreateFlags::empty()
    };

    let mut instance_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);

    if VALIDATION_ENABLED {
        instance_info = instance_info.push_next(debug_info);
    }

    let instance: Instance = entry
        .create_instance(&instance_info, None)?;

    Ok(instance)
}