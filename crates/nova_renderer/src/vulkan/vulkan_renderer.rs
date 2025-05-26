use crate::core_types::PrimitiveTopologyType;
use crate::pipeline_config::PipelineConfig;
use crate::pipeline_config::PipelineType;
use crate::pipeline_config::DEFAULT;
use crate::pipeline_config::DEFAULT_LINE;
use crate::pipeline_config::TRANSPARENT;
use crate::renderer_backend::RendererBackend;
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
use crate::vulkan::vulkan_buffer::{VulkanBuffer, VulkanBufferType};
use crate::vulkan::constants::{MAX_FRAMES_IN_FLIGHT, VALIDATION_ENABLED};
use crate::vulkan::debug::{create_debug_info, VulkanAppDebug};
use crate::vulkan::descriptor_pool::create_descriptor_pool;
use crate::vulkan::descriptor_sets::create_descriptor_sets;
use crate::vulkan::uniform_buffers::{create_uniform_buffers, update_uniform_buffer};

use nova_core::image::Image;
use nova_core::color::Color;
use nova_math::vector::Vector2;

use anyhow::{anyhow, Result};
use ash::{vk, khr, prelude::VkResult};
use ash::ext::index_type_uint8;
use ash::vk::PrimitiveTopology;
use raw_window_handle::{DisplayHandle, WindowHandle};
use std::collections::HashMap;
use std::time::Instant;
use std::u64;
use std::usize::MAX;
use std::vec;

use super::pipeline;

static PIPELINES_TYPES: [&PipelineConfig; 3] = [ 
    &DEFAULT,
    &DEFAULT_LINE,
    &TRANSPARENT
];

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
    
    pipelines: HashMap<PipelineType, VulkanPipeline>,

    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
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
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,
    uniform_buffers: Vec<VulkanBuffer>,
    start: Instant,
    window_dimensions: Vector2<u32>,
    pub resized: bool,

    physical_device_constriants: PhysicalDeviceConstraints,
    
    active_command_buffer: Option<vk::CommandBuffer>,
    current_image_index: Option<usize>,
    
    created_buffers: Vec<Vec<VulkanBuffer>>,

    uploaded_texture_images: Vec<VulkanTextureImage>,
    descriptor_sets: Vec<vk::DescriptorSet>,
}

struct PhysicalDeviceConstraints {
    max_per_stage_descriptor_samplers: u32,
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

    fn draw(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, pipeline_type: PipelineType) -> Result<()> {
        let pipeline = match pipeline_type {
            PipelineType::Default => self.pipelines.get(&PipelineType::Default).unwrap(),
            PipelineType::DefaultLine => self.pipelines.get(&PipelineType::DefaultLine).unwrap(),
            PipelineType::Transparent => self.pipelines.get(&PipelineType::Transparent).unwrap(),
        };

        let format = if pipeline_type == PipelineType::Transparent { 
            vk::Format::R8_UNORM
        } else {
            vk::Format::R8G8B8A8_SRGB
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

            self.created_buffers[self.current_frame].push(index_buffer);
            self.created_buffers[self.current_frame].push(vertex_buffer);

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

            self.logical_device.wait_for_fences(&[self.in_flight_fences[self.current_frame]], true, u64::MAX);

            // Clean up frame resources
            self.created_buffers[self.current_frame].iter().for_each(|b| b.destroy_buffer(&self.logical_device));
            self.created_buffers[self.current_frame].clear();

            // Check if the window was resized or if the swapchain needs to be recreated
            let changed = match result {
                VkResult::Ok(true) => true,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => true,
                _ => false
            };
    
            if self.resized || changed {
                self.resized = false;
                self.recreate_swapchain(self.window_dimensions)?;
            } else if let Err(e) = result {
                return Err(anyhow!(e));
            }        
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    fn load_images(&mut self, images: &Vec<Image>) -> Result<()> {
        log::debug!("Loading images {:?}", images.len());
        // Check if we need to reload descriptor sets if we have images where we didnt before
        let reload_descriptor_sets = self.uploaded_texture_images.len() == 0;

        // Temp, reload all images on each render pass (obviously this is inefficient, we will need a step to load new images) 
        self.uploaded_texture_images.iter().for_each(|texture_image| {
            texture_image.destroy_vulkan_image(&self.logical_device);
        });

        unsafe {
            self.logical_device.free_descriptor_sets(self.descriptor_pool, &self.descriptor_sets)?;
        }
    
        let texture_images = images.iter().map(|image| {
            // TODO: Fix use of expect 
            create_texture_image(
                image, 
                &self.instance, 
                &self.logical_device, 
                self.physical_device, 
                self.command_pool, 
                self.graphics_queue,
                vk::Format::R32G32B32A32_SFLOAT).expect("Error creating texture_image")
            }).collect::<Vec<_>>();

        // TODO: texture_images will be the "global" list of textures uploaded to the GPU.  
        if texture_images.len() == 0 {
            self.descriptor_sets = create_descriptor_sets(
                &self.logical_device, 
                self.descriptor_pool,
                self.descriptor_set_layout,
                &self.swapchain_images,
                &self.uniform_buffers, 
                None)?;
        } else {
            self.descriptor_sets = create_descriptor_sets(
                &self.logical_device, 
                self.descriptor_pool,
                self.descriptor_set_layout,
                &self.swapchain_images,
                &self.uniform_buffers, 
                Some(&texture_images))?;
        }
        
        self.uploaded_texture_images = texture_images;

        Ok(())
    }

    fn destroy(&self) {
        unsafe {
            self.logical_device.device_wait_idle().unwrap();

            self.destroy_swapchain();

            self.uploaded_texture_images.iter().for_each(|texture_image| {
                texture_image.destroy_vulkan_image(&self.logical_device);
            });

            self.logical_device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            self.in_flight_fences.iter().for_each(|f| self.logical_device.destroy_fence(*f, None));
            self.render_finished_semaphores.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));
            self.image_available_semaphores.iter().for_each(|s| self.logical_device.destroy_semaphore(*s, None));

            self.created_buffers.iter()
                .for_each(|cb| { 
                    cb.iter().for_each(|b| b.destroy_buffer(&self.logical_device));
                });

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
    pub fn new(window_handle: WindowHandle, display_handle: DisplayHandle, window_dimensions: Vector2<u32>) -> Result<Self> {
        unsafe {
            let entry = ash::Entry::load()?;
            let mut debug_info = create_debug_info();
            let instance = create_instance(&entry, &mut debug_info)?;
            let debug = VulkanAppDebug::get(&entry, &instance, debug_info)?;
            let surface_loader = khr::surface::Instance::new(&entry, &instance);
            let surface = create_surface(window_handle, display_handle, &entry, &instance)?;
            let physical_device = pick_physical_device(&instance, &surface_loader, surface)?;
            let indices = QueueFamilyIndices::get(&instance, physical_device, surface, &surface_loader)?;
            let logical_device = create_logical_device(&entry, &instance, physical_device, &indices)?;
            let graphics_queue = logical_device.get_device_queue(indices.graphics, 0);
            let present_queue = logical_device.get_device_queue(indices.present, 0);
            let swapchain = VulkanAppSwapchain::get(window_dimensions, &instance, &logical_device, physical_device, &surface_loader, surface, &indices)?;
            let swapchain_images = swapchain.loader.get_swapchain_images(swapchain.swapchain)?;
            let swapchain_image_views =  create_swapchain_image_views(&logical_device, &swapchain_images, swapchain.format)?;
            let render_pass = create_render_pass(&logical_device, swapchain.format)?;
            let descriptor_set_layout = create_descriptor_set_layout(&logical_device)?;

            let mut pipelines = HashMap::new();
            for pipeline in PIPELINES_TYPES {
                let vulkan_pipeline = VulkanPipeline::new(
                    &logical_device, 
                    render_pass, 
                    descriptor_set_layout, 
                    pipeline)?;

                if !pipelines.contains_key(&pipeline.pipeline_type) {
                    pipelines.insert(pipeline.pipeline_type, vulkan_pipeline);
                }
            }
   
            let framebuffers = create_framebuffers(&logical_device, &swapchain_image_views, render_pass, &swapchain.extent)?;
            let command_pool = create_command_pool(&logical_device, &indices)?;

            let uniform_buffers = create_uniform_buffers(&instance, &logical_device, physical_device)?;
            let descriptor_pool = create_descriptor_pool(&logical_device, &swapchain_images)?;
            let mut descriptor_sets = create_descriptor_sets(&logical_device, descriptor_pool, descriptor_set_layout, &swapchain_images, &uniform_buffers, None)?;
            let command_buffers = create_command_buffers(&logical_device, command_pool, &framebuffers)?;

            let props = instance.get_physical_device_properties(physical_device);
            let max_per_stage_descriptor_samplers = props.limits.max_per_stage_descriptor_samplers;

            let physical_device_constriants = PhysicalDeviceConstraints {
                max_per_stage_descriptor_samplers
            };

            let semaphore_info = vk::SemaphoreCreateInfo::default();
            let fence_info = vk::FenceCreateInfo::default()
                .flags(vk::FenceCreateFlags::SIGNALED);

            let mut image_available_semaphores = Vec::new();
            let mut render_finished_semaphores = Vec::new();
            let mut in_flight_fences = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
            let mut created_buffers = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);

            for _ in 0..MAX_FRAMES_IN_FLIGHT {
                image_available_semaphores.push(logical_device.create_semaphore(&semaphore_info, None)?);
                render_finished_semaphores.push(logical_device.create_semaphore(&semaphore_info, None)?);
                in_flight_fences.push(logical_device.create_fence(&fence_info, None)?);
                created_buffers.push(Vec::new());
            }

            let images_in_flight = swapchain_images.iter().map(|_| vk::Fence::null()).collect();

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
                pipelines,
                descriptor_set_layout,
                descriptor_pool,
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
                swapchain_images,
                swapchain_image_views,
                uniform_buffers,
                start,
                window_dimensions,
                resized: false,
                physical_device_constriants,
                active_command_buffer: None,
                current_image_index: None,
                created_buffers,
                uploaded_texture_images: vec![],
                descriptor_sets,
            };

            Ok(renderer)
        }
    }

    fn destroy_swapchain(&self) {
        unsafe {
            self.logical_device.free_descriptor_sets(self.descriptor_pool, &self.descriptor_sets);
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

            self.pipelines.values()
                .for_each(|p| p.destroy(&self.logical_device));

            self.logical_device.destroy_render_pass(self.render_pass, None);
            self.swapchain_image_views
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
            self.swapchain_images = self.swapchain.loader.get_swapchain_images(self.swapchain.swapchain)?;
            self.swapchain_image_views = create_swapchain_image_views(&self.logical_device, &self.swapchain_images, self.swapchain.format)?;
            self.render_pass = create_render_pass(&self.logical_device, self.swapchain.format)?;
            
            for pipeline in PIPELINES_TYPES {
                let vulkan_pipeline = VulkanPipeline::new(
                    &self.logical_device, 
                    self.render_pass, 
                    self.descriptor_set_layout, 
                    pipeline)?;

                    self.pipelines.insert(pipeline.pipeline_type, vulkan_pipeline);
            }

            self.framebuffers = create_framebuffers(&self.logical_device, &self.swapchain_image_views, self.render_pass, &self.swapchain.extent)?;
            self.uniform_buffers = create_uniform_buffers(&self.instance, &self.logical_device, self.physical_device)?;
            self.descriptor_pool = create_descriptor_pool(&self.logical_device, &self.swapchain_images)?;

            if self.uploaded_texture_images.len() > 0 {
                self.descriptor_sets = create_descriptor_sets(&self.logical_device, self.descriptor_pool, self.descriptor_set_layout, &self.swapchain_images, &self.uniform_buffers, Some(&self.uploaded_texture_images))?;
            } else {
                self.descriptor_sets = create_descriptor_sets(&self.logical_device, self.descriptor_pool, self.descriptor_set_layout, &self.swapchain_images, &self.uniform_buffers, None)?;
            }

            self.command_buffers = create_command_buffers(&self.logical_device, self.command_pool, &self.framebuffers)?;
            self.images_in_flight.resize(self.swapchain_images.len(), vk::Fence::null());
        }
        Ok(())
    }
}