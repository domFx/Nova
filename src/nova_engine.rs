use log::error;
use nova_renderer::window::{NovaWindow, get_window};
use nova_renderer::renderer::{Renderer, RendererType};

pub struct NovaEngine {
    window: dyn NovaWindow,
}

impl NovaEngine {
    pub fn new() {
        let mut window = get_window();

        let renderer_result = &mut Renderer::new(
            RendererType::Vulkan,
            window.get_window_handle(),
            window.get_raw_display_handle(),
            window.get_window_dimensions()
        );

        match renderer_result {
            Ok(renderer) => {
                window.run(renderer);
                renderer.destroy();
                window.destroy();
            },
            Err(e) => error!("Error creating renderer {e:?}")
        }
    }
}