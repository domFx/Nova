use crate::constants::*;
use crate::platform::*;

use std::ops::Deref;
use std::{sync::Arc, ffi::CStr, ffi::CString};

use anyhow::{Result,anyhow};
use log::*;
use thiserror::Error;
use vulkano::device::DeviceCreateInfo;
// use winit::window::Window;
// use winit::platform::windows::WindowExtWindows;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::{physical::PhysicalDevice, QueueFlags, Device, QueueCreateInfo, Queue};
use vulkano::VulkanLibrary;

// const ENABLE_VALIDATION_LAYERS: bool = true;
// const VALIDATION_LAYERS: &'static [&'static str] = &["VK_LAYER_KHRONOS_validation"];
// const PORTABILITY_MACOS_VERSION: ash::Version = Version::new(1, 3, 216);

pub struct VulkanApp {
    instance: Arc<Instance>,
    library: Arc<VulkanLibrary>,
    physical_device: Arc<PhysicalDevice>,
    logical_device: Arc<Device>,
    queue: Arc<Queue>
}

impl VulkanApp {
    pub fn new() -> Result<Self> {
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(library.clone(), InstanceCreateInfo::default())?;
        let physical_device = instance
            .enumerate_physical_devices()?.next().unwrap();
        
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_index, properties)| {
                properties.queue_flags.contains(QueueFlags::GRAPHICS)
            })
            .unwrap() as u32;

        let (logical_device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            }
        ).unwrap();

        let queue = queues.next().unwrap();

        Ok(Self {
            instance,
            library,
            physical_device,
            logical_device,
            queue
        })
    }

    pub fn destroy(&self) {

    }
}