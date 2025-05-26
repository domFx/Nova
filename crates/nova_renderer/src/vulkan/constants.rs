use std::ffi::CStr;
use ash::vk;

/// Game Engine Name
pub(crate) const ENGINE_NAME: &'static str = "Nova Engine";
/// Default Window Nam
pub(crate) const WINDOW_TITLE: &'static str = "Dom's Game";

/// Minimum required Vulkan API version
pub(crate) const MIN_API_VERSION: u32 = ash::vk::make_api_version(0, 1, 2, 0);
/// Engine Version 
pub(crate) const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
/// Application Version
pub(crate) const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
/// Minimum required Vulkan API version for MacOS
pub(crate) const PORTABILITY_MACOS_VERSION: u32 = ash::vk::make_api_version(0, 1, 3, 216);

/// Vulkan Validation Layers enabled 
pub(crate) const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
/// Validation Layers to enable
pub(crate) const VALIDATION_LAYERS: &[&CStr] = &[
    unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") }
];

/// Device Extensions required for the Vulkan renderer
pub(crate) const DEVICE_EXTENSIONS: &[&CStr] = &[
    vk::KHR_SWAPCHAIN_NAME
];

/// Minimum number of frames in flight for the Vulkan renderer
pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 2;