use std::ffi::c_void;
use anyhow::anyhow;
use ash::{Entry, Instance, vk};
use log::error;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub unsafe fn create_surface(
    raw_window_handle: RawWindowHandle,
    raw_display_handle: RawDisplayHandle,
    entry: &Entry,
    instance: &Instance
) -> anyhow::Result<vk::SurfaceKHR> {
    match (raw_display_handle, raw_window_handle) {
        #[cfg(target_os = "windows")]
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(win32_window_handle)) => {
            use ash::extensions::khr::Win32Surface;

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
        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
            use ash::extensions::ext::MetalSurface;
            use cocoa::appkit::{NSView};
            use cocoa::{base::id as cocoa_id};
            use metal::{MetalLayer, MetalLayerRef};
            use objc::runtime::{YES};

            let ns_view_ptr = window.ns_view.as_ptr();

            let mut layer = MetalLayer::new();
            layer.set_opaque(false);
            layer.set_presents_with_transaction(false);

            (ns_view_ptr as cocoa_id).setWantsLayer(YES);
            (ns_view_ptr as cocoa_id).setLayer(layer.as_ref() as *const _ as _);

            let layer_ptr = (layer.as_ref() as *const MetalLayerRef).cast::<c_void>();

            let info = vk::MetalSurfaceCreateInfoEXT::builder()
                .flags(vk::MetalSurfaceCreateFlagsEXT::empty())
                .layer(layer_ptr)
                .build();

            let metal_surface = MetalSurface::new(entry, instance);

            let surface = metal_surface.create_metal_surface(&info, None)?;
            Ok(surface)
        },
        _ => {
            error!("Raw window handle not implemented");
            Err(anyhow!("Missing valid physical device."))
        }
    }
}