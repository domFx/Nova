#![allow(dead_code)]

use std::sync::Mutex;

/// Nova Engine Global Input State
pub static INPUT_STATE: Mutex<InputState> = Mutex::new(InputState {
    keyboard_state: KeyboardState {
        buttons: [KeyboardKeyState::Released; MAX_KEYBOARD_BUTTONS]
    },
    mouse_state: MouseState {
        x_pos: 0.0,
        y_pos: 0.0,
        buttons: [MouseButtonState::Released; MAX_MOUSE_BUTTONS]
    }
});

pub struct KeyboardState {
    buttons: [KeyboardKeyState; MAX_KEYBOARD_BUTTONS]
}

pub struct MouseState {
    x_pos: f32,
    y_pos: f32,
    buttons: [MouseButtonState; MAX_MOUSE_BUTTONS]
}

pub struct InputState {
    pub keyboard_state: KeyboardState,
    pub mouse_state: MouseState
}

const MAX_KEYBOARD_BUTTONS: usize = 256;
const MAX_MOUSE_BUTTONS: usize = 8;

impl KeyboardState {
    /// Returns true if a given keyboard key is pressed
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        let key_index = key as usize;

        if self.buttons[key_index] == KeyboardKeyState::Pressed {
            return true;
        }

        false
    }

    /// Returns a vector of pressed keyboard keys
    pub fn get_pressed_keys(&self) -> Vec<KeyboardKey> {
        self.buttons
            .iter()
            .enumerate()
            .filter(|(_, k)| **k == KeyboardKeyState::Pressed)
            .map(|(i , _)| unsafe {
                std::mem::transmute(i as u8)
            })
            .collect::<Vec<KeyboardKey>>()
    }

    /// Sets the state of a given keyboard key
    pub fn set_key_state(&mut self, key: KeyboardKey, key_state: KeyboardKeyState) {
        let key_index = key as usize;

        if self.buttons[key_index] != key_state {
            self.buttons[key_index] = key_state;
        }
    }
}

/// Mouse state implementation
impl MouseState {
    /// Returns true if a given mouse button is pressed
    pub fn is_button_pressed(
        &self,
        mouse_button: MouseButton
    ) -> bool {
        let button_index = get_mouse_index(mouse_button);

        if let Some(idx) = button_index {
            if self.buttons[idx] == MouseButtonState::Pressed {
                return true;
            }
        }

        false
    }

    /// Returns a vector of pressed mouse buttons
    pub fn get_pressed_buttons(&self) -> Vec<MouseButton> {
        self.buttons
            .iter()
            .enumerate()
            .filter(|(_, m)| **m == MouseButtonState::Pressed)
            .map(|(i , _)| unsafe {
                std::mem::transmute(i as u8)
            })
            .collect::<Vec<MouseButton>>()
    }

    /// Returns the current state of a given mouse button
    pub fn get_mouse_button_state(
        &self,
        mouse_button: MouseButton
    ) -> MouseButtonState {
        let button_index = get_mouse_index(mouse_button);

        if let Some(idx) = button_index {
            return self.buttons[idx];
        }

        // Currently returning released if mouse button is not within the bounds
        return MouseButtonState::Released
    }

    /// Sets the state of a given mouse button
    pub fn set_mouse_state(
        &mut self,
        mouse_button: MouseButton,
        mouse_button_state: MouseButtonState
    ) {
        let button_index = mouse_button as usize;

        if self.buttons[button_index] != mouse_button_state {
            self.buttons[button_index] = mouse_button_state;
        }
    }

    /// Set mouse position
    /// TODO: Create vector implementation
    pub fn set_mouse_position(
        &mut self,
        x_pos: f32,
        y_pos: f32
    ) {
        self.y_pos = y_pos;
        self.x_pos = x_pos;
    }

    /// Returns the mouse position as a tuple
    /// TODO: Create vector implementation
    pub fn get_mouse_position(
        &self
    ) -> (f32, f32) {
        (self.x_pos, self.y_pos)
    }
}

fn get_mouse_index(mouse_button: MouseButton) -> Option<usize> {
    let idx = mouse_button as usize;

    let max_button_index = MouseButton::MaxButton as usize;

    if idx < max_button_index {
        return Some(idx)
    }

    None
}

fn get_key_index(keyboard_key: KeyboardKey) -> Option<usize> {
    let idx = keyboard_key as usize;

    let max_keyboard_index = KeyboardKey::KeyMax as usize;

    if idx < max_keyboard_index {
        return Some(idx)
    }

    None
}

/// Keyboard State enum
/// Represents whether a key is pressed
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyboardKeyState {
    Pressed,
    Released
}

/// Keyboard Keys enum
// TODO: Remove Eq, PartialEq with escape to close window
#[derive(Debug, Eq, PartialEq)]
pub enum KeyboardKey {
    KeyA = 0,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    KeyLeft,
    KeyRight,
    KeyUp,
    KeyDown,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    Delete,
    // DO NOT USE: Unused key, represents an unreachable key for bounds checking
    KeyMax,
}

/// Mouse Button State enum
/// Represents whether a mouse button is pressed
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MouseButtonState {
    Pressed,
    Released
}

/// Mouse Button enum
#[derive(Debug)]
pub enum MouseButton {
    LeftButton,
    RightButton,
    MiddleButton,
    //DO NOT USE: Unused key, represents an unreachable key for bounds checking
    MaxButton
}
