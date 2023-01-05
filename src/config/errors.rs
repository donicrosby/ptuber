use derive_more::Display;
use std::io;
use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("filesystem")]
    File(#[from] io::Error),
    #[error("serde")]
    Serde(#[from] TomlError),
}

#[derive(Error, Debug, Display)]
pub enum TomlError {
    Deserialize(#[from] toml::de::Error),
    Serialize(#[from] toml::ser::Error),
    DateTimeParse(#[from] toml::value::DatetimeParseError),
}
