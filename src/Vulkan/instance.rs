use crate::vulkan::constants::{VALIDATION_ENABLED, VALIDATION_LAYERS, WINDOW_TITLE};
use crate::vulkan::platform::required_extension_names;

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use anyhow::anyhow;
use anyhow::Result;
use ash::{Entry, Instance, vk};

pub unsafe fn create_instance(entry: &Entry, debug_info: &mut vk::DebugUtilsMessengerCreateInfoEXT) -> Result<Instance> {
    let app_info = vk::ApplicationInfo::builder()
        .application_name(CString::new(WINDOW_TITLE).unwrap().as_c_str())
        .application_version(vk::make_api_version(1, 0, 0, 0))
        .engine_name(CString::new(WINDOW_TITLE).unwrap().as_c_str())
        .api_version(vk::make_api_version(1, 0, 0, 0))
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

    let mut instance_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(vk::InstanceCreateFlags::empty());

    if VALIDATION_ENABLED {
        instance_info = instance_info.push_next(debug_info);
    }

    let instance: Instance = entry
            .create_instance(&instance_info, None)
            .expect("Failed to create Instance");

    Ok(instance)
}