use ash::vk;

#[cfg(target_os = "macos")]
use ash::mvk;

#[cfg(all(unix, not(target_os = "macos"), not(target_os="android")))]
use ash::extensions::khr::XlibSurface;

pub(crate) fn get_required_platform_extensions() -> Vec<*const i8> {
    #[cfg(target_os = "windows")]
    return vec![
        vk::KHR_SURFACE_NAME.as_ptr(),
        vk::KHR_WIN32_SURFACE_NAME.as_ptr(),
    ];

    #[cfg(target_os = "macos")]
    return vec![
        mvk::macos_surface::NAME.as_ptr(),
        vk::KHR_SURFACE_NAME.as_ptr()
    ];

    #[cfg(all(unix, not(target_os = "macos"), not(target_os="android")))]
    return vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr()
    ];
}