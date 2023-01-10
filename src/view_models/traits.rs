pub trait DeviceViewModel {
    type Button;
    type ButtonState;
    type Position;

    fn position(&self) -> Self::Position;
    fn set_position(&mut self, position: &Self::Position);

    fn button_pressed(&mut self, button: &Self::Button);
    fn button_released(&mut self, button: &Self::Button);

    fn button_state(&self) -> Self::ButtonState;
}

pub trait KeysViewModel {
    type Key;
    type KeyboardState;

    fn key_pressed(&mut self, key: &Self::Key);
    fn key_released(&mut self, key: &Self::Key);

    fn keyboard_state(&self) -> Self::KeyboardState;
}

pub trait AsyncUpdatedViewModel {
    fn start(&self);
    fn stop(&self);
}
