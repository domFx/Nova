use ash::extensions::khr::Surface;
use ash::extensions::ext::DebugUtils;

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;

#[cfg(target_os = "macos")]
use ash::extensions::khr::MacOsSurface;

#[cfg(all(unix, not(target_os = "macos"), not(target_os="android")))]
use ash::extensions::khr::XlibSurface;

pub fn required_extension_names() -> Vec<*const i8> {
    #[cfg(target_os = "windows")]
    return vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr()
    ];

    #[cfg(target_os = "macos")]
    return vec![
        Surface::name().as_ptr(),
        MacOsSurface::name().as_ptr(),
        DebugUtils::name().as_ptr()
    ];

    #[cfg(all(unix, not(target_os = "macos"), not(target_os="android")))]
    return vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr()
    ];
}