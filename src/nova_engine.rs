use log::{debug, error};
use nova_core::event::SystemEvents;
use nova_math::vector2::Vector2;
use nova_renderer::renderer::{Renderer, RendererType};
use nova_window::window::{NovaWindow};
use nova_window::winit_window::WinitWindow;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
enum NovaEngineState {
    Initialization,
    Running,
    Cleanup,
    Complete
}

pub struct NovaEngine {
    renderer: Box<Renderer>,
    system_event_handler: SystemEvents,
    state: NovaEngineState,
}

impl NovaEngine {
    pub fn new() -> Self {
        let system_event_handler = SystemEvents::default();
        let renderer = Box::new(Renderer::default());

        Self {
            renderer,
            system_event_handler,
            state: NovaEngineState::Initialization
        }
    }

    pub fn use_pretty_env_logger(self) -> NovaEngine {
        pretty_env_logger::init();
        self
    }

    pub fn set_log_level(self, log_level: &str) -> NovaEngine {
        std::env::set_var("RUST_LOG", log_level);
        self
    }

    pub fn run(&mut self) -> Result<&mut NovaEngine> {
        if self.state != NovaEngineState::Initialization {
            return Err(anyhow!("NovaEngine state is not Initialization"));
        }

        self.state = NovaEngineState::Running;

        // Start window loop
        {
            let renderer = &mut self.renderer;

            let nova_window =
                Box::new(WinitWindow::new("Nova Engine", renderer)?);

            nova_window.destroy();
        }

        debug!("NovaEngine clean up.");
        self.state = NovaEngineState::Cleanup;
        
        self.renderer.destroy();

        self.state = NovaEngineState::Complete;

        Ok(self)
    }
}