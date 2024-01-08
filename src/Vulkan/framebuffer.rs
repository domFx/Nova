use ash::{Device, vk};
use ash::vk::{Extent2D, Framebuffer, ImageView, RenderPass};
use anyhow::Result;

pub unsafe fn create_framebuffers(device: &Device, image_views: &Vec<ImageView>, render_pass: RenderPass, extent: &Extent2D) -> Result<Vec<Framebuffer>> {
    let framebuffers = image_views
        .iter()
        .map(|i| {
            let attachments = &[*i];
            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);

            device.create_framebuffer(&create_info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(framebuffers)
}