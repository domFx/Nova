use crate::constants::*;
use crate::platform::*;

use std::{ffi::CStr, ffi::CString};

use ash::{vk, Entry};
use anyhow::{Result,anyhow};
use thiserror::Error;
use log::*;

const ENABLE_VALIDATION_LAYERS: bool = true;
const VALIDATION_LAYERS: &'static [&'static str] = &["VK_LAYER_KHRONOS_validation"];

pub struct VulkanApp {
    entry: Entry, 
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    logical_device: ash::Device,
    graphics_queue: vk::Queue
}

impl VulkanApp {
    pub fn new() -> Result<Self> {
        let entry = unsafe { Entry::load().expect("Unable to load Entry") };
        let instance = create_instance(&entry)?;
        let physical_device = unsafe { pick_physical_device(&instance)? };
        let indices = unsafe { QueueFamilyIndices::get(&instance, physical_device)? };
        let logical_device = unsafe { create_logical_device(&entry, &instance, physical_device, &indices)? };
        let graphics_queue = unsafe { logical_device.get_device_queue(indices.graphics, 0) };

        Ok(Self {
            entry,
            instance,
            physical_device,
            logical_device,
            graphics_queue
        })
    }

    pub fn destroy(&mut self) {
        unsafe { 
            self.logical_device.destroy_device(None);

            self.instance.destroy_instance(None);
        }
    }
}

unsafe fn pick_physical_device(instance: &ash::Instance) -> Result<vk::PhysicalDevice> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, physical_device) {
            warn!("Skipping physical device (`{:?}`): {}", properties.device_name, error);
        } else {
            info!("Selected physical device (`{:?}`).", properties.device_name);
            return Ok(physical_device)
        }
    }

    Err(anyhow!(SuitablilityError("Missing valid physical device.")))
}

unsafe fn check_physical_device(instance: &ash::Instance, device: vk::PhysicalDevice) -> Result<()> {
    QueueFamilyIndices::get(instance, device)?;
    return Ok(())
}

#[derive(Debug, Error)]
#[error("{0}.")]
pub struct SuitablilityError(pub &'static str);

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
}

impl QueueFamilyIndices {
    unsafe fn get(
        instance: &ash::Instance,
        device: vk::PhysicalDevice
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(device);
        
        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        if let Some(graphics) = graphics {
            Ok(Self { graphics })
        } else {
            Err(anyhow!(SuitablilityError("Missing required queue families")))
        }
    }
}

unsafe fn create_logical_device(entry: &Entry, instance: &ash::Instance, physical_device: vk::PhysicalDevice, indices: &QueueFamilyIndices) -> Result<ash::Device> {
    let queue_priorities = &[1.0];
    let queue_info = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(indices.graphics)
        .queue_priorities(queue_priorities)
        .build();

    let mut extensions: Vec<*const i8> = vec![];
    // if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
    //     extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    // }


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

fn create_instance(entry: &Entry) -> Result<ash::Instance> {
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

    Ok(instance)
}

fn create_debug_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: std::ptr::null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: 
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            // vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        message_type:
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL | 
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        // vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING |
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