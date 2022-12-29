use crate::config::ConfigError;
use crate::sfml::SfmlError;
use std::result;
use thiserror::Error;
use crate::WindowFinderError;

#[derive(Error, Debug)]
pub enum PTuberError {
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("sfml error")]
    Sfml(#[from] SfmlError),
    #[error("window finder error")]
    WindowFinder(#[from] WindowFinderError)
}

pub type Result<T> = result::Result<T, PTuberError>;