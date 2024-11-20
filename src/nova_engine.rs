use log::debug;
use nova_core::event::SystemEvents;
use nova_math::vector2::Vector2;
use nova_renderer::renderer::{Renderer, RendererType};
use nova_window::window::{NovaWindow, get_nova_window};

enum NovaEngineState {
    Initialization,
    Running,
    Cleanup,
    Complete,
    Error
}

pub struct NovaEngine {
    window: Option<Box<dyn NovaWindow>>,
    renderer: Option<Renderer>,
    system_event_handler: SystemEvents,
    state: NovaEngineState,
}

impl NovaEngine {
    pub fn new() -> Self {
        let system_event_handler = SystemEvents::default();

        Self {
            window: None,
            renderer: None,
            system_event_handler,
            state: NovaEngineState::Initialization
        }
    }

    pub fn use_pretty_env_logger(&mut self) -> &mut NovaEngine {
        pretty_env_logger::init();
        self
    }

    pub fn set_log_level(&mut self, log_level: &str) -> &mut NovaEngine {
        std::env::set_var("RUST_LOG", log_level);
        self
    }

    pub fn run(&mut self)  {
        let mut renderer = Renderer::default();

        self.state = NovaEngineState::Running;
        self.window = Some(get_nova_window("Nova Engine"));
        // let mut renderer = Renderer::new(
        //     RendererType::Vulkan,
        //     self.window.get_window_handle(),
        //     self.window.get_raw_display_handle(),
        //     self.window.get_window_dimensions()
        // ).expect("Expected valid renderer");
    }
}

impl Drop for NovaEngine {
    fn drop(&mut self) {
        self.state = NovaEngineState::Cleanup;
        if let Some(ref mut renderer) = self.renderer {
            renderer.destroy();
            self.renderer = None;
        }

        if let Some(ref mut window) = self.window {
            window.destroy();
            self.window = None;
        }
        self.state = NovaEngineState::Complete;
    }
}