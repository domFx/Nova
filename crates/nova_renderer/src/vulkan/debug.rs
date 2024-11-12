use std::ffi::{c_void, CStr};
use anyhow::Result;
use ash::vk;
use ash::ext::debug_utils;
use log::{debug, error, info, warn};

pub(crate) struct VulkanAppDebug {
    pub debug_utils: debug_utils::Instance,
    pub utils_messenger: vk::DebugUtilsMessengerEXT
}

impl VulkanAppDebug {
    pub(crate) fn get(
        entry: &ash::Entry,
        instance: &ash::Instance,
        debug_info: vk::DebugUtilsMessengerCreateInfoEXT
    ) -> Result<Self> {
        let debug_utils = debug_utils::Instance::new(&entry, &instance);

        unsafe {
            let utils_messenger = debug_utils.create_debug_utils_messenger(&debug_info, None)?;

            Ok(Self{
                debug_utils,
                utils_messenger
            })
        }
    }
}

pub(crate) fn create_debug_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
    vk::DebugUtilsMessengerCreateInfoEXT::default()
        .flags(vk::DebugUtilsMessengerCreateFlagsEXT::empty())
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            // vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE ,
            // vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
        )
        .pfn_user_callback(Some(vulkan_debug_callback))
        .user_data(std::ptr::null_mut())
}

extern "system" fn vulkan_debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.p_message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        info!("({:?}) {}", type_, message);
    } else {
        debug!("({:?}) {}", type_, message);
    }

    vk::FALSE
}