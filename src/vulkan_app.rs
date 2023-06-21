use std::ffi::CString;
use std::{ffi::CStr, collections::HashSet};
use std::os::raw::c_char;

use ash::extensions::ext::DebugUtils;
use ash::{vk, Entry};
use ash_window;

use raw_window_handle::{HasRawDisplayHandle};
use winit::window::Window;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

struct VulkanApp {
    entry: Entry, 
    instance: vk::Instance
}

pub unsafe fn create(window: &Window) {
    let entry = Entry::load().expect("Unable to load Entry");
    
    let app_name = CStr::from_bytes_with_nul_unchecked(b"Game\0");
    let engine_name = CStr::from_bytes_with_nul_unchecked(b"Nova Engine\0");

    let available_layers = entry
        .enumerate_instance_layer_properties()
        .expect("Unable to enumerate instance layer properties")
        .iter()
        .map(|l| {
            let layer_name = std::ffi::CStr::from_ptr(l.layer_name.as_ptr());
            layer_name.to_str().unwrap()
        })
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains("VK_LAYER_KHRONOS_validation") {
        println!("Validation layer requested but not supported");
        return;
    }

    let layers: Vec<*const c_char> = if VALIDATION_ENABLED {
        vec![CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0")]
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect()
    } else {
        Vec::new()
    };

    let mut extension_names =
        ash_window::enumerate_required_extensions(window.raw_display_handle())
            .unwrap()
            .to_vec();

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        extension_names.push(vk::KhrPortabilityEnumerationFn::name().as_ptr());
        extension_names.push(vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
    }

    if VALIDATION_ENABLED {
        extension_names.push(ash::extensions::ext::DebugUtils::name().as_ptr());
    }

    let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    let app_info = vk::ApplicationInfo::builder()
        .application_name(app_name)
        .application_version(0)
        .engine_name(engine_name)
        .engine_version(1)
        .api_version(vk::make_api_version(0, 1, 0, 0))
        .build();

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extension_names)
        .flags(create_flags)
        .build();

    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING |
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL | 
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        )
        .pfn_user_callback(Some(debug_callback))
        .build();

    let instance = entry
        .create_instance(&create_info, None)
        .expect("Unable to create instance");
    
    if VALIDATION_ENABLED {
        let debug_utils_loader = DebugUtils::new(&entry, &instance);
        debug_utils_loader
            .create_debug_utils_messenger(&debug_info, None)
            .unwrap(); 
    }
}

extern "system" fn debug_callback(
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