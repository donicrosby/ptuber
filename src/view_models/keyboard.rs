use crate::models::KeyboardModel;
use crate::ButtonOrKey;

use super::{Keyboard, KeyboardState, KeysViewModel};

#[derive(Debug, Clone, Default)]
pub struct KeyboardViewModel {
    model: Keyboard,
}

impl KeyboardViewModel {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl KeysViewModel for KeyboardViewModel {
    type Key = ButtonOrKey;
    type KeyboardState = KeyboardState;

    fn key_pressed(&mut self, key: &ButtonOrKey) {
        self.model.key_pressed(key);
    }
    fn key_released(&mut self, key: &ButtonOrKey) {
        self.model.key_released(key);
    }

    fn keyboard_state(&self) -> KeyboardState {
        let keys = self.model.keys_pressed();
        let is_even = keys.len() % 2 == 0;
        if keys.is_empty() {
            KeyboardState::Up
        } else if is_even {
            KeyboardState::Left
        } else {
            KeyboardState::Right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use device_query::Keycode;

    #[test]
    fn test_keyboard_state() {
        let mut keyboard = KeyboardViewModel::new();
        let a_key = ButtonOrKey::Key(Keycode::A);
        let b_key = ButtonOrKey::Key(Keycode::B);
        let c_key = ButtonOrKey::Key(Keycode::C);
        let d_key = ButtonOrKey::Key(Keycode::D);

        let up = KeyboardState::Up;
        assert_eq!(up, keyboard.keyboard_state());

        keyboard.key_pressed(&a_key);
        let right = KeyboardState::Right;
        assert_eq!(right, keyboard.keyboard_state());

        keyboard.key_pressed(&b_key);
        let left = KeyboardState::Left;
        assert_eq!(left, keyboard.keyboard_state());

        keyboard.key_pressed(&c_key);
        assert_eq!(right, keyboard.keyboard_state());

        keyboard.key_pressed(&d_key);
        assert_eq!(left, keyboard.keyboard_state());
    }
}
