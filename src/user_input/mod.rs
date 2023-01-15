mod drain_filter;
mod errors;
mod user_input_monitor;
mod notifier;
mod gamepad;

use device_query::Keycode;
use sfml::system::Vector2f;

use crate::models::DeviceType;
use gamepad::{GamePad, GampadDB, SFMLJoystick, JoystickEvent, AxisType, MAX_AXIS_VAL};

pub use self::gamepad::ButtonType;

pub use self::errors::UtilError;
use super::DeviceButton;
use drain_filter::DrainFilter;

pub enum DeviceEvent {
    ButtonPressed(DeviceButton),
    ButtonReleased(DeviceButton),
    MouseMoved(Vector2f),
    AxisXMoved(f32),
    AxisYMoved(f32),
    DeviceChanged(DeviceType)
}

pub enum KeyboardEvent {
    KeyPressed(Keycode),
    KeyReleased(Keycode),
    ButtonPressed(ButtonType),
    ButtonReleased(ButtonType)
}

pub use self::user_input_monitor::UserInputMonitor;
pub use self::notifier::{CallbackGuard, Notifier, NotifierCallback};
