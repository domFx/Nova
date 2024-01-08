use std::ffi::CStr;

use anyhow::Result;
use ash::{Device, vk};
use ash::vk::{Pipeline, PipelineLayout, RenderPass};

pub struct VulkanAppPipeline {
    pub pipeline_layout: PipelineLayout,
    pub pipeline: Pipeline
}
pub unsafe fn create_pipeline(device: &Device, extent: vk::Extent2D, render_pass: RenderPass) -> Result<VulkanAppPipeline> {
    let vert = ash::util::read_spv(&mut std::fs::File::open("./src/shaders/vert.spv").unwrap()).unwrap();
    let frag = ash::util::read_spv(&mut std::fs::File::open("./src/shaders/frag.spv").unwrap()).unwrap();

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

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder();

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    let viewport = vk::Viewport::builder()
        .x(0.0)
        .y(0.0)
        .width(extent.width as f32)
        .height(extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)
        .build();

    let scissor = vk::Rect2D::builder()
        .offset(vk::Offset2D { x: 0, y: 0 })
        .extent(extent)
        .build();

    let viewports = &[viewport];
    let scissors = &[scissor];
    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(viewports)
        .scissors(scissors);

    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::CLOCKWISE)
        .depth_bias_enable(false);

    let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);

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
        .blend_constants([0.0, 0.0, 0.0, 0.0]);

    let layout_info = vk::PipelineLayoutCreateInfo::builder();

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
        .build();

    let pipeline = device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
        .expect("Unable to create graphics pipeline")
        .pop()
        .unwrap();

    // Cleanup
    device.destroy_shader_module(vertex_shader_module, None);
    device.destroy_shader_module(fragment_shader_module, None);

    Ok(VulkanAppPipeline {
        pipeline_layout,
        pipeline
    })
}

unsafe fn create_shader_module(device: &Device, bytecode: &[u32]) -> Result<vk::ShaderModule> {
    let info = vk::ShaderModuleCreateInfo::builder()
        .code(bytecode);

    let shader_module = device.create_shader_module(&info, None)?;

    Ok(shader_module)
}