mod config;
mod errors;

pub use self::config::{Background, Config, Flipper, Mouse};
pub(crate) use self::errors::ConfigError;
