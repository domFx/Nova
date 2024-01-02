use std::ffi::CStr;
use ash::vk::KhrSwapchainFn;

pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub const ENGINE_NAME: &'static str = "Nova Engine";
pub const WINDOW_TITLE: &'static str = "Dom's Game";
pub const API_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 92);
pub const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub const PORTABILITY_MACOS_VERSION: u32 = ash::vk::make_api_version(1, 3, 216, 0);
pub const ENABLE_VALIDATION_LAYERS: bool = true;
pub const VALIDATION_LAYERS: &'static [&'static str] = &[
    "VK_LAYER_KHRONOS_validation"
];
pub const DEVICE_EXTENSIONS: &[&CStr] = &[
    KhrSwapchainFn::name()
];