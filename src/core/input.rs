use std::sync::Mutex;

pub struct KeyboardState {
    buttons: [KeyboardKeyState; MAX_KEYBOARD_BUTTONS]
}

struct MouseState {
    x_pos: usize,
    y_pos: usize,
    buttons: [MouseButtonState; MAX_MOUSE_BUTTONS]
}

pub struct InputState {
    pub keyboard_state: KeyboardState,
    pub mouse_state: MouseState
}

const MAX_KEYBOARD_BUTTONS: usize = 256;
const MAX_MOUSE_BUTTONS: usize = 8;

pub static INPUT_STATE: Mutex<InputState> = Mutex::new(InputState {
    keyboard_state: KeyboardState {
        buttons: [KeyboardKeyState::Released; MAX_KEYBOARD_BUTTONS]
    },
    mouse_state: MouseState {
        x_pos: 0,
        y_pos: 0,
        buttons: [MouseButtonState::Released; MAX_MOUSE_BUTTONS]
    }
});

impl KeyboardState {
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        let key_index = key as usize;

        if self.buttons[key_index] == KeyboardKeyState::Pressed {
            return true;
        }

        false
    }

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

    pub fn set_key_state(&mut self, key: KeyboardKey, key_state: KeyboardKeyState) {
        let key_index = key as usize;

        if self.buttons[key_index] != key_state {
            self.buttons[key_index] = key_state;
        }
    }
}

impl MouseState {
    pub fn set_mouse_state(&mut self, button: MouseButton, button_state: MouseButtonState) {
        let button_index = button as usize;

        if self.buttons[button_index] != button_state {
            self.buttons[button_index] = button_state;
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyboardKeyState {
    Pressed,
    Released
}

#[derive(Debug)]
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
    KeyDown
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MouseButtonState {
    Pressed,
    Released
}

pub enum MouseButton {
    LeftClick,
    RightClick,
    MiddleButton
}
