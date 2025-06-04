use crate::core_types::PrimitiveTopologyType;

pub(crate) enum FaceCullMode {
    None,
    Front,
    Back,
    FrontAndBack,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub(crate) enum PipelineType {
    Default,
    DefaultLine,
    Transparent,   
}

pub(crate) struct PipelineConfig<'a> {
    pub(crate) pipeline_type: PipelineType,
    pub(crate) topology_type: PrimitiveTopologyType,

    pub(crate) vertex_shader_path: &'a str,
    pub(crate) fragment_shader_path: &'a str,

    pub(crate) face_cull_mode: FaceCullMode,
    pub(crate) blend_enabled: bool
}

pub(crate) static DEFAULT: PipelineConfig = PipelineConfig {
    pipeline_type: PipelineType::Default,
    topology_type: PrimitiveTopologyType::TriangleList,
    vertex_shader_path: "./src/shaders/default_vert.spv",
    fragment_shader_path: "./src/shaders/default_frag.spv",
    face_cull_mode: FaceCullMode::Back,
    blend_enabled: true
};

pub(crate) static DEFAULT_LINE: PipelineConfig = PipelineConfig {
    pipeline_type: PipelineType::DefaultLine,
    topology_type: PrimitiveTopologyType::LineList,
    vertex_shader_path: "./src/shaders/default_vert.spv",
    fragment_shader_path: "./src/shaders/default_frag.spv",
    face_cull_mode: FaceCullMode::Back,
    blend_enabled: true
};


pub(crate) static TRANSPARENT: PipelineConfig = PipelineConfig {
    pipeline_type: PipelineType::Transparent,
    topology_type: PrimitiveTopologyType::TriangleList,
    vertex_shader_path: "./src/shaders/default_vert.spv",
    fragment_shader_path: "./src/shaders/transparent_frag.spv",
    face_cull_mode: FaceCullMode::Back,
    blend_enabled: true
};