use std::collections::HashSet;
use std::hash::Hash;

use device_query::Keycode;
use crate::ButtonType;

#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Mouse,
    GamePad
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ButtonOrKey {
    Key(Keycode),
    Button(ButtonType)
}

pub enum GamepadMouseStick {
    Left,
    Right
}

impl Default for DeviceType {
    fn default() -> Self {
        Self::Mouse
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum DeviceButton {
    Left,
    Right,
    Middle,
    Forward,
    Back,
    Unknown(usize),
}

impl Default for DeviceButton {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

#[derive(Debug, Clone)]
pub struct PressedKeyMap<T> {
    pressed_keys: HashSet<T>,
}

impl<T> PressedKeyMap<T>
where
    T: Eq + Hash + PartialEq + Clone,
{
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn pressed_keys(&self) -> Vec<T> {
        self.pressed_keys.iter().cloned().collect()
    }

    pub fn key_pressed(&mut self, key: &T) {
        if !self.pressed_keys.contains(key) {
            self.pressed_keys.insert(key.clone());
        }
    }

    pub fn key_released(&mut self, key: &T) {
        if self.pressed_keys.contains(key) {
            self.pressed_keys.remove(key);
        }
    }
}

impl<T> Default for PressedKeyMap<T> {
    fn default() -> Self {
        let pressed_keys = HashSet::new();
        Self { pressed_keys }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestPressedKeyMapImpl {
        keys: PressedKeyMap<DeviceButton>,
    }

    impl TestPressedKeyMapImpl {
        pub fn new() -> Self {
            let keys = PressedKeyMap::new();
            Self {
                keys
            }
        }

        pub fn pressed_keys(&self) -> Vec<DeviceButton> {
            self.keys.pressed_keys()
        }

        pub fn key_pressed(&mut self, b: &DeviceButton) {
            self.keys.key_pressed(b);
        }

        pub fn key_released(&mut self, b: &DeviceButton) {
            self.keys.key_released(b);
        }
    }

    #[test]
    fn test_keys_pressed() {
        let key_press_map = TestPressedKeyMapImpl::new();
        let keys_pressed = key_press_map.pressed_keys();
        assert!(keys_pressed.is_empty());
    }

    #[test]
    fn test_key_pressed() {
        let mut key_press_map = TestPressedKeyMapImpl::new();
        let key = DeviceButton::Left;
        key_press_map.key_pressed(&key);
        let keys_pressed = key_press_map.pressed_keys();
        assert!(!keys_pressed.is_empty());
        assert!(keys_pressed.contains(&key));
    }

    #[test]
    fn test_key_pressed_multiple_times() {
        let mut key_press_map = TestPressedKeyMapImpl::new();
        let key = DeviceButton::Left;
        key_press_map.key_pressed(&key);
        key_press_map.key_pressed(&key);
        let keys_pressed = key_press_map.pressed_keys();
        assert_eq!(keys_pressed.len(), 1);
    }

    #[test]
    fn test_key_released() {
        let mut key_press_map = TestPressedKeyMapImpl::new();
        let key = DeviceButton::Left;
        key_press_map.key_pressed(&key);
        let keys_pressed = key_press_map.pressed_keys();
        assert!(!keys_pressed.is_empty());

        key_press_map.key_released(&key);
        let keys_pressed = key_press_map.pressed_keys();
        assert!(keys_pressed.is_empty());
    }

    #[test]
    fn test_key_released_multiple_times() {
        let mut key_press_map = TestPressedKeyMapImpl::new();
        let left_key = DeviceButton::Left;
        let right_key = DeviceButton::Right;
        key_press_map.key_pressed(&left_key);
        key_press_map.key_pressed(&right_key);
        let keys_pressed = key_press_map.pressed_keys();
        assert!(!keys_pressed.is_empty());
        assert_eq!(keys_pressed.len(), 2);

        key_press_map.key_released(&right_key);
        let keys_pressed = key_press_map.pressed_keys();
        assert_eq!(keys_pressed.len(), 1);

        key_press_map.key_released(&right_key);
        let keys_pressed = key_press_map.pressed_keys();
        assert_eq!(keys_pressed.len(), 1);
        assert!(!keys_pressed.is_empty());
    }
}
