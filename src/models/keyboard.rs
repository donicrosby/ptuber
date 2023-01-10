use device_query::Keycode;

use super::{KeyboardModel, PressedKeyMap};

#[derive(Debug, Clone, Default)]
pub struct Keyboard {
    keys: PressedKeyMap<Keycode>,
}

impl KeyboardModel for Keyboard {
    type Key = Keycode;

    fn keys_pressed(&self) -> Vec<Keycode> {
        self.keys.pressed_keys()
    }
    fn key_pressed(&mut self, key: &Keycode) {
        self.keys.key_pressed(key);
    }
    fn key_released(&mut self, key: &Keycode) {
        self.keys.key_released(key);
    }
}
