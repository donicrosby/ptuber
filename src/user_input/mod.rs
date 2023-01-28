mod drain_filter;
mod errors;
mod user_input_monitor;
mod notifier;

use device_query::Keycode;
use sfml::system::Vector2f;
use gilrs::Button;

use crate::models::DeviceType;
pub const MAX_AXIS_VAL: f32 = 1.0;

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
    ButtonPressed(Button),
    ButtonReleased(Button)
}

pub use self::user_input_monitor::UserInputMonitor;
pub use self::notifier::{CallbackGuard, Notifier, NotifierCallback};
