use anyhow::anyhow;
use log::{error, info};
use crate::vulkan_app::VulkanApp;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop},
    window::WindowBuilder,
};
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::event_loop::ControlFlow;
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::core::input::{INPUT_STATE, KeyboardKey, KeyboardKeyState, MouseButtonState};

pub trait Window {
    fn init() -> Self;
    fn run();
    fn destroy();
}

pub struct WinitWindow {
    event_loop: EventLoop<()>
}

impl Window for WinitWindow {
    fn init() -> Self {
        todo!()
    }

    fn run() {
        todo!()
    }

    fn destroy() {
        todo!()
    }
}

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
            Event::WindowEvent { event, .. } => match event {
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
                        Some(physical_size) => {
                            if size.width == 0 || size.height == 0 {
                                minimized = true;
                            } else {
                                minimized = false;
                                vulkan_app.resized = true;
                            }
                            last_window_size = Some(physical_size);
                        }
                    }
                },
                WindowEvent::KeyboardInput { event: key_event, is_synthetic: false, .. } => {
                    let keyboard_button_state = match key_event.state {
                        ElementState::Pressed => KeyboardKeyState::Pressed,
                        ElementState::Released => KeyboardKeyState::Released
                    };

                    let key = match key_event.physical_key {
                        PhysicalKey::Code(key_code) => parse_winit_key_code(key_code),
                        PhysicalKey::Unidentified(_) => None
                    };

                    if let Some(keyboard_key) = key {

                        // TODO: Remove this, temp for quick debug
                        if keyboard_key == KeyboardKey::Escape {
                            elwt.exit()
                        } else {
                            // Pass to input system
                            INPUT_STATE.lock().unwrap().keyboard_state.set_key_state(keyboard_key, keyboard_button_state);
                        }

                        println!("Pressed keys {:?}", INPUT_STATE.lock().unwrap().keyboard_state.get_pressed_keys());
                    } else {
                        info!("Unable to parse key {:?}", key_event.physical_key)
                    }
                },
                WindowEvent::MouseInput { state,  button, ..  } => {
                    let mouse_button_state = match state {
                        ElementState::Pressed => MouseButtonState::Pressed,
                        ElementState::Released => MouseButtonState::Released
                    };

                    let mouse_button = match button {
                        MouseButton::Right => Some(crate::core::input::MouseButton::RightButton),
                        MouseButton::Left => Some(crate::core::input::MouseButton::LeftButton),
                        MouseButton::Middle => Some(crate::core::input::MouseButton::MiddleButton),
                        MouseButton::Back => None,
                        MouseButton::Forward => None,
                        MouseButton::Other(_) => None,
                    };

                    if let Some(button) = mouse_button {
                        INPUT_STATE.lock().unwrap().mouse_state.set_mouse_state(button, mouse_button_state);

                        println!("Pressed mouse buttons {:?}", INPUT_STATE.lock().unwrap().mouse_state.get_pressed_buttons());

                    } else {
                        info!("Unable to parse mouse button {:?}", button)
                    }
                }
                _ => ()
            },
            _ => (),
        }
    }).expect("TODO: panic message");
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
        // KeyCode::Delete => todo!(),
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
        // KeyCode::PrintScreen => todo!(),
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
        // KeyCode::F1 => todo!(),
        // KeyCode::F2 => todo!(),
        // KeyCode::F3 => todo!(),
        // KeyCode::F4 => todo!(),
        // KeyCode::F5 => todo!(),
        // KeyCode::F6 => todo!(),
        // KeyCode::F7 => todo!(),
        // KeyCode::F8 => todo!(),
        // KeyCode::F9 => todo!(),
        // KeyCode::F10 => todo!(),
        // KeyCode::F11 => todo!(),
        // KeyCode::F12 => todo!(),
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
        _ => {
            None
        }
    }
}