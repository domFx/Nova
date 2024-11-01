use std::ffi::CStr;
use ash::vk::KhrSwapchainFn;

pub(crate) const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub(crate) const ENGINE_NAME: &'static str = "Nova Engine";
pub(crate) const WINDOW_TITLE: &'static str = "Dom's Game";
pub(crate) const API_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub(crate) const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub(crate) const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub(crate) const PORTABILITY_MACOS_VERSION: u32 = ash::vk::make_api_version(0, 1, 3, 216);

pub(crate) const VALIDATION_LAYERS: &[&CStr] = &[
    unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") }
];

pub(crate) const DEVICE_EXTENSIONS: &[&CStr] = &[
    KhrSwapchainFn::name()
];

pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 2;