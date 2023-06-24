// #[cfg(target_os = "windows")]
// #[path = "./win32_window.rs"]
// mod win32_window;

// #[cfg(not(target_os= "windows"))]
// #[cfg(target_os= "windows")]

#[path = "./winit_window.rs"]
mod winit_window;

pub struct NovaEngine {

}

impl NovaEngine {
    pub fn new() {
        #[cfg(target_os = "windows")]
        {


            // win32_window::open_win32_window();
            winit_window::open_winit_window();
        }

        #[cfg(not(target_os = "windows"))] 
        {
            winit_window::open_winit_window();
        }
    }
}