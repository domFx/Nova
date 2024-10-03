use crate::window::NovaWindow;
use nova_core::event::{fire_event, EventContext};
use nova_core::input::{KeyboardKey, KeyboardKeyState, MouseButtonState, INPUT_STATE};
use nova_math::vector::vector2::Vector2;
use nova_renderer::renderer::Renderer;

use log::info;
use std::time::{Duration, Instant};

use raw_window_handle::{
    HasDisplayHandle, HasRawWindowHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WindowHandle,
};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    platform::run_on_demand::EventLoopExtRunOnDemand,
    window::WindowBuilder,
};

pub(crate) struct WinitWindow {
    event_loop: EventLoop<()>,
    window: winit::window::Window,

    last_window_size: Option<Vector2<u32>>,
    minimized: bool,
    destroying: bool,
}

impl WinitWindow {
    pub(crate) fn new() -> Self {
        let destroying = false;
        let minimized = false;

        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let inner_size = window.inner_size();
        let last_window_size = Some(Vector2::new(inner_size.width, inner_size.height));

        Self {
            event_loop,
            window,

            last_window_size,
            destroying,
            minimized,
        }
    }
}

impl NovaWindow for WinitWindow {
    fn run(&mut self, render: &mut Renderer) {
        self.event_loop.set_control_flow(ControlFlow::Poll);

        let mut last_frame_time = Instant::now();
        let mut fps_counter_time = Instant::now();
        let mut frame_counter = 0;
        let mut fps: f32 = 0.0;
        const TARGET_FRAME_RATE: f32 = 1.0 / 100.0;
        let fixed_update = true;

        let _ = self.event_loop.run_on_demand(|event_loop_event, elwt| {
            match event_loop_event {
                Event::AboutToWait => {
                    if fixed_update == true {
                        if last_frame_time.elapsed().as_secs_f32() >= TARGET_FRAME_RATE {
                            self.window.request_redraw();
                        }
                    } else {
                        self.window.request_redraw();
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        self.destroying = true;
                        elwt.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();
                        frame_counter += 1;
                        last_frame_time = now;

                        if fps_counter_time.elapsed().as_secs_f32()
                            >= Duration::from_secs(1).as_secs_f32()
                        {
                            fps = (frame_counter as f32) / fps_counter_time.elapsed().as_secs_f32();
                            fps_counter_time = now;
                            frame_counter = 0;
                            info!("FPS {:.2}", fps);
                        }

                        if !self.destroying && !self.minimized {
                            match self.last_window_size {
                                Some(window_dimensions) => {
                                    render.render(window_dimensions).unwrap();
                                }
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::Resized(size) => {
                        let window_dimensions = Vector2::new(size.width, size.height);
                        match self.last_window_size {
                            None => self.last_window_size = Some(window_dimensions),
                            Some(_) => {
                                if size.width == 0 || size.height == 0 {
                                    self.minimized = true;
                                } else {
                                    self.minimized = false;
                                    render.resized = true;
                                }

                                self.last_window_size = Some(window_dimensions);
                            }
                        }
                    }
                    WindowEvent::KeyboardInput {
                        event: key_event,
                        is_synthetic: false,
                        ..
                    } => {
                        let keyboard_button_state = match key_event.state {
                            ElementState::Pressed => KeyboardKeyState::Pressed,
                            ElementState::Released => KeyboardKeyState::Released,
                        };

                        let key = match key_event.physical_key {
                            PhysicalKey::Code(key_code) => parse_winit_key_code(key_code),
                            PhysicalKey::Unidentified(_) => None,
                        };

                        if let Some(keyboard_key) = key {
                            // TODO: Remove this, temp for quick debug
                            if keyboard_key == KeyboardKey::Escape {
                                elwt.exit()
                            } else {
                                // Pass to input system
                                INPUT_STATE
                                    .lock()
                                    .unwrap()
                                    .keyboard_state
                                    .set_key_state(keyboard_key, keyboard_button_state);
                            }

                            println!(
                                "Pressed keys {:?}",
                                INPUT_STATE
                                    .lock()
                                    .unwrap()
                                    .keyboard_state
                                    .get_pressed_keys()
                            );
                        } else {
                            info!("Unable to parse key {:?}", key_event.physical_key)
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        let mouse_button_state = match state {
                            ElementState::Pressed => MouseButtonState::Pressed,
                            ElementState::Released => MouseButtonState::Released,
                        };

                        let mouse_button = match button {
                            MouseButton::Right => Some(nova_core::input::MouseButton::RightButton),
                            MouseButton::Left => Some(nova_core::input::MouseButton::LeftButton),
                            MouseButton::Middle => {
                                Some(nova_core::input::MouseButton::MiddleButton)
                            }
                            MouseButton::Back => None,
                            MouseButton::Forward => None,
                            MouseButton::Other(_) => None,
                        };

                        if let Some(button) = mouse_button {
                            INPUT_STATE
                                .lock()
                                .unwrap()
                                .mouse_state
                                .set_mouse_state(button, mouse_button_state);

                            println!(
                                "Pressed mouse buttons {:?}",
                                INPUT_STATE
                                    .lock()
                                    .unwrap()
                                    .mouse_state
                                    .get_pressed_buttons()
                            );
                        } else {
                            info!("Unable to parse mouse button {:?}", button)
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }

    fn destroy(&self) {
        return;
    }

    fn get_raw_display_handle(&self) -> RawDisplayHandle {
        self.window.display_handle().unwrap().as_raw()
    }

    fn get_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle().unwrap()
    }

    fn get_window_dimensions(&self) -> Vector2<u32> {
        let inner_size = self.window.inner_size();
        let dimensions = Vector2::new(inner_size.width, inner_size.height);
        dimensions
    }

    fn get_window(&self) -> &winit::window::Window {
        &self.window
    }

    fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }
}

fn parse_winit_key_code(key_code: KeyCode) -> Option<KeyboardKey> {
    match key_code {
        KeyCode::Digit0 => Some(KeyboardKey::Key0),
        KeyCode::Digit1 => Some(KeyboardKey::Key1),
        KeyCode::Digit2 => Some(KeyboardKey::Key2),
        KeyCode::Digit3 => Some(KeyboardKey::Key3),
        KeyCode::Digit4 => Some(KeyboardKey::Key4),
        KeyCode::Digit5 => Some(KeyboardKey::Key5),
        KeyCode::Digit6 => Some(KeyboardKey::Key6),
        KeyCode::Digit7 => Some(KeyboardKey::Key7),
        KeyCode::Digit8 => Some(KeyboardKey::Key8),
        KeyCode::Digit9 => Some(KeyboardKey::Key9),
        KeyCode::KeyA => Some(KeyboardKey::KeyA),
        KeyCode::KeyB => Some(KeyboardKey::KeyB),
        KeyCode::KeyC => Some(KeyboardKey::KeyC),
        KeyCode::KeyD => Some(KeyboardKey::KeyD),
        KeyCode::KeyE => Some(KeyboardKey::KeyE),
        KeyCode::KeyF => Some(KeyboardKey::KeyF),
        KeyCode::KeyG => Some(KeyboardKey::KeyG),
        KeyCode::KeyH => Some(KeyboardKey::KeyH),
        KeyCode::KeyI => Some(KeyboardKey::KeyI),
        KeyCode::KeyJ => Some(KeyboardKey::KeyJ),
        KeyCode::KeyK => Some(KeyboardKey::KeyK),
        KeyCode::KeyL => Some(KeyboardKey::KeyL),
        KeyCode::KeyM => Some(KeyboardKey::KeyM),
        KeyCode::KeyN => Some(KeyboardKey::KeyN),
        KeyCode::KeyO => Some(KeyboardKey::KeyO),
        KeyCode::KeyP => Some(KeyboardKey::KeyP),
        KeyCode::KeyQ => Some(KeyboardKey::KeyQ),
        KeyCode::KeyR => Some(KeyboardKey::KeyR),
        KeyCode::KeyS => Some(KeyboardKey::KeyS),
        KeyCode::KeyT => Some(KeyboardKey::KeyT),
        KeyCode::KeyU => Some(KeyboardKey::KeyU),
        KeyCode::KeyV => Some(KeyboardKey::KeyV),
        KeyCode::KeyW => Some(KeyboardKey::KeyW),
        KeyCode::KeyX => Some(KeyboardKey::KeyX),
        KeyCode::KeyY => Some(KeyboardKey::KeyY),
        KeyCode::KeyZ => Some(KeyboardKey::KeyZ),
        // KeyCode::Backquote => todo!(),
        // KeyCode::Backslash => todo!(),
        // KeyCode::BracketLeft => todo!(),
        // KeyCode::BracketRight => todo!(),
        // KeyCode::Comma => todo!(),
        // KeyCode::Equal => todo!(),
        // KeyCode::IntlBackslash => todo!(),
        // KeyCode::IntlRo => todo!(),
        // KeyCode::IntlYen => todo!(),
        // KeyCode::Minus => todo!(),
        // KeyCode::Period => todo!(),
        // KeyCode::Quote => todo!(),
        // KeyCode::Semicolon => todo!(),
        // KeyCode::Slash => todo!(),
        // KeyCode::AltLeft => todo!(),
        // KeyCode::AltRight => todo!(),
        // KeyCode::Backspace => todo!(),
        // KeyCode::CapsLock => todo!(),
        // KeyCode::ContextMenu => todo!(),
        // KeyCode::ControlLeft => todo!(),
        // KeyCode::ControlRight => todo!(),
        // KeyCode::Enter => todo!(),
        // KeyCode::SuperLeft => todo!(),
        // KeyCode::SuperRight => todo!(),
        // KeyCode::ShiftLeft => todo!(),
        // KeyCode::ShiftRight => todo!(),
        // KeyCode::Space => todo!(),
        // KeyCode::Tab => todo!(),
        // KeyCode::Convert => todo!(),
        // KeyCode::KanaMode => todo!(),
        // KeyCode::Lang1 => todo!(),
        // KeyCode::Lang2 => todo!(),
        // KeyCode::Lang3 => todo!(),
        // KeyCode::Lang4 => todo!(),
        // KeyCode::Lang5 => todo!(),
        // KeyCode::NonConvert => todo!(),
        KeyCode::Delete => Some(KeyboardKey::Delete),
        // KeyCode::End => todo!(),
        // KeyCode::Help => todo!(),
        // KeyCode::Home => todo!(),
        // KeyCode::Insert => todo!(),
        // KeyCode::PageDown => todo!(),
        // KeyCode::PageUp => todo!(),
        // KeyCode::ArrowDown => todo!(),
        // KeyCode::ArrowLeft => todo!(),
        // KeyCode::ArrowRight => todo!(),
        // KeyCode::ArrowUp => todo!(),
        // KeyCode::NumLock => todo!(),
        // KeyCode::Numpad0 => todo!(),
        // KeyCode::Numpad1 => todo!(),
        // KeyCode::Numpad2 => todo!(),
        // KeyCode::Numpad3 => todo!(),
        // KeyCode::Numpad4 => todo!(),
        // KeyCode::Numpad5 => todo!(),
        // KeyCode::Numpad6 => todo!(),
        // KeyCode::Numpad7 => todo!(),
        // KeyCode::Numpad8 => todo!(),
        // KeyCode::Numpad9 => todo!(),
        // KeyCode::NumpadAdd => todo!(),
        // KeyCode::NumpadBackspace => todo!(),
        // KeyCode::NumpadClear => todo!(),
        // KeyCode::NumpadClearEntry => todo!(),
        // KeyCode::NumpadComma => todo!(),
        // KeyCode::NumpadDecimal => todo!(),
        // KeyCode::NumpadDivide => todo!(),
        // KeyCode::NumpadEnter => todo!(),
        // KeyCode::NumpadEqual => todo!(),
        // KeyCode::NumpadHash => todo!(),
        // KeyCode::NumpadMemoryAdd => todo!(),
        // KeyCode::NumpadMemoryClear => todo!(),
        // KeyCode::NumpadMemoryRecall => todo!(),
        // KeyCode::NumpadMemoryStore => todo!(),
        // KeyCode::NumpadMemorySubtract => todo!(),
        // KeyCode::NumpadMultiply => todo!(),
        // KeyCode::NumpadParenLeft => todo!(),
        // KeyCode::NumpadParenRight => todo!(),
        // KeyCode::NumpadStar => todo!(),
        // KeyCode::NumpadSubtract => todo!(),
        KeyCode::Escape => Some(KeyboardKey::Escape),
        // KeyCode::Fn => todo!(),
        // KeyCode::FnLock => todo!(),
        KeyCode::PrintScreen => Some(KeyboardKey::PrintScreen),
        // KeyCode::ScrollLock => todo!(),
        // KeyCode::Pause => todo!(),
        // KeyCode::BrowserBack => todo!(),
        // KeyCode::BrowserFavorites => todo!(),
        // KeyCode::BrowserForward => todo!(),
        // KeyCode::BrowserHome => todo!(),
        // KeyCode::BrowserRefresh => todo!(),
        // KeyCode::BrowserSearch => todo!(),
        // KeyCode::BrowserStop => todo!(),
        // KeyCode::Eject => todo!(),
        // KeyCode::LaunchApp1 => todo!(),
        // KeyCode::LaunchApp2 => todo!(),
        // KeyCode::LaunchMail => todo!(),
        // KeyCode::MediaPlayPause => todo!(),
        // KeyCode::MediaSelect => todo!(),
        // KeyCode::MediaStop => todo!(),
        // KeyCode::MediaTrackNext => todo!(),
        // KeyCode::MediaTrackPrevious => todo!(),
        // KeyCode::Power => todo!(),
        // KeyCode::Sleep => todo!(),
        // KeyCode::AudioVolumeDown => todo!(),
        // KeyCode::AudioVolumeMute => todo!(),
        // KeyCode::AudioVolumeUp => todo!(),
        // KeyCode::WakeUp => todo!(),
        // KeyCode::Meta => todo!(),
        // KeyCode::Hyper => todo!(),
        // KeyCode::Turbo => todo!(),
        // KeyCode::Abort => todo!(),
        // KeyCode::Resume => todo!(),
        // KeyCode::Suspend => todo!(),
        // KeyCode::Again => todo!(),
        // KeyCode::Copy => todo!(),
        // KeyCode::Cut => todo!(),
        // KeyCode::Find => todo!(),
        // KeyCode::Open => todo!(),
        // KeyCode::Paste => todo!(),
        // KeyCode::Props => todo!(),
        // KeyCode::Select => todo!(),
        // KeyCode::Undo => todo!(),
        // KeyCode::Hiragana => todo!(),
        // KeyCode::Katakana => todo!(),
        KeyCode::F1 => Some(KeyboardKey::F1),
        KeyCode::F2 => Some(KeyboardKey::F2),
        KeyCode::F3 => Some(KeyboardKey::F3),
        KeyCode::F4 => Some(KeyboardKey::F4),
        KeyCode::F5 => Some(KeyboardKey::F5),
        KeyCode::F6 => Some(KeyboardKey::F6),
        KeyCode::F7 => Some(KeyboardKey::F7),
        KeyCode::F8 => Some(KeyboardKey::F8),
        KeyCode::F9 => Some(KeyboardKey::F9),
        KeyCode::F10 => Some(KeyboardKey::F10),
        KeyCode::F11 => Some(KeyboardKey::F11),
        KeyCode::F12 => Some(KeyboardKey::F12),
        // KeyCode::F13 => todo!(),
        // KeyCode::F14 => todo!(),
        // KeyCode::F15 => todo!(),
        // KeyCode::F16 => todo!(),
        // KeyCode::F17 => todo!(),
        // KeyCode::F18 => todo!(),
        // KeyCode::F19 => todo!(),
        // KeyCode::F20 => todo!(),
        // KeyCode::F21 => todo!(),
        // KeyCode::F22 => todo!(),
        // KeyCode::F23 => todo!(),
        // KeyCode::F24 => todo!(),
        // KeyCode::F25 => todo!(),
        // KeyCode::F26 => todo!(),
        // KeyCode::F27 => todo!(),
        // KeyCode::F28 => todo!(),
        // KeyCode::F29 => todo!(),
        // KeyCode::F30 => todo!(),
        // KeyCode::F31 => todo!(),
        // KeyCode::F32 => todo!(),
        // KeyCode::F33 => todo!(),
        // KeyCode::F34 => todo!(),
        // KeyCode::F35 => todo!(),
        _ => None,
    }
}
