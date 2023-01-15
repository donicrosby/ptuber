mod keyboard;
#[cfg(all(unix, target_os = "linux"))]
mod linux;
mod mouse;
mod traits;
mod types;
#[cfg(windows)]
mod windows;

pub(crate) use self::keyboard::Keyboard;
pub(crate) use self::mouse::MouseModel;
pub(crate) use self::traits::{DeviceModel, KeyboardModel};
pub(crate) use self::types::{DeviceButton, PressedKeyMap, DeviceType, GamepadMouseStick, ButtonOrKey};
