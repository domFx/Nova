use crate::core_types::PrimitiveTopologyType;
use crate::vertex;
use crate::vertex::Vertex;
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
use crate::vulkan::texture_image::*;

use crate::vulkan::vulkan_buffer::{ VulkanBuffer, VulkanBufferType };
use crate::vulkan::constants::{MAX_FRAMES_IN_FLIGHT, VALIDATION_ENABLED};
use crate::vulkan::debug::{create_debug_info, VulkanAppDebug};
use crate::vulkan::descriptor_pool::create_descriptor_pool;
use crate::vulkan::descriptor_sets::create_descriptor_sets;
use crate::vulkan::uniform_buffers::{create_uniform_buffers, update_uniform_buffer};
use crate::renderer_backend::RendererBackend;

use anyhow::Ok;
use ash::ext::index_type_uint8;
use ash::vk::PrimitiveTopology;
use nova_core::color::Color;
use nova_math::vector::vector2::Vector2;

use anyhow::{anyhow, Result};
use ash::vk;
use ash::khr;
use ash::prelude::VkResult;
use png::chunk::acTL;
use std::time::Instant;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use super::pipeline;

pub(crate) enum VulkanTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan
}

impl VulkanTopology {
    pub(crate) fn to_vulkan_topology(&self) -> vk::PrimitiveTopology {
        match self {
            VulkanTopology::PointList => vk::PrimitiveTopology::POINT_LIST,
            VulkanTopology::LineList => vk::PrimitiveTopology::LINE_LIST,
            VulkanTopology::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
            VulkanTopology::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
            VulkanTopology::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
            VulkanTopology::TriangleFan => vk::PrimitiveTopology::TRIANGLE_FAN,
            _ => vk::PrimitiveTopology::TRIANGLE_LIST
        }
    }
}

pub struct VulkanRenderer {
    entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: khr::surface::Instance,
    logical_device: ash::Device,
    debug: VulkanAppDebug,
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    surface: vk::SurfaceKHR,
    swapchain: VulkanAppSwapchain,
    
    // TODO: Make pipeline stuff better
    line_pipeline: VulkanPipeline,
    triangle_pipeline: VulkanPipeline,

    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    render_pass: vk::RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,    
    command_buffers: Vec<vk::CommandBuffer>,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    current_frame: usize,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,
    indices: QueueFamilyIndices,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    uniform_buffers: Vec<VulkanBuffer>,
    start: Instant,
    window_dimensions: Vector2<u32>,
    texture_image: VulkanImage,
    sampler: vk::Sampler,
    pub resized: bool,
    active_command_buffer: Option<vk::CommandBuffer>,
    current_image_index: Option<usize>
}

impl RendererBackend for VulkanRenderer {
    fn prepare_window_surface(&mut self, window_dimensions: Vector2<u32>) -> Result<()> {
        unsafe {
            self.logical_device.wait_for_fences(&[self.in_flight_fences[self.current_frame]], true, u64::MAX)?;

            let result = self.swapchain.loader.acquire_next_image(
                self.swapchain.swapchain,
                u64::MAX,
                self.image_available_semaphores[self.current_frame],
                vk::Fence::null(),
            );

            let image_index = match result {
                VkResult::Ok((image_index, _)) => image_index as usize,
                VkResult::Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return self.recreate_swapchain(self.window_dimensions),
                VkResult::Err(e) => return Err(anyhow!(e))
            };
            self.current_image_index = Some(image_index);
            
            update_uniform_buffer(self.current_frame, &self.uniform_buffers, self.swapchain.extent, &self.logical_device, &self.start)?;
            
            self.logical_device.reset_fences(&[self.in_flight_fences[self.current_frame]])?;
        }

        Ok(())
    }

    fn begin_commands(&mut self) -> Result<()> {
        unsafe {
            self.logical_device.reset_command_buffer(self.command_buffers[self.current_frame], vk::CommandBufferResetFlags::empty())?;
        }

        self.active_command_buffer = Some(self.command_buffers[self.current_frame]);

        let current_image_index = if let Some(image_index) = self.current_image_index {
            image_index
        } else {
            return Err(anyhow!("Current image index found"));
        };

        let active_command_buffer = if let Some(active_command_buffer) = self.active_command_buffer {
            active_command_buffer
        } else {
            return Err(anyhow!("No active command buffer found"));
        };

        unsafe {
            let info = vk::CommandBufferBeginInfo::default();

            self.logical_device.begin_command_buffer(active_command_buffer, &info)?;
            Ok(())
        }
    }

    fn begin_render_pass(&mut self, clear_color: Color) -> Result<()> {        
        let render_area = vk::Rect2D::default()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(self.swapchain.extent);

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: clear_color.to_rgb_array(),
            },
        };

        let clear_values = &[color_clear_value];
        let info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[self.current_image_index.unwrap()])
            .render_area(render_area)
            .clear_values(clear_values);
        
        unsafe {
            self.logical_device.cmd_begin_render_pass(self.active_command_buffer.unwrap(), &info, vk::SubpassContents::INLINE);
        }
        
        Ok(())
    }

    fn draw(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, topology_type: PrimitiveTopologyType) -> Result<()> {
        // TODO: Improve pipeline selection
        let pipeline = match topology_type {
            PrimitiveTopologyType::LineList => &self.line_pipeline,
            PrimitiveTopologyType::TriangleList => &self.triangle_pipeline,
            _ => &self.triangle_pipeline
        };

        unsafe {
            self.logical_device.cmd_bind_pipeline(self.active_command_buffer.unwrap(), vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline);

            let viewport = vk::Viewport::default()
                .x(0.0)
                .y(self.swapchain.extent.height as f32)
                .width(self.swapchain.extent.width as f32)
                .height(-(self.swapchain.extent.height as f32))
                .min_depth(0.0)
                .max_depth(1.0);
            self.logical_device.cmd_set_viewport(self.active_command_buffer.unwrap(), 0, &[viewport]);

            let scissor = vk::Rect2D::default()
                .offset(vk::Offset2D { x: 0, y: 0 })
                .extent(self.swapchain.extent);
            self.logical_device.cmd_set_scissor(self.active_command_buffer.unwrap(), 0, &[scissor]);

            let vertex_buffer = VulkanBuffer::new_staged(
                vertices.as_ptr(), 
                vertices.len(), 
                &self.logical_device, 
                &self.instance, 
                self.physical_device, 
                self.graphics_queue, 
                self.command_pool,         
                VulkanBufferType::Vertex)?;

            let index_buffer = VulkanBuffer::new_staged(
                indices.as_ptr(), 
                indices.len(), 
                &self.logical_device, 
                &self.instance, 
                self.physical_device, 
                self.graphics_queue, 
                self.command_pool,
                VulkanBufferType::Index)?;

            self.logical_device.cmd_bind_vertex_buffers(self.active_command_buffer.unwrap(), 0, &[vertex_buffer.buffer], &[0]);
            self.logical_device.cmd_bind_index_buffer(self.active_command_buffer.unwrap(), index_buffer.buffer, 0, vk::IndexType::UINT32);

            self.logical_device.cmd_bind_descriptor_sets(
                self.active_command_buffer.unwrap(),
                vk::PipelineBindPoint::GRAPHICS,
                pipeline.pipeline_layout,
                0,
                &[self.descriptor_sets[self.current_frame]],
                &[]);

            self.logical_device.cmd_draw_indexed(self.active_command_buffer.unwrap(), indices.len() as u32, 1, 0, 0, 0);
       
            // vertex_buffer.destroy_buffer(&self.logical_device);
            // index_buffer.destroy_buffer(&self.logical_device);
        }

        Ok(())
    }

    fn end_render_pass(&mut self) -> Result<()> {
        unsafe {
            self.logical_device.cmd_end_render_pass(self.active_command_buffer.expect("No active command buffer"));
        }
        Ok(())
    }

    fn end_commands(&mut self) -> Result<()> {
        unsafe { 
            self.logical_device.end_command_buffer(self.active_command_buffer.expect("No active command buffer"))?;
        }

        Ok(())
    }

    fn submit(&mut self) -> Result<()> {
        let wait_semaphores = &[self.image_available_semaphores[self.current_frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.command_buffers[self.current_frame]];
        let signal_semaphores = &[self.render_finished_semaphores[self.current_frame]];
        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        unsafe { 
            self.logical_device.queue_submit(self.graphics_queue, &[submit_info], self.in_flight_fences[self.current_frame])?;
        }

        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        let swapchains = &[self.swapchain.swapchain];
        let image_indices = &[self.current_image_index.unwrap() as u32];
        let signal_semaphores = &[self.render_finished_semaphores[self.current_frame]];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        unsafe {
            let result = self.swapchain.loader.queue_present(self.present_queue, &present_info);

            let changed = match result {
                VkResult::Ok(true) => true,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => true,
                _ => false
            };
    
            if self.resized || changed {
                // debug!("Window resized resized: {:?} changed {:?}", self.resized, changed);
                self.resized = false;
                self.recreate_swapchain(self.window_dimensions)?;
            } else if let Err(e) = result {
                return Err(anyhow!(e));
            }        
    
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    fn destroy(&self) {
        unsafe {
            self.logical_device.device_wait_idle().unwrap();

            self.destroy_swapchain();

            self.logical_device.destroy_sampler(self.sampler, None);
            self.texture_image.destroy_vulkan_image(&self.logical_device);

            self.logical_device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.in_flight_fences.iter().for_each(|f| self.logical_device.destroy_fence(*f, None));
            self.render_finished_semaphores.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));
            self.image_available_semaphores.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));

            self.logical_device.destroy_command_pool(self.command_pool, None);
            self.logical_device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            if VALIDATION_ENABLED {
                self.debug.debug_utils.destroy_debug_utils_messenger(self.debug.utils_messenger, None);
            }

            self.instance.destroy_instance(None);
        }
    }
}

impl VulkanRenderer {
    pub fn new(raw_window_handle: RawWindowHandle, raw_display_handle: RawDisplayHandle, window_dimensions: Vector2<u32>) -> Result<Self> {
        unsafe {
            let entry = ash::Entry::load()?;
            let mut debug_info = create_debug_info();
            let instance = create_instance(&entry, &mut debug_info)?;
            let debug = VulkanAppDebug::get(&entry, &instance, debug_info)?;
            let surface_loader = khr::surface::Instance::new(&entry, &instance);
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
            let triangle_pipeline = VulkanPipeline::new(&logical_device, render_pass, descriptor_set_layout, VulkanTopology::TriangleList)?;
            let line_pipeline = VulkanPipeline::new(&logical_device, render_pass, descriptor_set_layout, VulkanTopology::LineList)?;
            
            let framebuffers = create_framebuffers(&logical_device, &image_views, render_pass, &swapchain.extent)?;
            let command_pool = create_command_pool(&logical_device, &indices)?;
            let texture_image = create_texture_image(&instance, &logical_device, physical_device, command_pool, graphics_queue)?;
            let sampler = create_texture_sampler(&logical_device)?;
            let uniform_buffers = create_uniform_buffers(&instance, &logical_device, physical_device)?;
            let descriptor_pool = create_descriptor_pool(&logical_device, &images)?;
            let descriptor_sets = create_descriptor_sets(&logical_device, descriptor_pool, descriptor_set_layout, &images, &uniform_buffers, texture_image.texture_image_view, sampler)?;
            let command_buffers = create_command_buffers(&logical_device, command_pool, &framebuffers)?;

            let semaphore_info = vk::SemaphoreCreateInfo::default();
            let fence_info = vk::FenceCreateInfo::default()
                .flags(vk::FenceCreateFlags::SIGNALED);

            let mut image_available_semaphores = Vec::new();
            let mut render_finished_semaphores = Vec::new();
            let mut in_flight_fences = Vec::new();

            for _ in 0..MAX_FRAMES_IN_FLIGHT {
                image_available_semaphores.push(logical_device.create_semaphore(&semaphore_info, None)?);
                render_finished_semaphores.push(logical_device.create_semaphore(&semaphore_info, None)?);
                in_flight_fences.push(logical_device.create_fence(&fence_info, None)?);
            }

            let images_in_flight = images.iter().map(|_| vk::Fence::null()).collect();

            let start = Instant::now();

            let renderer = Self {
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
                triangle_pipeline,
                line_pipeline,
                descriptor_set_layout,
                descriptor_pool,
                descriptor_sets,
                render_pass,
                framebuffers,
                command_pool,
                command_buffers,
                image_available_semaphores,
                render_finished_semaphores,
                current_frame: 0,
                in_flight_fences,
                images_in_flight,
                indices,
                images,
                image_views,
                uniform_buffers,
                start,
                window_dimensions,
                texture_image,
                sampler,
                resized: false,
                active_command_buffer: None,
                current_image_index: None
            };

            Ok(renderer)
        }
    }

    fn destroy_swapchain(&self) {
        unsafe {
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
            self.triangle_pipeline.destroy(&self.logical_device);
            self.line_pipeline.destroy(&self.logical_device);
            self.logical_device.destroy_render_pass(self.render_pass, None);
            self.image_views
                .iter()
                .for_each(|v| self.logical_device.destroy_image_view(*v, None));
            self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);
        }
    }

    fn recreate_swapchain(&mut self, window_dimensions: Vector2<u32>) -> Result<()> {
        unsafe {
            self.logical_device.device_wait_idle().unwrap();

            self.destroy_swapchain();

            self.swapchain = VulkanAppSwapchain::get(window_dimensions, &self.instance, &self.logical_device, self.physical_device, &self.surface_loader, self.surface, &self.indices)?;
            self.images = self.swapchain.loader.get_swapchain_images(self.swapchain.swapchain)?;
            self.image_views = create_swapchain_image_views(&self.logical_device, &self.images, self.swapchain.format)?;
            self.render_pass = create_render_pass(&self.instance, &self.logical_device, self.swapchain.format)?;
            
            self.triangle_pipeline = VulkanPipeline::new(&self.logical_device, self.render_pass, self.descriptor_set_layout, VulkanTopology::TriangleList)?;
            self.line_pipeline = VulkanPipeline::new(&self.logical_device, self.render_pass, self.descriptor_set_layout, VulkanTopology::LineList)?;
            
            self.framebuffers = create_framebuffers(&self.logical_device, &self.image_views, self.render_pass, &self.swapchain.extent)?;
            self.uniform_buffers = create_uniform_buffers(&self.instance, &self.logical_device, self.physical_device)?;
            self.descriptor_pool = create_descriptor_pool(&self.logical_device, &self.images)?;
            self.descriptor_sets = create_descriptor_sets(&self.logical_device, self.descriptor_pool, self.descriptor_set_layout, &self.images, &self.uniform_buffers, self.texture_image.texture_image_view, self.sampler)?;
            self.command_buffers = create_command_buffers(&self.logical_device, self.command_pool, &self.framebuffers)?;
            self.images_in_flight.resize(self.images.len(), vk::Fence::null());
        }
        Ok(())
    }
}