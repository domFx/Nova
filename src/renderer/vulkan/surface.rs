use std::ffi::c_void;
use anyhow::anyhow;
use ash::{Entry, Instance, vk};
use ash::extensions::mvk::*;
use ash::extensions::khr::Win32Surface;
use ash::extensions::mvk;
use ash::vk::{MacOSSurfaceCreateFlagsMVK, MacOSSurfaceCreateInfoMVK};
use log::error;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use winit::window::Window;
use crate::core::window::NovaWindow;

pub unsafe fn create_surface(
    raw_window_handle: RawWindowHandle,
    raw_display_handle: RawDisplayHandle,
    entry: &Entry,
    instance: &Instance
) -> anyhow::Result<vk::SurfaceKHR> {
    match (raw_display_handle, raw_window_handle) {
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(win32_window_handle)) => {
            let hinstance_ptr: *const c_void = match win32_window_handle.hinstance {
                Some(hinstance) => hinstance.get() as *const c_void,
                None => std::ptr::null(),
            };

            let hwnd_ptr: *const c_void = win32_window_handle.hwnd.get() as *const c_void;

            let info = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(hinstance_ptr)
                .hwnd(hwnd_ptr)
                .build();

            let win32_surface = Win32Surface::new(entry, instance);

            let surface = win32_surface.create_win32_surface(&info, None).unwrap();

            Ok(surface)
        },
        (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(ui_kit_handle)) => {
            let macos_surface = MacOSSurface::new(entry, instance);

            let info = MacOSSurfaceCreateInfoMVK::builder()
                .build();

            let surface = macos_surface.create_mac_os_surface(&info, None).unwrap();

            Ok(surface)
        }
        _ => {
            error!("Raw window handle not implemented");
            Err(anyhow!("Missing valid physical device."))
        }
    }
}