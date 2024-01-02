use crate::constants::*;
use crate::platform::*;
use crate::utils::*;

use std::ffi::{CStr, CString, c_void};
use std::collections::HashSet;

use anyhow::{Result,anyhow};
use ash::{vk, Entry, Instance, Device};
use ash::extensions::khr::{ Win32Surface, Surface, Swapchain };
use ash::vk::{KhrSwapchainFn, SwapchainKHR, SurfaceKHR, PhysicalDevice, Queue, KhrPortabilitySubsetFn, ComponentMapping, Image};
use log::*;
use thiserror::Error;
use winit::window::{Window};
use raw_window_handle::{RawWindowHandle, HasWindowHandle};

const ENABLE_VALIDATION_LAYERS: bool = true;
const VALIDATION_LAYERS: &'static [&'static str] = &[
    "VK_LAYER_KHRONOS_validation"
];

const DEVICE_EXTENSIONS: &[&CStr] = &[
    KhrSwapchainFn::name()
];

pub struct VulkanApp {
    entry: Entry, 
    instance: Instance,
    surface_loader: Surface,
    logical_device: Device,
    physical_device: PhysicalDevice,
    graphics_queue: Queue,
    surface: SurfaceKHR,
    swapchain: VulkanAppSwapchain,
}

impl VulkanApp {
    pub fn new(window: &Window) -> Result<Self> {
        let entry = unsafe { Entry::load()? };
        let instance = create_instance(&entry)?;
        let surface_loader = Surface::new(&entry, &instance);
        let surface = unsafe { create_surface(window, &entry, &instance)? };
        let physical_device = unsafe { pick_physical_device(&instance, &surface_loader, surface )? };
        let indices = unsafe { QueueFamilyIndices::get(&instance, physical_device)? };
        let logical_device = unsafe { create_logical_device(&entry, &instance, physical_device, &indices)? };
        let graphics_queue = unsafe { logical_device.get_device_queue(indices.graphics, 0) };

        let swapchain = unsafe { VulkanAppSwapchain::get(&window, &instance, &logical_device, physical_device, &surface_loader, surface)? };

        Ok(Self {
            entry,
            instance,
            surface_loader,
            physical_device,
            logical_device,
            graphics_queue,
            surface,
            swapchain,
        })
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.swapchain.image_views
                .iter()
                .for_each(|v| self.logical_device.destroy_image_view(*v, None));

            self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);

            self.logical_device.destroy_device(None);

            // self.surface.destroy_surface_khr --> probably a cleaner way to do this
            self.surface_loader.destroy_surface(self.surface, None);

            self.instance.destroy_instance(None);
        }
    }
}

unsafe fn create_surface(window: &Window, entry: &Entry, instance: &Instance) -> Result<vk::SurfaceKHR> {
    let window_handle = window.window_handle()?;
    let raw_window_handle = window_handle.as_ref();

    match raw_window_handle {
         RawWindowHandle::Win32(win32_window_handle) => {
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
        _ => {
            error!("Raw window handle not implemented");
            Err(anyhow!("Missing valid physical device."))
        }
    }
}

unsafe fn pick_physical_device(instance: &Instance, surface_loader: &Surface, surface: vk::SurfaceKHR) -> Result<vk::PhysicalDevice> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        let device_name = convert_vec_to_string(&properties.device_name.to_vec());

        if let Err(error) = check_physical_device(instance, surface_loader, surface, physical_device) {
            warn!("Skipping physical device (`{:?}`): {}", device_name, error);
        } else {
            info!("Selected physical device (`{:?}`).", device_name);
            return Ok(physical_device)
        }
    }

    Err(anyhow!(SuitabilityError("Missing valid physical device.")))
}

unsafe fn check_physical_device(instance: &Instance, surface_loader: &Surface, surface: SurfaceKHR, physical_device: PhysicalDevice) -> Result<()> {
    QueueFamilyIndices::get(instance, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(surface_loader, physical_device, surface)?;

    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")))
    }

    return Ok(())
}

unsafe fn check_physical_device_extensions(instance: &Instance, physical_device: PhysicalDevice) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device)?
        .iter()
        .map(|e| unsafe { CStr::from_ptr(e.extension_name.as_ptr()) })
        .collect::<HashSet<_>>();

    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError("Missing required device extensions")))
    }
}

#[derive(Debug, Error)]
#[error("{0}.")]
pub struct SuitabilityError(pub &'static str);

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
    present: u32
}

impl QueueFamilyIndices {
    unsafe fn get(
        instance: &Instance,
        device: PhysicalDevice
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let present = 0;

        if let Some(graphics) = graphics {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families")))
        }
    }
}

// Represents capabilities of GPU
#[derive(Clone, Debug)]
struct SwapchainSupport {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>
}

impl SwapchainSupport {
    unsafe fn get(
        surface_loader: &Surface,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR
    ) -> Result<Self> {
        Ok(Self {
            capabilities: surface_loader.get_physical_device_surface_capabilities(physical_device, surface)?,
            formats: surface_loader.get_physical_device_surface_formats(physical_device, surface)?,
            present_modes: surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?
        })
    }
}

struct VulkanAppSwapchain {
    swapchain: SwapchainKHR,
    images: Vec<Image>,
    loader: Swapchain,
    format: vk::Format,
    extent: vk::Extent2D,
    image_views: Vec<vk::ImageView>
}

impl VulkanAppSwapchain {
    pub unsafe fn get(
        window: &Window,
        instance: &Instance,
        logical_device: &Device,
        physical_device: PhysicalDevice,
        surface_loader: &Surface,
        surface: SurfaceKHR) -> Result<Self> {

        let swapchain_loader = Swapchain::new(instance, logical_device);

        let indices = QueueFamilyIndices::get(instance, physical_device)?;
        let swapchain_support = SwapchainSupport::get(&surface_loader, physical_device, surface)?;

        let surface_format = get_swapchain_surface_format(&swapchain_support.formats);
        let present_mode = get_swapchain_present_mode(&swapchain_support.present_modes);
        let extent = get_swapchain_extent(window, swapchain_support.capabilities);

        let mut image_count = swapchain_support.capabilities.min_image_count + 1;
        if swapchain_support.capabilities.max_image_count != 0 && image_count > swapchain_support.capabilities.max_image_count {
            image_count = swapchain_support.capabilities.max_image_count;
        }

        let mut queue_family_indices = vec![];
        let image_sharing_mode = if indices.graphics != indices.present {
            queue_family_indices.push(indices.graphics);
            queue_family_indices.push(indices.present);
            vk::SharingMode::CONCURRENT
        } else {
            vk::SharingMode::EXCLUSIVE
        };

        let info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(swapchain_support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(SwapchainKHR::null());

        let swapchain = swapchain_loader.create_swapchain(&info, None)?;

        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };

        let image_views = create_swapchain_image_views(logical_device, &swapchain_images, surface_format.format)?;

        Ok(Self {
            swapchain,
            images: swapchain_images,
            loader: swapchain_loader,
            format: surface_format.format,
            extent,
            image_views
        })
    }
}

unsafe fn create_swapchain_image_views(
    device: &Device,
    swapchain_images: &Vec<Image>,
    format: vk::Format
) -> Result<Vec<vk::ImageView>> {
    let components = ComponentMapping::builder()
        .r(vk::ComponentSwizzle::IDENTITY)
        .g(vk::ComponentSwizzle::IDENTITY)
        .b(vk::ComponentSwizzle::IDENTITY)
        .a(vk::ComponentSwizzle::IDENTITY)
        .build();

    let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let image_views = swapchain_images
        .iter()
        .map(|i| {
            let info = vk::ImageViewCreateInfo::builder()
                .image(*i)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .components(components)
                .subresource_range(subresource_range);

            device.create_image_view(&info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(image_views)
}

fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    formats
        .iter()
        .cloned()
        .find(|f| {
            f.format == vk::Format::B8G8R8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or_else(|| formats[0])
}

fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    present_modes
        .iter()
        .cloned()
        .find(|m| *m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

// Get resolution of the swapchain images
fn get_swapchain_extent(
    window: &Window,
    capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    } else {
        let size = window.inner_size();
        let clamp = |min: u32, max: u32, v: u32| min.max(max.min(v));
        vk::Extent2D::builder()
            .width(clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
                size.width
            ))
            .height(clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
                size.height
            ))
            .build()
    }
}

unsafe fn create_logical_device(entry: &Entry, instance: &Instance, physical_device: PhysicalDevice, indices: &QueueFamilyIndices) -> Result<Device> {
    let queue_priorities = &[1.0];
    let queue_info = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(indices.graphics)
        .queue_priorities(queue_priorities)
        .build();

    let mut extensions: Vec<*const i8> = DEVICE_EXTENSIONS
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    if cfg!(target_os = "macos") && entry.try_enumerate_instance_version()?.unwrap() >= PORTABILITY_MACOS_VERSION {
        let portability = KhrPortabilitySubsetFn::name();
        extensions.push(portability.as_ptr());
    }

    let layers: Vec<*const i8> = if ENABLE_VALIDATION_LAYERS {
        VALIDATION_LAYERS
            .iter()
            .map(|v| CString::new(*v).unwrap())
            .map(|v| v.as_ptr())
            .collect()
    } else  {
        vec![]
    };

    let features = vk::PhysicalDeviceFeatures::builder();

    let queue_infos = &[queue_info];
    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(physical_device, &info, None)?;

    Ok(device)
}

fn create_instance(entry: &Entry) -> Result<Instance> {
    let app_name = CString::new(WINDOW_TITLE).unwrap();
    let engine_name = CString::new(ENGINE_NAME).unwrap();

    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: std::ptr::null(),
        application_version: APPLICATION_VERSION,
        p_engine_name: engine_name.as_ptr(),
        engine_version: ENGINE_VERSION,
        api_version: API_VERSION
    };

    let debug_info = create_debug_info();

    let required_extension_names = required_extension_names();

    let required_validation_layer_raw_names: Vec<CString> = VALIDATION_LAYERS
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();

    let layer_names: Vec<*const i8>= required_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    let create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: if VALIDATION_ENABLED {
            &debug_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void
        } else {
            std::ptr::null()
        },
        flags: vk::InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        pp_enabled_layer_names: if VALIDATION_ENABLED {
            layer_names.as_ptr()
        } else {
            std::ptr::null()
        },
        enabled_layer_count: if VALIDATION_ENABLED {
            layer_names.len()
        } else {
            0
        } as u32,
        pp_enabled_extension_names: required_extension_names.as_ptr(),
        enabled_extension_count: required_extension_names.len() as u32
    };

    let instance: Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create Instance")
    };

    Ok(instance)
}

fn create_debug_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: std::ptr::null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity:
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            // vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        message_type:
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE ,
            // vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
        pfn_user_callback: Some(vulkan_debug_callback),
        p_user_data: std::ptr::null_mut()
    }
}

extern "system" fn vulkan_debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.p_message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        info!("({:?}) {}", type_, message);
    } else {
        debug!("({:?}) {}", type_, message);
    }

    vk::FALSE
}