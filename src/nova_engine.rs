use crate::core::window::get_window;
use crate::core::window::NovaWindow;
use crate::vulkan_app::VulkanApp;

pub struct NovaEngine {

}

impl NovaEngine {
    pub fn new() {
        let mut window = get_window();

        let vulkan_app = &mut VulkanApp::new(
            window.get_window_handle().unwrap(),
            window.get_raw_display_handle().unwrap(),
            window.get_window_dimensions()
        ).unwrap();

        window.run(vulkan_app);
        window.destroy();
    }
}