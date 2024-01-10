use std::ffi::{c_void, CString};
use ash::{Entry, Instance, vk};
use crate::vulkan::constants::{API_VERSION, APPLICATION_VERSION, ENGINE_NAME, ENGINE_VERSION, VALIDATION_ENABLED, VALIDATION_LAYERS, WINDOW_TITLE};
use crate::vulkan::debug::create_debug_info;
use crate::vulkan::platform::required_extension_names;

pub fn create_instance(entry: &Entry, debug_info: vk::DebugUtilsMessengerCreateInfoEXT) -> anyhow::Result<Instance> {
    let app_name = CString::new(WINDOW_TITLE).unwrap();
    let engine_name = CString::new(ENGINE_NAME).unwrap();

    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: std::ptr::null(),
        application_version: APPLICATION_VERSION,
        p_engine_name: engine_name.as_ptr(),
        engine_version: ENGINE_VERSION,
        api_version: API_VERSION
    };


    let required_extension_names = required_extension_names();

    let required_validation_layer_raw_names: Vec<CString> = VALIDATION_LAYERS
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();

    let layer_names: Vec<*const i8>= required_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    let create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: if VALIDATION_ENABLED {
            &debug_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void
        } else {
            std::ptr::null()
        },
        flags: vk::InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        pp_enabled_layer_names: if VALIDATION_ENABLED {
            layer_names.as_ptr()
        } else {
            std::ptr::null()
        },
        enabled_layer_count: if VALIDATION_ENABLED {
            layer_names.len()
        } else {
            0
        } as u32,
        pp_enabled_extension_names: required_extension_names.as_ptr(),
        enabled_extension_count: required_extension_names.len() as u32
    };

    let instance: Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create Instance")
    };

    Ok(instance)
}