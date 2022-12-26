use crate::config::ConfigError;
use crate::sfml::SfmlError;
use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PTuberError {
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("sfml error")]
    Sfml(#[from] SfmlError)
}

pub type Result<T> = result::Result<T, PTuberError>;
