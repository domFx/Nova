use crate::vertex::Vertex;
use crate::vulkan::utils::shader::create_shader_module;

use std::ffi::CStr;

use anyhow::Result;
use ash::vk;

pub struct VulkanAppPipeline {
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl VulkanAppPipeline {
    pub fn new(
        device: &ash::Device,
        _extent: vk::Extent2D,
        render_pass: vk::RenderPass,
        descriptor_set_layout: vk::DescriptorSetLayout,
    ) -> Result<VulkanAppPipeline> {
        let vert =
            ash::util::read_spv(&mut std::fs::File::open("./src/shaders/vert.spv").unwrap()).unwrap();
        let frag =
            ash::util::read_spv(&mut std::fs::File::open("./src/shaders/frag.spv").unwrap()).unwrap();

        unsafe {

            let vertex_shader_module = create_shader_module(device, &vert[..])?;
            let fragment_shader_module = create_shader_module(device, &frag[..])?;

            let vertex_stage = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vertex_shader_module)
                .name(CStr::from_bytes_with_nul_unchecked(b"main\0"))
                .build();

            let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_shader_module)
                .name(CStr::from_bytes_with_nul_unchecked(b"main\0"))
                .build();

            let binding_descriptions = &[Vertex::binding_description()];
            let attribute_descriptions = Vertex::attribute_descriptions();
            let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
                .vertex_binding_descriptions(binding_descriptions)
                .vertex_attribute_descriptions(&attribute_descriptions)
                .build();

            let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
                .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
                .primitive_restart_enable(false);

            // let viewports = &[viewport];
            // let scissors = &[scissor];
            let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
                .viewport_count(1)
                .scissor_count(1)
                // .viewports(viewports)
                // .scissors(scissors)
                .build();

            let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
                .depth_clamp_enable(false)
                .rasterizer_discard_enable(false)
                .polygon_mode(vk::PolygonMode::FILL)
                .line_width(1.0)
                .cull_mode(vk::CullModeFlags::NONE)
                // .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
                .front_face(vk::FrontFace::CLOCKWISE)
                .depth_bias_enable(false)
                .build();

            let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
                .sample_shading_enable(false)
                .rasterization_samples(vk::SampleCountFlags::TYPE_1)
                .build();

            let attachment = vk::PipelineColorBlendAttachmentState::builder()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(false)
                // .src_color_blend_factor(vk::BlendFactor::ONE)
                // .dst_color_blend_factor(vk::BlendFactor::ZERO)
                // .color_blend_op(vk::BlendOp::ADD)
                // .src_alpha_blend_factor(vk::BlendFactor::ONE)
                // .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
                // .alpha_blend_op(vk::BlendOp::ADD)
                .build();

            let attachments = &[attachment];
            let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
                .logic_op_enable(false)
                .logic_op(vk::LogicOp::COPY)
                .attachments(attachments)
                .blend_constants([0.0, 0.0, 0.0, 0.0])
                .build();

            let dynamic_states = vec![vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::builder()
                .dynamic_states(&dynamic_states)
                .build();

            let layout_info = vk::PipelineLayoutCreateInfo::builder()
                .set_layouts(&[descriptor_set_layout])
                .push_constant_ranges(&[])
                .build();

            let pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;

            let stages = &[vertex_stage, frag_stage];
            let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
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
                .dynamic_state(&dynamic_state_info)
                .build();

            let pipeline = device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .expect("Unable to create graphics pipeline")
                .pop()
                .unwrap();

            // Cleanup
            device.destroy_shader_module(vertex_shader_module, None);
            device.destroy_shader_module(fragment_shader_module, None);

            Ok(VulkanAppPipeline {
                pipeline_layout,
                pipeline,
            })
        }
    }
}