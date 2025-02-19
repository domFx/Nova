use anyhow::anyhow;
use ash::{vk, Entry, Instance};
use log::error;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::vulkan::errors::VulkanConfigurationError;

pub unsafe fn create_surface(
    raw_window_handle: RawWindowHandle,
    raw_display_handle: RawDisplayHandle,
    entry: &Entry,
    instance: &Instance,
) -> anyhow::Result<vk::SurfaceKHR> {
    match (raw_display_handle, raw_window_handle) {
        #[cfg(target_os = "windows")]
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(win32_window_handle)) => {
            use ash::khr;

            let hinstance_ptr = match win32_window_handle.hinstance {
                Some(hinstance) => hinstance.get(),
                None => 0,
            };

            let hwnd = win32_window_handle.hwnd.get();

            let info = vk::Win32SurfaceCreateInfoKHR::default()
                .hinstance(hinstance_ptr)
                .hwnd(hwnd);

            let win32_surface = khr::win32_surface::Instance::new(entry, instance);

            let surface = win32_surface.create_win32_surface(&info, None)?;

            Ok(surface)
        }
        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
            use ash::ext::metal_surface;
            use cocoa::appkit::NSView;
            use cocoa::base::id as cocoa_id;
            use metal::{MetalLayer, MetalLayerRef};
            use objc::runtime::YES;
            use std::ffi::c_void;

            let ns_view_ptr = window.ns_view.as_ptr();

            let mut layer = MetalLayer::new();
            layer.set_opaque(false);
            layer.set_presents_with_transaction(false);

            (ns_view_ptr as cocoa_id).setWantsLayer(YES);
            (ns_view_ptr as cocoa_id).setLayer(layer.as_ref() as *const _ as _);

            let layer_ptr = (layer.as_ref() as *const MetalLayerRef).cast::<c_void>();

            let info = vk::MetalSurfaceCreateInfoEXT::default()
                .flags(vk::MetalSurfaceCreateFlagsEXT::empty())
                .layer(layer_ptr);

            let metal_surface = metal_surface::Instance::new(entry, instance);

            let surface = metal_surface.create_metal_surface(&info, None)?;
            Ok(surface)
        }
        _ => {
            Err(anyhow!(VulkanConfigurationError("Missing valid physical device.")))
        }
    }
}
