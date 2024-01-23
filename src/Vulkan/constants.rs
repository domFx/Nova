use std::ffi::CStr;
use ash::vk::KhrSwapchainFn;

pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub const ENGINE_NAME: &'static str = "Nova Engine";
pub const WINDOW_TITLE: &'static str = "Dom's Game";
pub const API_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub const PORTABILITY_MACOS_VERSION: u32 = ash::vk::make_api_version(0, 1, 3, 216);

pub const VALIDATION_LAYERS: &[&CStr] = &[
    unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") }
];

pub const DEVICE_EXTENSIONS: &[&CStr] = &[
    KhrSwapchainFn::name()
];

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;