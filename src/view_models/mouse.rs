use crate::models::DeviceModel;
use sfml::system::Vector2f;

use super::{DeviceButton, DeviceViewModel, MouseButtonState, MouseModel};

#[derive(Debug, Clone, Default)]
pub struct MouseViewModel {
    model: MouseModel,
}

impl MouseViewModel {
    pub fn new() -> Self {
        let model = MouseModel::new();
        Self {
            model
        }
    }
}

impl DeviceViewModel for MouseViewModel {
    type Button = DeviceButton;
    type ButtonState = MouseButtonState;
    type Position = Vector2f;

    fn position(&self) -> Vector2f {
        self.model.position()
    }
    fn set_position(&mut self, pos: &Vector2f) {
        self.model.set_position(pos)
    }

    fn button_pressed(&mut self, button: &DeviceButton) {
        self.model.button_pressed(button);
    }
    fn button_released(&mut self, button: &DeviceButton) {
        self.model.button_released(button);
    }

    fn button_state(&self) -> MouseButtonState {
        let buttons = self.model.buttons_pressed();
        let left_pressed = buttons.contains(&DeviceButton::Left);
        let right_pressed = buttons.contains(&DeviceButton::Right);
        match (left_pressed, right_pressed) {
            (false, false) => MouseButtonState::None,
            (true, false) => MouseButtonState::Left,
            (false, true) => MouseButtonState::Right,
            (true, true) => MouseButtonState::Both,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_state() {
        let mut mouse = MouseViewModel::new();
        let left_button = DeviceButton::Left;
        let right_button = DeviceButton::Right;
        let none = MouseButtonState::None;
        assert_eq!(none, mouse.button_state());

        mouse.button_pressed(&left_button);
        let left = MouseButtonState::Left;
        assert_eq!(left, mouse.button_state());

        mouse.button_pressed(&right_button);
        let both = MouseButtonState::Both;
        assert_eq!(both, mouse.button_state());

        mouse.button_released(&left_button);
        let right = MouseButtonState::Right;
        assert_eq!(right, mouse.button_state());
    }
}
