use crate::vulkan::queue_families::*;
use nova_math::vector::vector2::Vector2;

use ash::extensions::khr::{Surface, Swapchain};
use ash::{Device, Instance, vk};
use ash::vk::{Image, PhysicalDevice, SurfaceKHR, SwapchainKHR};

pub struct VulkanAppSwapchain {
    pub swapchain: SwapchainKHR,
    pub loader: Swapchain,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl VulkanAppSwapchain {
    pub unsafe fn get(
        window_dimensions: Vector2,
        instance: &Instance,
        logical_device: &Device,
        physical_device: PhysicalDevice,
        surface_loader: &Surface,
        surface: SurfaceKHR,
        indices: &QueueFamilyIndices) -> anyhow::Result<Self> {

        let swapchain_loader = Swapchain::new(instance, logical_device);
        let swapchain_support = SwapchainSupport::get(&surface_loader, physical_device, surface)?;

        let surface_format = get_swapchain_surface_format(&swapchain_support.formats);
        let present_mode = get_swapchain_present_mode(&swapchain_support.present_modes);
        let extent = get_swapchain_extent(window_dimensions, swapchain_support.capabilities);

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
            .old_swapchain(SwapchainKHR::null())
            .build();

        let swapchain = swapchain_loader.create_swapchain(&info, None)?;

        Ok(Self {
            swapchain,
            loader: swapchain_loader,
            format: surface_format.format,
            extent
        })
    }
}

pub unsafe fn create_swapchain_image_views(
    device: &Device,
    images: &Vec<Image>,
    format: vk::Format,
) -> anyhow::Result<Vec<vk::ImageView>> {
    let image_views = images
        .iter()
        .map(|i| {
            let components = vk::ComponentMapping::builder()
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

            let info = vk::ImageViewCreateInfo::builder()
                .image(*i)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .components(components)
                .subresource_range(subresource_range)
                .build();

            device.create_image_view(&info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(image_views)
}

fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    formats
        .iter()
        .cloned()
        .find(|f| f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
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
    window_dimensions: Vector2,
    capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    } else {
        let clamp = |min: u32, max: u32, v: u32| min.max(max.min(v));
        vk::Extent2D::builder()
            .width(clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
                window_dimensions.x as u32,
            ))
            .height(clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
                window_dimensions.y as u32,
            ))
            .build()
    }
}

// Represents capabilities of GPU
#[derive(Clone, Debug)]
pub struct SwapchainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>
}

impl SwapchainSupport {
    pub unsafe fn get(
        surface_loader: &Surface,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR
    ) -> anyhow::Result<Self> {
        Ok(Self {
            capabilities: surface_loader.get_physical_device_surface_capabilities(physical_device, surface)?,
            formats: surface_loader.get_physical_device_surface_formats(physical_device, surface)?,
            present_modes: surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?
        })
    }
}