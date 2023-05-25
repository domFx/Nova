#[path = "./win32_window.rs"]
mod win32_window;
#[path = "./winit_window.rs"]
mod winit_window;

pub struct NovaEngine {

}

impl NovaEngine {
    pub fn new() {
        if cfg!(windows) {
            win32_window::open_win32_window();
        } else {
            winit_window::open_winit_window();
        }
    }
}