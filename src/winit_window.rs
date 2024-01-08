use crate::vulkan_app::VulkanApp;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop},
    window::WindowBuilder,
};

pub fn open_winit_window() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut vulkan_app = VulkanApp::new(&window).unwrap();

    event_loop.run(move |event_loop_event, elwt| {
        match event_loop_event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                        vulkan_app.destroy();
                    },
                    WindowEvent::RedrawRequested => {
                        unsafe { vulkan_app.render(&window).unwrap() };
                    },
                    _ => ()
                }
            },
            _ => (),
        }
    }).expect("TODO: panic message");
}