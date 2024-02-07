use crate::vulkan_app::VulkanApp;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop},
    window::WindowBuilder,
};
use winit::event_loop::ControlFlow;

pub fn open_winit_window() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut vulkan_app = VulkanApp::new(&window).unwrap();

    let mut last_window_size = None;
    let mut destroying = false;
    let mut minimized = false;

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move |event_loop_event, elwt| {
        match event_loop_event {
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        destroying = true;
                        elwt.exit();
                        unsafe { vulkan_app.destroy() };
                    },
                    WindowEvent::RedrawRequested => {
                        if !destroying && !minimized {
                            unsafe { vulkan_app.render(&window).unwrap() };
                        }
                    },
                    WindowEvent::Resized(size) => {
                        match last_window_size {
                            None => last_window_size = Some(size),
                            Some(x) => {
                                if size.width == 0 || size.height == 0 {
                                    minimized = true;
                                } else {
                                    minimized = false;
                                    vulkan_app.resized = true;
                                }
                                last_window_size = Some(x);
                            }
                        }
                    }
                    _ => ()
                }
            },
            _ => (),
        }
    }).expect("TODO: panic message");
}