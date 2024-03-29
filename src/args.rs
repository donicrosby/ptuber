use clap::{Parser, ValueHint};
use std::path::{PathBuf, MAIN_SEPARATOR};

pub const DEFAULT_SKIN_DIR_NAME: &str = "skin";
pub const DEFAULT_CONFIG_NAME: &str = "config.toml";

pub fn default_skin_dir() -> String {
    format!(".{}{}", MAIN_SEPARATOR, DEFAULT_SKIN_DIR_NAME)
}

pub fn default_config() -> String {
    format!(
        ".{}{}{}{}",
        MAIN_SEPARATOR, DEFAULT_SKIN_DIR_NAME, MAIN_SEPARATOR, DEFAULT_CONFIG_NAME
    )
}

/// Ptuber vtuber simple rigger
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory to find the sprite files and config
    #[arg(short, long, default_value_t = default_skin_dir() , value_hint=ValueHint::DirPath)]
    pub skin_dir: String,
    /// What config file to use
    #[arg(short, long, default_value_t = default_config(), value_hint=ValueHint::FilePath)]
    pub config: String,
}

impl Args {
    pub fn config_path(&self) -> PathBuf {
        PathBuf::from(&self.config)
    }

    pub fn skin_dir(&self) -> PathBuf {
        PathBuf::from(&self.skin_dir)
    }
}
