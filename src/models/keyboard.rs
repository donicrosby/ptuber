use super::{KeyboardModel, PressedKeyMap, ButtonOrKey};

#[derive(Debug, Clone, Default)]
pub struct Keyboard {
    keys: PressedKeyMap<ButtonOrKey>,
}

impl KeyboardModel for Keyboard {
    type Key = ButtonOrKey;

    fn keys_pressed(&self) -> Vec<ButtonOrKey> {
        self.keys.pressed_keys()
    }
    fn key_pressed(&mut self, key: &ButtonOrKey) {
        self.keys.key_pressed(key);
    }
    fn key_released(&mut self, key: &ButtonOrKey) {
        self.keys.key_released(key);
    }
}
