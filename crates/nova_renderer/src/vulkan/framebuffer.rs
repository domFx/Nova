use ash::vk;
use anyhow::Result;

pub(crate) fn create_framebuffers(
    device: &ash::Device,
    image_views: &Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    extent: &vk::Extent2D
) -> Result<Vec<vk::Framebuffer>> {
    unsafe {
        let framebuffers =
            image_views
            .iter()
            .map(|i| {
                let attachments = &[*i];
                let create_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(render_pass)
                    .attachments(attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1)
                    .build();

                device.create_framebuffer(&create_info, None)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(framebuffers)
    }
}