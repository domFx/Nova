use log::debug;
use nova_core::event::SystemEvents;
use nova_math::vector2::Vector2;
use nova_renderer::renderer::{Renderer, RendererType};
use nova_window::window::{NovaWindow};
use nova_window::winit_window::WinitWindow;

use anyhow::Result;

enum NovaEngineState {
    Initialization,
    Running,
    Cleanup,
    Complete,
    Error
}

pub struct NovaEngine<'a> {
    window: Option<Box<WinitWindow<'a>>>,
    renderer: Box<Renderer>,
    system_event_handler: SystemEvents,
    state: NovaEngineState,
}

impl<'a> NovaEngine<'a> {
    pub fn new() -> Self {
        let system_event_handler = SystemEvents::default();
        let renderer = Box::new(Renderer::default());

        Self {
            window: None,
            renderer,
            system_event_handler,
            state: NovaEngineState::Initialization
        }
    }

    pub fn use_pretty_env_logger(self) -> NovaEngine<'a> {
        pretty_env_logger::init();
        self
    }

    pub fn set_log_level(self, log_level: &str) -> NovaEngine<'a> {
        std::env::set_var("RUST_LOG", log_level);
        self
    }

    pub fn run(&'a mut self) -> Result<()> {
        self.state = NovaEngineState::Running;

        let nova_window =
            Box::new(WinitWindow::new("Nova Engine", &mut self.renderer)?);

        self.window = Some(nova_window);
        
        Ok(())
    }

    pub fn destroy(&mut self) {
        self.state = NovaEngineState::Cleanup;

        if let Some(ref mut window) = self.window {
            window.destroy();
            self.window = None;
        }

        self.renderer.destroy();

        self.state = NovaEngineState::Complete;
    }
}

// impl<'a> Drop for NovaEngine<'a> {
//     fn drop(&mut self) {
//         self.state = NovaEngineState::Cleanup;
// 
//         if let Some(ref mut window) = self.window {
//             window.destroy();
//             self.window = None;
//         }
// 
//         self.renderer.destroy();
// 
//         self.state = NovaEngineState::Complete;
//     }
// }