use std::ffi::c_void;
use anyhow::anyhow;
use ash::{Entry, Instance, vk};
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
            use ash::extensions::mvk::*;
            use ash::vk::{MacOSSurfaceCreateFlagsMVK, MacOSSurfaceCreateInfoMVK, PFN_vkCreateMetalSurfaceEXT};
            use cocoa::appkit::{NSView, NSWindow};
            use cocoa::{base::id as cocoa_id};
            use cocoa::appkit::NSEvent;
            use core_graphics_types::{base::CGFloat, geometry::CGRect};
            use metal::MetalLayer;
            use objc::runtime::{YES,BOOL,Object};
            use objc::*;
            use objc::msg_send;
            use objc2::rc::Id;
            use std::mem;
            use winit::platform::macos::WindowExtMacOS;
            use winit::platform::macos;

            // let ns_view = window.ns_view as cocoa_id;

            // SAFETY: The pointer came from `WindowHandle`, which ensures
            // that the `AppKitWindowHandle` contains a valid pointer to an
            // `NSView`.
            // Unwrap is fine, since the pointer came from `NonNull`.
            // let ns_view: Id<NSView> = unsafe { Id::retain(ns_view.cast()) }.unwrap();
            // Do something with the NSView here, like getting the `NSWindow`

            // let ns_view = Id::<NSView>::new(ns_view_ptr).unwrap();
            let view = window.ns_view.as_ptr() as cocoa_id;
            let win: cocoa_id = msg_send![view, window];
            if !win.is_null() {
                let ns_window = win.window();

                // TODO: this logic is duplicated from gfx-backend-metal, refactor?
                unsafe {
                    let existing: *mut Object = msg_send![view, layer];
                    let class = class!(CAMetalLayer);

                    let use_current = if existing.is_null() {
                        false
                    } else {
                        let result: BOOL = msg_send![existing, isKindOfClass: class];
                        result == YES
                    };

                    if !use_current {
                        let layer: *mut Object = msg_send![class, new];
                        let () = msg_send![view, setLayer: layer];
                        let bounds: CGRect = msg_send![view, bounds];
                        let () = msg_send![layer, setBounds: bounds];

                        let window: *mut Object = msg_send![view, window];
                        if !window.is_null() {
                            let scale_factor: CGFloat = msg_send![window, backingScaleFactor];
                            let () = msg_send![layer, setContentsScale: scale_factor];
                        }
                    }
                }
                // let wnd: *mut objc::runtime::Object = mem::transmute(ns_window);

                // let ns_window = ns_view.window().expect("view was not installed in a window");

                // let layer = MetalLayer::new();
                // layer.set_edge_antialiasing_mask(0);
                // layer.set_presents_with_transaction(false);
                // layer.remove_all_animations();
                //
                // let view = ns_window.contentView();
                //
                // layer.set_contents_scale(win.backingScaleFactor());
                // win.setLayer(mem::transmute(layer.as_ref()));
                // win.setWantsLayer(YES);

                let info = vk::MacOSSurfaceCreateInfoMVK::builder()
                    .flags(vk::MacOSSurfaceCreateFlagsMVK::empty())
                    .view(window.ns_view.as_ptr())
                    .build();

                let metal_surface = MacOSSurface::new(entry, instance);

                let surface = metal_surface.create_mac_os_surface(&info, None)?;
                return Ok(surface);
            }

            Err(anyhow!("Unable to create macos window"))
        },
        _ => {
            error!("Raw window handle not implemented");
            Err(anyhow!("Missing valid physical device."))
        }
    }
}