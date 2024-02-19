use crate::vulkan::command_buffers::*;
use crate::vulkan::command_pool::*;
use crate::vulkan::descriptor_set_layout::*;
use crate::vulkan::framebuffer::*;
use crate::vulkan::instance::*;
use crate::vulkan::logical_device::*;
use crate::vulkan::physical_device::*;
use crate::vulkan::pipeline::*;
use crate::vulkan::queue_families::*;
use crate::vulkan::render_pass::*;
use crate::vulkan::swapchain::*;
use crate::vulkan::surface::*;
use crate::vulkan::vertex::*;
use crate::vulkan::index_buffer::*;

use crate::vulkan::buffer::VulkanAppBuffer;
use crate::vulkan::constants::{MAX_FRAMES_IN_FLIGHT, VALIDATION_ENABLED};
use crate::vulkan::debug::{create_debug_info, VulkanAppDebug};
use crate::vulkan::uniform_buffers::{create_uniform_buffers, update_uniform_buffer};
use crate::vulkan::vertex::create_vertex_buffer;

use anyhow::*;
use anyhow::{Result};
use ash::{Entry, Instance, Device, vk};
use ash::extensions::khr::{Surface};
use ash::prelude::VkResult;
use ash::vk::{SurfaceKHR, PhysicalDevice, Queue, RenderPass, Framebuffer, CommandPool, CommandBuffer, Fence, Semaphore, Image, DescriptorSetLayout, DescriptorPool, DescriptorSet};
use std::time::Instant;
use cgmath::Vector2;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use winit::window::{Window};
use crate::core::window::NovaWindow;
use crate::vulkan::descriptor_pool::create_descriptor_pool;
use crate::vulkan::descriptor_sets::create_descriptor_sets;

pub struct VulkanApp {
    entry: Entry,
    instance: Instance,
    surface_loader: Surface,
    logical_device: Device,
    debug: VulkanAppDebug,
    physical_device: PhysicalDevice,
    graphics_queue: Queue,
    present_queue: Queue,
    surface: SurfaceKHR,
    swapchain: VulkanAppSwapchain,
    pipeline: VulkanAppPipeline,
    descriptor_set_layout: DescriptorSetLayout,
    descriptor_pool: DescriptorPool,
    descriptor_sets: Vec<DescriptorSet>,
    render_pass: RenderPass,
    framebuffers: Vec<Framebuffer>,
    command_pool: CommandPool,
    command_buffers: Vec<CommandBuffer>,
    image_available_semaphores: Vec<Semaphore>,
    render_finished_semaphores: Vec<Semaphore>,
    frame: usize,
    in_flight_fences: Vec<Fence>,
    images_in_flight: Vec<Fence>,
    indices: QueueFamilyIndices,
    images: Vec<Image>,
    image_views: Vec<vk::ImageView>,
    vertex_buffer: VulkanAppBuffer,
    index_buffer: VulkanAppBuffer,
    uniform_buffers: Vec<VulkanAppBuffer>,
    start: Instant,
    pub resized: bool
}

impl VulkanApp {
    pub fn new(raw_window_handle: RawWindowHandle, raw_display_handle: RawDisplayHandle, window_dimensions: Vector2<f32>) -> Result<Self> {
        unsafe {
            let entry = Entry::load()?;
            let mut debug_info = create_debug_info();
            let instance
                = create_instance(&entry, &mut debug_info)?;
            let debug = VulkanAppDebug::get(&entry, &instance, debug_info)?;
            let surface_loader = Surface::new(&entry, &instance);
            let surface = create_surface(raw_window_handle, raw_display_handle, &entry, &instance)?;
            let physical_device = pick_physical_device(&instance, &surface_loader, surface)?;
            let indices = QueueFamilyIndices::get(&instance, physical_device, surface, &surface_loader)?;

            let logical_device = create_logical_device(&entry, &instance, physical_device, &indices)?;
            let graphics_queue = logical_device.get_device_queue(indices.graphics, 0);
            let present_queue = logical_device.get_device_queue(indices.present, 0);
            let swapchain = VulkanAppSwapchain::get(window_dimensions, &instance, &logical_device, physical_device, &surface_loader, surface, &indices)?;
            let images = swapchain.loader.get_swapchain_images(swapchain.swapchain)?;
            let image_views =  create_swapchain_image_views(&logical_device, &images, swapchain.format)?;
            let render_pass = create_render_pass(&instance, &logical_device, swapchain.format)?;
            let descriptor_set_layout = create_descriptor_set_layout(&logical_device)?;
            let pipeline = create_pipeline(&logical_device, swapchain.extent, render_pass, descriptor_set_layout)?;
            let framebuffers = create_framebuffers(&logical_device, &image_views, render_pass, &swapchain.extent)?;
            let command_pool = create_command_pool(&logical_device, &indices)?;
            let vertex_buffer = create_vertex_buffer(&instance, &logical_device, physical_device, graphics_queue, command_pool)?;
            let index_buffer = create_index_buffer(&instance, &logical_device, physical_device, graphics_queue, command_pool)?;
            let uniform_buffers = create_uniform_buffers(&instance, &logical_device, physical_device, &images)?;
            let descriptor_pool = create_descriptor_pool(&logical_device, &images)?;
            let descriptor_sets = create_descriptor_sets(&logical_device, descriptor_pool, descriptor_set_layout, &images, &uniform_buffers)?;
            let command_buffers = create_command_buffers(&logical_device, command_pool, swapchain.extent, render_pass, &framebuffers, &pipeline, vertex_buffer.buffer, index_buffer.buffer, &descriptor_sets)?;

            let semaphore_info = vk::SemaphoreCreateInfo::builder();
            let fence_info = vk::FenceCreateInfo::builder()
                .flags(vk::FenceCreateFlags::SIGNALED)
                .build();

            let mut image_available_semaphores = Vec::new();
            let mut render_finished_semaphores = Vec::new();
            let mut in_flight_fences = Vec::new();

            for _ in 0..MAX_FRAMES_IN_FLIGHT {
                image_available_semaphores.push(logical_device.create_semaphore(&semaphore_info, None)?);
                render_finished_semaphores.push(logical_device.create_semaphore(&semaphore_info, None)?);
                in_flight_fences.push(logical_device.create_fence(&fence_info, None)?);
            }

            let images_in_flight = images.iter().map(|_| Fence::null()).collect();

            let start = Instant::now();

            Ok(Self {
                entry,
                instance,
                surface_loader,
                physical_device,
                logical_device,
                debug,
                graphics_queue,
                present_queue,
                surface,
                swapchain,
                pipeline,
                descriptor_set_layout,
                descriptor_pool,
                descriptor_sets,
                render_pass,
                framebuffers,
                command_pool,
                command_buffers,
                image_available_semaphores,
                render_finished_semaphores,
                frame: 0,
                in_flight_fences,
                images_in_flight,
                indices,
                images,
                image_views,
                vertex_buffer,
                index_buffer,
                uniform_buffers,
                start,
                resized: false
            })
        }
    }

    pub unsafe fn render(&mut self, window_dimensions: Vector2<f32>) -> Result<()> {
        let in_flight_fence = self.in_flight_fences[self.frame];

        self.logical_device.wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

        let result = self.swapchain.loader.acquire_next_image(
            self.swapchain.swapchain,
            u64::MAX,
            self.image_available_semaphores[self.frame],
            Fence::null(),
        );

        let image_index = match result {
            VkResult::Ok((image_index, _)) => image_index as usize,
            VkResult::Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return self.recreate_swapchain(window_dimensions),
            VkResult::Err(e) => return Err(anyhow!(e))
        };

        let image_in_flight = self.images_in_flight[image_index];
        if image_in_flight != Fence::null() {
            self.logical_device.wait_for_fences(&[image_in_flight], true, u64::MAX)?;
        }

        self.images_in_flight[image_index] = in_flight_fence;

        update_uniform_buffer(image_index, &self.uniform_buffers, self.swapchain.extent, &self.logical_device, &self.start)?;

        let wait_semaphores = &[self.image_available_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.command_buffers[image_index]];
        let signal_semaphores = &[self.render_finished_semaphores[self.frame]];
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
            .image_indices(image_indices)
            .build();

        let result = self.swapchain.loader.queue_present(self.present_queue, &present_info);

        let changed = match result {
            VkResult::Ok(true) => true,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => true,
            _ => false
        };

        if self.resized || changed {
            // debug!("Window resized resized: {:?} changed {:?}", self.resized, changed);
            self.resized = false;
            self.recreate_swapchain(window_dimensions)?;
        } else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.logical_device.device_wait_idle().unwrap();

            self.destroy_swapchain();

            self.logical_device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.in_flight_fences.iter().for_each(|f| self.logical_device.destroy_fence(*f, None));
            self.render_finished_semaphores.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));
            self.image_available_semaphores.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));

            self.logical_device.destroy_buffer(self.vertex_buffer.buffer, None);
            self.logical_device.free_memory(self.vertex_buffer.buffer_memory, None);
            self.logical_device.destroy_buffer(self.index_buffer.buffer, None);
            self.logical_device.free_memory(self.index_buffer.buffer_memory, None);

            self.logical_device.destroy_command_pool(self.command_pool, None);
            self.logical_device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            if VALIDATION_ENABLED {
                self.debug.debug_utils.destroy_debug_utils_messenger(self.debug.utils_messenger, None);
            }

            self.instance.destroy_instance(None);
        }
    }

    pub unsafe fn destroy_swapchain(&mut self) {
        self.logical_device.destroy_descriptor_pool(self.descriptor_pool, None);
        self.uniform_buffers
            .iter()
            .for_each(|b| {
                self.logical_device.destroy_buffer(b.buffer, None);
                self.logical_device.free_memory(b.buffer_memory, None);
            });
        self.logical_device.free_command_buffers(self.command_pool, &self.command_buffers);
        self.framebuffers
            .iter()
            .for_each(|f| self.logical_device.destroy_framebuffer(*f, None));
        self.logical_device.destroy_pipeline(self.pipeline.pipeline, None);
        self.logical_device.destroy_pipeline_layout(self.pipeline.pipeline_layout, None);
        self.logical_device.destroy_render_pass(self.render_pass, None);
        self.image_views
            .iter()
            .for_each(|v| self.logical_device.destroy_image_view(*v, None));
        self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);
    }

    pub unsafe fn recreate_swapchain(&mut self, window_dimensions: Vector2<f32>) -> Result<()> {
        self.logical_device.device_wait_idle().unwrap();

        self.destroy_swapchain();

        self.swapchain = VulkanAppSwapchain::get(window_dimensions, &self.instance, &self.logical_device, self.physical_device, &self.surface_loader, self.surface, &self.indices)?;
        self.images = self.swapchain.loader.get_swapchain_images(self.swapchain.swapchain)?;
        self.image_views = create_swapchain_image_views(&self.logical_device, &self.images, self.swapchain.format)?;
        self.render_pass = create_render_pass(&self.instance, &self.logical_device, self.swapchain.format)?;
        self.pipeline = create_pipeline(&self.logical_device, self.swapchain.extent, self.render_pass, self.descriptor_set_layout)?;
        self.framebuffers = create_framebuffers(&self.logical_device, &self.image_views, self.render_pass, &self.swapchain.extent)?;
        self.uniform_buffers = create_uniform_buffers(&self.instance, &self.logical_device, self.physical_device, &self.images)?;
        self.descriptor_pool = create_descriptor_pool(&self.logical_device, &self.images)?;
        self.descriptor_sets = create_descriptor_sets(&self.logical_device, self.descriptor_pool, self.descriptor_set_layout, &self.images, &self.uniform_buffers)?;
        self.command_buffers = create_command_buffers(&self.logical_device, self.command_pool, self.swapchain.extent, self.render_pass, &self.framebuffers, &self.pipeline, self.vertex_buffer.buffer, self.index_buffer.buffer, &self.descriptor_sets)?;
        self.images_in_flight.resize(self.images.len(), Fence::null());
        Ok(())
    }
}