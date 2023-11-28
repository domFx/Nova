#[path = "./vulkan_app.rs"]
mod vulkan_app;

use vulkan_app::VulkanApp;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

pub fn open_winit_window() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut vulkan_app = VulkanApp::new(&window).unwrap();

    event_loop.run(move |event_loop_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event_loop_event {
            Event::WindowEvent { event, ..} => {
                match event {
                    WindowEvent::CloseRequested =>  { 
                        *control_flow = ControlFlow::Exit;
                        vulkan_app.destroy();
                    },
                    _ => () 
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_window_id) => (),
            _ => (),
        }
    });
}