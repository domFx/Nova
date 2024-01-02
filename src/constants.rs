pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub const ENGINE_NAME: &'static str = "Nova Engine";
pub const WINDOW_TITLE: &'static str = "Dom's Game";
pub const API_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 92);
pub const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
pub const PORTABILITY_MACOS_VERSION: u32 = ash::vk::make_api_version(1, 3, 216, 0);