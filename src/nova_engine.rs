use log::error;
use crate::core::window::get_window;
use crate::core::window::NovaWindow;
use crate::renderer::vulkan::vulkan_app::VulkanApp;

pub struct NovaEngine {
    window: dyn NovaWindow,
}

impl NovaEngine {
    pub fn new() {
        let mut window = get_window();

        let vulkan_app_result = &mut VulkanApp::new(
            window.get_window_handle(),
            window.get_raw_display_handle(),
            window.get_window_dimensions()
        );

        match vulkan_app_result {
            Ok(vulkan_app) => {
                window.run(vulkan_app);
                window.destroy();
            },
            Err(e) => error!("Error creating VulkanApp {e:?}")
        }
    }
}