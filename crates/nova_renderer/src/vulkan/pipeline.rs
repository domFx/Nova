use crate::vertex::Vertex;
use crate::vulkan::utils::shader::create_shader_module;

use std::ffi::CStr;

use anyhow::Result;
use ash::vk;

use super::vulkan_renderer::VulkanTopology;

pub struct VulkanPipeline {
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline
}

impl VulkanPipeline {
    pub(crate) fn new(
        device: &ash::Device,
        render_pass: vk::RenderPass,
        descriptor_set_layout: vk::DescriptorSetLayout,
        topology: VulkanTopology
    ) -> Result<VulkanPipeline> {
        let vert =
            ash::util::read_spv(&mut std::fs::File::open("./src/shaders/vert.spv")?)?;
        let frag =
            ash::util::read_spv(&mut std::fs::File::open("./src/shaders/frag.spv")?)?;

        unsafe {
            let vertex_shader_module = create_shader_module(device, &vert[..])?;
            let fragment_shader_module = create_shader_module(device, &frag[..])?;

            let vertex_stage = vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vertex_shader_module)
                .name(CStr::from_bytes_with_nul_unchecked(b"main\0"));

            let frag_stage = vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_shader_module)
                .name(CStr::from_bytes_with_nul_unchecked(b"main\0"));

            let binding_descriptions = &[Vertex::binding_description()];
            let attribute_descriptions = Vertex::attribute_descriptions();
            let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default()
                .vertex_binding_descriptions(binding_descriptions)
                .vertex_attribute_descriptions(&attribute_descriptions);

            let input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo = vk::PipelineInputAssemblyStateCreateInfo::default()
                .topology(topology.to_vulkan_topology())
                .primitive_restart_enable(false);

            let viewport_state = vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1);

            let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
                .depth_clamp_enable(false)
                .rasterizer_discard_enable(false)
                .polygon_mode(vk::PolygonMode::FILL)
                .line_width(1.0)
                .cull_mode(vk::CullModeFlags::BACK)
                .front_face(vk::FrontFace::CLOCKWISE)
                .depth_bias_enable(false);

            let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
                .sample_shading_enable(false)
                .rasterization_samples(vk::SampleCountFlags::TYPE_1);

            let attachment = vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(false);

            let attachments = &[attachment];
            let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
                .attachments(attachments);

            let dynamic_states = vec![vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
                .dynamic_states(&dynamic_states);

            let layouts = &[descriptor_set_layout];
            let layout_info = vk::PipelineLayoutCreateInfo::default()
                .set_layouts(layouts)
                .push_constant_ranges(&[]);

            let pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;

            let stages = &[vertex_stage, frag_stage];
            let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
                .stages(stages)
                .vertex_input_state(&vertex_input_state)
                .input_assembly_state(&input_assembly_state)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterization_state)
                .multisample_state(&multisample_state)
                .color_blend_state(&color_blend_state)
                .layout(pipeline_layout)
                .render_pass(render_pass)
                .subpass(0)
                .dynamic_state(&dynamic_state_info);

            let pipeline = device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .expect("Unable to create graphics pipeline")
                .pop()
                .unwrap();

            // Cleanup
            device.destroy_shader_module(vertex_shader_module, None);
            device.destroy_shader_module(fragment_shader_module, None);

            Ok(Self {
                pipeline_layout,
                pipeline,
            })
        }
    }

    pub(crate) fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}