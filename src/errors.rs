use crate::config::ConfigError;
use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PTuberError {
    #[error("config error")]
    Config(#[from] ConfigError),
}

pub type Result<T> = result::Result<T, PTuberError>;
