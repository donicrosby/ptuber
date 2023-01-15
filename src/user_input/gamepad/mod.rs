mod sdl_parse;
mod types;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "controller-db/"]
#[include = "*.txt"]
pub struct GampadDB;

pub use self::types::{Axisish, Buttonish, GamePad, SFMLJoystick, JoystickEvent, ButtonType, AxisType, MAX_AXIS_VAL};
use self::sdl_parse::{AxisOrButton, Guid, parse_axis_or_button};