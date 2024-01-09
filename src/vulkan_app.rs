use crate::vulkan::command_buffers::*;
use crate::vulkan::command_pool::*;
use crate::vulkan::framebuffer::*;
use crate::vulkan::instance::*;
use crate::vulkan::logical_device::*;
use crate::vulkan::physical_device::*;
use crate::vulkan::pipeline::*;
use crate::vulkan::queue_families::*;
use crate::vulkan::render_pass::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::surface::*;

use anyhow::{Result};
use ash::{Entry, Instance, Device, vk};
use ash::extensions::khr::{Surface};
use ash::vk::{SurfaceKHR, PhysicalDevice, Queue, RenderPass, Framebuffer, CommandPool, CommandBuffer, Fence, Semaphore};
use winit::window::{Window};
use crate::vulkan::constants::MAX_FRAMES_IN_FLIGHT;

pub struct VulkanApp {
    entry: Entry,
    instance: Instance,
    surface_loader: Surface,
    logical_device: Device,
    physical_device: PhysicalDevice,
    graphics_queue: Queue,
    present_queue: Queue,
    surface: SurfaceKHR,
    swapchain: VulkanAppSwapchain,
    pipeline: VulkanAppPipeline,
    render_pass: RenderPass,
    framebuffers: Vec<Framebuffer>,
    command_pool: CommandPool,
    command_buffers: Vec<CommandBuffer>,
    image_available_semaphore: Vec<Semaphore>,
    render_finished_semaphore: Vec<Semaphore>,
    frame: usize,
    in_flight_fences: Vec<Fence>,
    images_in_flight: Vec<Fence>
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
        let present_queue = unsafe { logical_device.get_device_queue(indices.present, 0) };
        let swapchain = unsafe { VulkanAppSwapchain::get(&window, &instance, &logical_device, physical_device, &surface_loader, surface)? };
        let render_pass = unsafe { create_render_pass(&instance, &logical_device, swapchain.format)? };
        let pipeline = unsafe { create_pipeline(&logical_device, swapchain.extent, render_pass)? };
        let framebuffers = unsafe { create_framebuffers(&logical_device, &swapchain.image_views, render_pass, &swapchain.extent)? };
        let command_pool = unsafe { create_command_pool(&logical_device, &indices)? };
        let command_buffers = unsafe { create_command_buffers(&logical_device, command_pool, swapchain.extent, render_pass, &framebuffers, pipeline.pipeline)? };

        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let fence_info = vk::FenceCreateInfo::builder();

        let mut image_available_semaphore = Vec::new();
        let mut render_finished_semaphore = Vec::new();
        let mut in_flight_fences = Vec::new();

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            image_available_semaphore.push(unsafe{ logical_device.create_semaphore(&semaphore_info, None)? });
            render_finished_semaphore.push(unsafe{ logical_device.create_semaphore(&semaphore_info, None)? });
            in_flight_fences.push(unsafe { logical_device.create_fence(&fence_info, None)? });
        }

        let images_in_flight = swapchain.images.iter().map(|_| vk::Fence::null()).collect();

        Ok(Self {
            entry,
            instance,
            surface_loader,
            physical_device,
            logical_device,
            graphics_queue,
            present_queue,
            surface,
            swapchain,
            pipeline,
            render_pass,
            framebuffers,
            command_pool,
            command_buffers,
            image_available_semaphore,
            render_finished_semaphore,
            frame: 0,
            in_flight_fences,
            images_in_flight
        })
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        let in_flight_fence = self.in_flight_fences[self.frame];

        self.logical_device.wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

        let image_index = self
            .swapchain
            .loader
            .acquire_next_image(
                self.swapchain.swapchain,
                u64::MAX,
                self.image_available_semaphore[self.frame],
                Fence::null(),
            )?
            .0 as usize;

        let image_in_flight = self.images_in_flight[image_index];
        // if !image_in_flight.any() {
            self.logical_device.wait_for_fences(&[image_in_flight], true, u64::MAX)?;
        // }

        self.images_in_flight[image_index] = in_flight_fence;

        let wait_semaphores = &[self.image_available_semaphore[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.command_buffers[image_index]];
        let signal_semaphores = &[self.render_finished_semaphore[self.frame]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores)
            .build();

        self.logical_device.reset_fences(&[in_flight_fence])?;

        self.logical_device.queue_submit(self.graphics_queue, &[submit_info], in_flight_fence)?;

        let swapchains = &[self.swapchain.swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        self.swapchain.loader.queue_present(self.present_queue, &present_info)?;

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.logical_device.device_wait_idle().unwrap();

            self.in_flight_fences.iter().for_each(|f| self.logical_device.destroy_fence(*f, None));
            self.image_available_semaphore.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));
            self.render_finished_semaphore.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));
            self.logical_device.destroy_command_pool(self.command_pool, None);
            self.framebuffers.iter().for_each(|f| self.logical_device.destroy_framebuffer(*f, None));
            self.logical_device.destroy_pipeline(self.pipeline.pipeline, None);
            self.logical_device.destroy_pipeline_layout(self.pipeline.pipeline_layout, None);
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