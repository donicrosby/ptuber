use sfml::system::Vector2f;
use super::{DeviceButton, DeviceModel, PressedKeyMap};


#[derive(Debug, Clone, Default)]
pub struct MouseModel {
    position: Vector2f,
    buttons: PressedKeyMap<DeviceButton>,
}

impl MouseModel {
    pub fn new() -> Self {
        let buttons = PressedKeyMap::new();
        Self {
            buttons,
            ..Default::default()
        }
    }
}

impl DeviceModel for MouseModel {
    type Button = DeviceButton;
    type Position = Vector2f;
    
    fn position(&self) -> Vector2f {
        self.position
    }

    fn set_position(&mut self, pos: &Vector2f) {
        self.position = *pos;
    }

    fn buttons_pressed(&self) -> Vec<DeviceButton> {
        self.buttons.pressed_keys()
    }

    fn button_pressed(&mut self, button: &DeviceButton) {
        self.buttons.key_pressed(button);
    }

    fn button_released(&mut self, button: &DeviceButton) {
        self.buttons.key_released(button);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_position() {
        let mouse = MouseModel::new();
        let default_pos: Vector2f = Default::default();
        let pos = mouse.position();
        assert_eq!(pos, default_pos);
    }

    #[test]
    fn test_set_mouse_position() {
        let mut mouse = MouseModel::new();
        let new_pos = Vector2f::new(3.0, 3.0);
        mouse.set_position(&new_pos);
        let pos = mouse.position();
        assert_eq!(pos, new_pos);
    }
}