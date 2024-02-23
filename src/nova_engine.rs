use crate::core::window::get_window;
use crate::core::window::NovaWindow;
use crate::renderer::vulkan::vulkan_app::VulkanApp;

pub struct NovaEngine {
    window: dyn NovaWindow,
}

impl NovaEngine {
    pub fn new() {
        let mut window = get_window();

        let vulkan_app = &mut VulkanApp::new(
            window.get_window_handle(),
            window.get_raw_display_handle(),
            window.get_window_dimensions()
        ).unwrap();

        window.run(vulkan_app);
        window.destroy();
    }
}