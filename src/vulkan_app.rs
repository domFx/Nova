use crate::vulkan::instance::*;
use crate::vulkan::logical_device::*;
use crate::vulkan::physical_device::*;
use crate::vulkan::queue_families::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::surface::*;

use anyhow::{Result};
use ash::{Entry, Instance, Device};
use ash::extensions::khr::{ Surface };
use ash::vk::{SurfaceKHR, PhysicalDevice, Queue};
use winit::window::{Window};

pub struct VulkanApp {
    entry: Entry, 
    instance: Instance,
    surface_loader: Surface,
    logical_device: Device,
    physical_device: PhysicalDevice,
    graphics_queue: Queue,
    surface: SurfaceKHR,
    swapchain: VulkanAppSwapchain,
}

impl VulkanApp {
    pub fn new(window: &Window) -> Result<Self> {
        let entry = unsafe { Entry::load()? };
        let instance = create_instance(&entry)?;
        let surface_loader = Surface::new(&entry, &instance);
        let surface = unsafe { create_surface(window, &entry, &instance)? };
        let physical_device = unsafe { pick_physical_device(&instance, &surface_loader, surface )? };
        let indices = unsafe { QueueFamilyIndices::get(&instance, physical_device)? };
        let logical_device = unsafe { create_logical_device(&entry, &instance, physical_device, &indices)? };
        let graphics_queue = unsafe { logical_device.get_device_queue(indices.graphics, 0) };

        let swapchain = unsafe { VulkanAppSwapchain::get(&window, &instance, &logical_device, physical_device, &surface_loader, surface)? };

        Ok(Self {
            entry,
            instance,
            surface_loader,
            physical_device,
            logical_device,
            graphics_queue,
            surface,
            swapchain,
        })
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.swapchain.image_views
                .iter()
                .for_each(|v| self.logical_device.destroy_image_view(*v, None));

            self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);

            self.logical_device.destroy_device(None);

            // self.surface.destroy_surface_khr --> probably a cleaner way to do this
            self.surface_loader.destroy_surface(self.surface, None);

            self.instance.destroy_instance(None);
        }
    }
}