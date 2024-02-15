use anyhow::anyhow;
use log::{error, info};
use crate::vulkan_app::VulkanApp;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop},
    window::WindowBuilder,
};
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::ControlFlow;
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::core::input::{INPUT_STATE, KeyboardKey, KeyboardKeyState};

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
                },
                WindowEvent::KeyboardInput { event: key_event, is_synthetic: false, .. } => {
                    let keyboard_button_state = match key_event.state {
                        ElementState::Pressed => KeyboardKeyState::Pressed,
                        ElementState::Released => KeyboardKeyState::Released
                    };

                    let key = match key_event.physical_key {
                        PhysicalKey::Code(key_code) => Some(parse_winit_key_code(key_code)),
                        PhysicalKey::Unidentified(_) => { None }
                    };

                    if let Some(keyboard_key) = key {
                        // Pass to input system
                        INPUT_STATE.lock().unwrap().keyboard_state.set_key_state(keyboard_key, keyboard_button_state);

                        println!("pressed keys {:?}", INPUT_STATE.lock().unwrap().keyboard_state.get_pressed_keys());
                    } else {
                        info!("Unable to parse key {:?}", key_event.physical_key)
                    }
                }
                _ => ()
            },
            _ => (),
        }
    }).expect("TODO: panic message");
}

fn parse_winit_key_code(key_code: KeyCode) -> KeyboardKey {
    match key_code {
        KeyCode::Digit0 => KeyboardKey::Key0,
        KeyCode::Digit1 => KeyboardKey::Key1,
        KeyCode::Digit2 => KeyboardKey::Key2,
        KeyCode::Digit3 => KeyboardKey::Key3,
        KeyCode::Digit4 => KeyboardKey::Key4,
        KeyCode::Digit5 => KeyboardKey::Key5,
        KeyCode::Digit6 => KeyboardKey::Key6,
        KeyCode::Digit7 => KeyboardKey::Key7,
        KeyCode::Digit8 => KeyboardKey::Key8,
        KeyCode::Digit9 => KeyboardKey::Key9,
        KeyCode::KeyA => KeyboardKey::KeyA,
        KeyCode::KeyB => KeyboardKey::KeyB,
        KeyCode::KeyC => KeyboardKey::KeyC,
        KeyCode::KeyD => KeyboardKey::KeyD,
        KeyCode::KeyE => KeyboardKey::KeyE,
        KeyCode::KeyF => KeyboardKey::KeyF,
        KeyCode::KeyG => KeyboardKey::KeyG,
        KeyCode::KeyH => KeyboardKey::KeyH,
        KeyCode::KeyI => KeyboardKey::KeyI,
        KeyCode::KeyJ => KeyboardKey::KeyJ,
        KeyCode::KeyK => KeyboardKey::KeyK,
        KeyCode::KeyL => KeyboardKey::KeyL,
        KeyCode::KeyM => KeyboardKey::KeyM,
        KeyCode::KeyN => KeyboardKey::KeyN,
        KeyCode::KeyO => KeyboardKey::KeyO,
        KeyCode::KeyP => KeyboardKey::KeyP,
        KeyCode::KeyQ => KeyboardKey::KeyQ,
        KeyCode::KeyR => KeyboardKey::KeyR,
        KeyCode::KeyS => KeyboardKey::KeyS,
        KeyCode::KeyT => KeyboardKey::KeyT,
        KeyCode::KeyU => KeyboardKey::KeyU,
        KeyCode::KeyV => KeyboardKey::KeyV,
        KeyCode::KeyW => KeyboardKey::KeyW,
        KeyCode::KeyX => KeyboardKey::KeyX,
        KeyCode::KeyY => KeyboardKey::KeyY,
        KeyCode::KeyZ => KeyboardKey::KeyZ,
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
        // KeyCode::Escape => todo!(),
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
            todo!()
        }
    }
}