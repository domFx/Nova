use crate::vulkan::instance::*;
use crate::vulkan::logical_device::*;
use crate::vulkan::physical_device::*;
use crate::vulkan::queue_families::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::surface::*;
use crate::vulkan::pipeline::*;

use anyhow::{Result};
use ash::{Entry, Instance, Device};
use ash::extensions::khr::{ Surface };
use ash::vk::{SurfaceKHR, PhysicalDevice, Queue, PipelineLayout, RenderPass};
use winit::window::{Window};
use crate::vulkan::render_pass::create_render_pass;

pub struct VulkanApp {
    entry: Entry,
    instance: Instance,
    surface_loader: Surface,
    logical_device: Device,
    physical_device: PhysicalDevice,
    graphics_queue: Queue,
    surface: SurfaceKHR,
    swapchain: VulkanAppSwapchain,
    pipeline_layout: PipelineLayout,
    render_pass: RenderPass
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
        let render_pass = unsafe { create_render_pass(&instance, &logical_device, swapchain.format)? };
        let pipeline_layout = unsafe { create_pipeline(&logical_device, swapchain.extent)? };

        Ok(Self {
            entry,
            instance,
            surface_loader,
            physical_device,
            logical_device,
            graphics_queue,
            surface,
            swapchain,
            pipeline_layout,
            render_pass
        })
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.logical_device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.logical_device.destroy_render_pass(self.render_pass, None);
            self.swapchain.image_views
                .iter()
                .for_each(|v| self.logical_device.destroy_image_view(*v, None));
            self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);
            self.logical_device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}