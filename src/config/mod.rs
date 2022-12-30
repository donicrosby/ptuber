mod config_impl;
mod errors;

pub use self::config_impl::{Anchors, Color, Config, WindowDimensions};
pub(crate) use self::errors::ConfigError;
