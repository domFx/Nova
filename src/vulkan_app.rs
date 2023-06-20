use std::ffi::CStr;
use std::os::raw::c_char;

use ash::{vk, Entry};
use ash_window;
use ash::extensions::{
    ext::DebugUtils
};
#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{
    KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn, KhrPortabilitySubsetFn,
};

use raw_window_handle::{HasRawDisplayHandle};
use winit::window::Window;

struct VulkanApp {
    entry: Entry, 
    instance: vk::Instance
}

pub unsafe fn create(window: Window) {
    let entry = Entry::load().expect("Unable to load Entry");
    
    let app_name = CStr::from_bytes_with_nul_unchecked(b"Game\0");
    let engine_name = CStr::from_bytes_with_nul_unchecked(b"Nova Engine\0");

    let layer_names = [CStr::from_bytes_with_nul_unchecked(
        b"VK_LAYER_KHRONOS_validation\0",
    )];

    let layer_names_raw: Vec<*const c_char>= layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();

    let mut extension_names =
        ash_window::enumerate_required_extensions(window.raw_display_handle())
            .unwrap()
            .to_vec();

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        extension_names.push(vk::KhrPortabilityEnumerationFn::name().as_ptr());
        extension_names.push(vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
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
        .enabled_layer_names(&layer_names_raw)
        .enabled_extension_names(&extension_names)
        .flags(create_flags)
        .build();
    
    let instance = entry
        .create_instance(&create_info, None)
        .expect("Unable to create instance");

}