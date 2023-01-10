mod drain_filter;
mod errors;
mod user_input_monitor;
mod notifier;

use device_query::Keycode;
use sfml::system::Vector2f;

pub use self::errors::UtilError;
use super::DeviceButton;
use drain_filter::DrainFilter;

pub enum DeviceEvent {
    ButtonPressed(DeviceButton),
    ButtonReleased(DeviceButton),
    MouseMoved(Vector2f),
}

pub enum KeyboardEvent {
    KeyPressed(Keycode),
    KeyReleased(Keycode),
}

pub use self::user_input_monitor::UserInputMonitor;
pub use self::notifier::{CallbackGuard, Notifier, NotifierCallback};
