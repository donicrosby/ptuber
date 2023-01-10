use super::DeviceButton;
use device_query::MouseButton;

impl From<MouseButton> for DeviceButton {
    fn from(value: MouseButton) -> Self {
        match value {
            1 => DeviceButton::Left,
            2 => DeviceButton::Middle,
            3 => DeviceButton::Right,
            4 => DeviceButton::Forward,
            5 => DeviceButton::Back,
            v => DeviceButton::Unknown(v),
        }
    }
}
