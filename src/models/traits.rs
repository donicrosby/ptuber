pub trait DeviceModel {
    type Button;
    type Position;
    type DeviceType;

    fn position(&self) -> Self::Position;
    fn set_position(&mut self, pos: &Self::Position);
    fn buttons_pressed(&self) -> Vec<Self::Button>;
    fn button_pressed(&mut self, button: &Self::Button);
    fn button_released(&mut self, button: &Self::Button);
    fn device_type(&self) -> Self::DeviceType;
    fn set_device_type(&mut self, device_type: &Self::DeviceType);
    
}

pub trait KeyboardModel {
    type Key;
    fn keys_pressed(&self) -> Vec<Self::Key>;
    fn key_pressed(&mut self, key: &Self::Key);
    fn key_released(&mut self, key: &Self::Key);
}
