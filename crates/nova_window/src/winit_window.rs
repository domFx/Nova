use crate::window::NovaWindow;
use nova_core::input::{KeyboardKey, KeyboardKeyState, MouseButtonState, INPUT_STATE};
use nova_math::vector::vector2::Vector2;
use nova_renderer::renderer::Renderer;

use log::{debug, error, info};
use anyhow::Result;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle};
use winit::{
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowAttributes,
};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub struct WinitWindow<'a> {
    window: Option<winit::window::Window>,
    renderer: &'a mut Renderer,

    raw_display_handle: Option<RawDisplayHandle>,
    raw_window_handle: Option<RawWindowHandle>,

    minimized: bool,
    destroying: bool,
    redraw: bool,
    running: bool,

    window_size: Vector2<u32>,
    window_title: String
}

impl<'a> WinitWindow<'a> {
    pub fn new(window_title: &str, renderer: &'a mut Renderer) -> Result<Self> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut winit_window = Self {
            window: None,
            renderer,

            raw_display_handle: None,
            raw_window_handle: None,

            destroying: false,
            minimized: false,
            redraw: false,
            running: true,

            window_size: Vector2::zero(),
            window_title: String::from(window_title)
        };

        event_loop.run_app(&mut winit_window).unwrap();

        Ok(winit_window)
    }
}

impl NovaWindow<'_> for WinitWindow<'_> {
    fn run(&self) {
       return
    }

    fn destroy(&self) {
        return
    }

    fn get_raw_display_handle(&self) -> RawDisplayHandle {
        self.raw_display_handle.unwrap()
    }

    fn get_window_handle(&self) -> RawWindowHandle {
        self.raw_window_handle.unwrap()
    }

    fn get_window_dimensions(&self) -> Vector2<u32> {
        self.window_size
    }

    fn set_title(&mut self, title: &str) {
        self.window_title = title.to_string();
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn redraw(&self) -> bool {
        self.redraw
    }
}

impl ApplicationHandler for WinitWindow<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_title = self.window_title.to_string();

        let window_attributes = WindowAttributes::default()
            .with_title(window_title)
            .with_inner_size(LogicalSize::new(800, 600));

        let window = event_loop.create_window(window_attributes).unwrap();

        let inner_size = window.inner_size();
        let window_size = Vector2::new(inner_size.width, inner_size.height);

        self.raw_display_handle = Some(window.raw_display_handle().unwrap());
        self.raw_window_handle = Some(window.window_handle().unwrap().raw_window_handle().unwrap());

        self.window_size = window_size;

        self.renderer.init(self.raw_window_handle.unwrap(), self.raw_display_handle.unwrap(), window_size).unwrap();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(size) => {
                let current_window_size = Vector2::new(size.width, size.height);

                if !self.window_size.equals(&current_window_size) {
                    self.window_size = current_window_size;
                }
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                debug!("WindowEvent: Close Requested");
                self.destroying = true;
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
                self.renderer.render(self.window_size).unwrap();

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }

                if !self.destroying && !self.minimized && !self.window_size.equals(&Vector2::zero()) {
                    self.redraw = true;
                } else {
                    self.redraw = false;
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        debug!("WinitWindow Exiting");

        if !event_loop.exiting() {
            event_loop.exit();
        }

        self.running = false;
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