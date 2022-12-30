use serde_yaml;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("filesystem")]
    File(#[from] io::Error),
    #[error("serde")]
    Serde(#[from] serde_yaml::Error),
}
