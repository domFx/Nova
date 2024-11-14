use log::debug;
use nova_core::event::SystemEvents;
use nova_renderer::renderer::{Renderer, RendererType};
use nova_window::window::{NovaWindow, get_window};

pub struct NovaEngine {
    window: Box<dyn NovaWindow>,
    renderer: Option<Renderer>,
    system_event_handler: SystemEvents
}

impl NovaEngine {
    pub fn new() -> Self {
        pretty_env_logger::init();
        std::env::set_var("RUST_LOG", "trace");

        let window = get_window();

        let system_event_handler = SystemEvents::default();

        Self {
            window: Box::new(window),
            renderer: None,
            system_event_handler
        }
    }

    pub fn use_pretty_env_logger(&mut self) -> &mut NovaEngine {
        // pretty_env_logger::init();
        self
    }

    pub fn set_log_level(&mut self, log_level: &str) -> &mut NovaEngine {
        // std::env::set_var("RUST_LOG", log_level);
        self
    }

    pub fn run(&mut self) {
        // let mut renderer = Renderer::new(
        //     RendererType::Vulkan,
        //     self.window.get_window_handle(),
        //     self.window.get_raw_display_handle(),
        //     self.window.get_window_dimensions()
        // ).expect("Expected valid renderer");
        //
        // while self.window.is_running() {
        //     println!("running");
        //     if self.window.redraw() {
        //         println!("redraw");
        //         // renderer.render(self.window.get_window_dimensions()).unwrap()
        //     }
        // }
        //
        // println!("end run")
    }
}

impl Drop for NovaEngine {
    fn drop(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.destroy();
        }

        self.window.destroy();
    }
}