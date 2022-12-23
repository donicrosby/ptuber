use clap::{Parser, ValueHint};
use std::path::{PathBuf};

/// Ptuber vtuber simple rigger
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
   /// Directory to find the sprite files and config
   #[arg(short, long, default_value_t = String::from("./skin"), value_hint=ValueHint::DirPath)]
   pub skin_dir: String,
   /// What config file to use
   #[arg(short, long, default_value_t = String::from("./skin/config.yaml"), value_hint=ValueHint::FilePath)]
   pub config: String,
}

impl Args {
   pub fn config_path(self) -> PathBuf {
      PathBuf::from(self.config)
   }

   pub fn skin_dir(self) -> PathBuf {
      PathBuf::from(self.skin_dir)
   }
}