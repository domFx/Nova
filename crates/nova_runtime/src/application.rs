use nova_core::{color::Color, input::{KeyboardKey, KeyboardKeyState, MouseButtonState, INPUT_STATE}};
use nova_math::vector::{Vector2, Vector4};
use nova_renderer::renderer::{self, Renderer};

use anyhow::{anyhow, Result};
use log::{debug, error, Log};
use raw_window_handle::{DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle};
use std::default;
use winit::{
    dpi::{PhysicalSize, Position}, 
    event::{ElementState, MouseButton, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}, 
    window::WindowAttributes
};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowId;

#[derive(Debug, PartialEq)]
enum NovaApplicationState {
    Initialization,
    Running,
    Cleanup,
    Exiting,
    Complete,
}

pub struct NovaApplication {
    renderer: Option<Box<Renderer>>,
    window_state: Option<WindowState>,
    application_state: NovaApplicationState
}

impl NovaApplication {
    pub fn new() -> Self {
        Self {
            renderer: None,
            window_state: None,
            application_state: NovaApplicationState::Initialization
        }
    }

    pub fn use_pretty_env_logger(self) -> NovaApplication {
        pretty_env_logger::init();
        self
    }

    pub fn set_log_level(self, log_level: &str) -> NovaApplication {
        std::env::set_var("RUST_LOG", log_level);
        self
    }

    pub fn run(mut self) -> Result<NovaApplication> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        
        event_loop.run_app(&mut self);
        Ok(self)
    }

    fn destroy(&mut self) {
        debug!("NovaEngine clean up.");
        self.application_state = NovaApplicationState::Cleanup;

        if let Some(mut renderer) = self.renderer.as_mut() {
            renderer.destroy();
        }

        self.application_state = NovaApplicationState::Complete;
    }
}

impl ApplicationHandler for NovaApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.application_state == NovaApplicationState::Initialization {
            let window_title = "Nova Engine";

            let window_attributes = WindowAttributes::default()
                .with_title(window_title)
                .with_inner_size(LogicalSize::new(1280, 720));
    
            let window = event_loop.create_window(window_attributes).unwrap();

            let inner_size = window.inner_size();
            let window_size = Vector2::new(inner_size.width, inner_size.height);    

            // TODO: Make renderer backend configurable
            let mut renderer: Renderer = Renderer::new_vulkan_renderer(
                window.window_handle().unwrap(),
                window.display_handle().unwrap(),
                window_size,
                window.scale_factor() as f32 // Loss of precision but likely fine
            );

            let window_state = WindowState {
                window,
                window_size,
                redraw: false,
                destroying: false,
                minimized: false
            };

            self.renderer = Some(Box::from(renderer));
            self.window_state = Some(window_state);

            self.application_state = NovaApplicationState::Running;
            debug!("Application initialization complete -> Running")
        } else {
            debug!("Application resumed");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(size) => {
                let current_window_size = Vector2::new(size.width, size.height);

                if let Some(window_state) = self.window_state.as_mut() {
                    if !window_state.window_size.equals(&current_window_size) {
                        window_state.window_size = current_window_size;
                    }
                }
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                debug!("WindowEvent: Close Requested");
                if let Some(window_state) = self.window_state.as_mut() {
                    window_state.destroying = true;
                }
                event_loop.exit();
            }
            WindowEvent::Destroyed => {
                event_loop.exit();
            }
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
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
                    if keyboard_key == KeyboardKey::Escape {
                        event_loop.exit()
                    } else {
                        // Pass to input system
                        INPUT_STATE
                            .lock()
                            .unwrap()
                            .keyboard_state
                            .set_key_state(keyboard_key, keyboard_button_state);
                    }

                    let pressed_keys = INPUT_STATE
                        .lock()
                        .unwrap()
                        .keyboard_state
                        .get_pressed_keys();
                    debug!("Pressed keys {:?}", pressed_keys);
                } else {
                    error!("Unable to parse key {:?}", key_event.physical_key)
                }
            }
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button_state = match state {
                    ElementState::Pressed => MouseButtonState::Pressed,
                    ElementState::Released => MouseButtonState::Released,
                };

                let mouse_button = match button {
                    MouseButton::Right => Some(nova_core::input::MouseButton::RightButton),
                    MouseButton::Left => Some(nova_core::input::MouseButton::LeftButton),
                    MouseButton::Middle => Some(nova_core::input::MouseButton::MiddleButton),
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
                } else {
                    error!("Unable to parse mouse button {:?}", button)
                }
            }
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                if let Some(window_state) = self.window_state.as_mut() {
                    if let Some(renderer) = self.renderer.as_mut() {
                        renderer.begin_rendering(window_state.window_size);
    
                        // Start Drawing
                        renderer.draw_filled_rect(
                            Vector2::new(0f32, 0f32),
                            0.7f32,
                            0.7f32,
                            Color::from_vector4(Vector4::new(1.0, 1.0, 0.0, 1.0)),
                        );
    
                        renderer.draw_line(
                            Vector2::new(0f32, 0f32),
                            Vector2::new(0.5f32, 0.5f32),
                            Color::WHITE,
                        );
                        
                        renderer.draw_image(Vector2::zero(), nova_core::image::Image::load("./src/images/texture.png").unwrap());
                        
                        // End Drawing
                        renderer.end_rendering();
                    }
    
                    if !window_state.destroying && !window_state.minimized && !window_state.get_window_size().equals(&Vector2::zero())
                    {
                        window_state.redraw = true;
                    } else {
                        window_state.redraw = false;
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window_state) = self.window_state.as_ref() {
            window_state.window.request_redraw();
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        debug!("Nova Engine Exiting");
        self.application_state = NovaApplicationState::Exiting;

        if event_loop.exiting() {
            event_loop.exit();
        }

        self.destroy();
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

pub(crate) struct WindowState {
    window: Window,
    window_size: Vector2<u32>,
    redraw: bool,
    destroying: bool,
    minimized: bool
}

impl WindowState {
    pub(crate) fn get_window_size(&self) -> Vector2<u32> {
        let inner_size = self.window.inner_size();
        
        Vector2::new(inner_size.width, inner_size.height)
    } 
}