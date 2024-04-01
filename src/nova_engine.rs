use nova_core::event::SystemEvents;
use nova_renderer::window::{NovaWindow, get_window};
use nova_renderer::renderer::{Renderer, RendererType};

pub struct NovaEngine {
    window: Box<dyn NovaWindow>,
    renderer: Renderer,
    system_event_handler: SystemEvents
}

impl NovaEngine {
    pub fn new() -> Box<Self> {
        let window = get_window();

        let system_event_handler = SystemEvents::default();

        let renderer = Renderer::new(
            RendererType::Vulkan,
            window.get_window_handle(),
            window.get_raw_display_handle(),
            window.get_window_dimensions()
        ).expect("Expected valid renderer");

        Box::new(Self {
            window: Box::new(window),
            renderer,
            system_event_handler
        })
    }

    pub fn run(&mut self) {
        self.window.run(&mut self.renderer);
    }
}

impl Drop for NovaEngine {
    fn drop(&mut self) {
        self.renderer.destroy();
        self.window.destroy();
    }
}