use super::DeviceButton;
use device_query::MouseButton;

impl From<MouseButton> for DeviceButton {
    fn from(value: MouseButton) -> Self {
        match value {
            1 => DeviceButton::Left,
            2 => DeviceButton::Right,
            3 => DeviceButton::Middle,
            4 => DeviceButton::Forward,
            5 => DeviceButton::Back,
            v => DeviceButton::Unknown(v),
        }
    }
}
