use crate::constants::*;
use crate::platform::*;

use std::{ffi::CStr, ffi::CString};

use ash::{vk, Entry};

pub struct VulkanApp {
    entry: Entry, 
    instance: ash::Instance
}

impl VulkanApp {
    pub fn new() -> VulkanApp {
        let entry = unsafe { Entry::load().expect("Unable to load Entry") };
        let instance = create_instance(&entry);

        VulkanApp {
            entry,
            instance
        }
    }

    pub fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None); }
    }
}

fn create_instance(entry: &Entry) -> ash::Instance {
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

    let debug_info = create_debug_info();

    let required_extension_names = required_extension_names();

    let required_validation_layer_raw_names: Vec<CString> = ["VK_LAYER_KHRONOS_validation"]
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
            &debug_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const std::ffi::c_void
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

    let instance: ash::Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create Instance")
    };

    instance
}

fn create_debug_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: std::ptr::null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: 
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        message_type:
            vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING |
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL | 
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(vulkan_debug_callback),
        p_user_data: std::ptr::null_mut()
    }
}

extern "system" fn vulkan_debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.p_message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        println!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        println!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        println!("({:?}) {}", type_, message);
    } else {
        println!("({:?}) {}", type_, message);
    }

    vk::FALSE
}