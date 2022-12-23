use clap::Parser;

pub mod config;
pub mod errors;
pub mod args;

use self::args::Args;
pub use self::errors::Result;
use self::config::Config;

pub const WINDOW_WIDTH: u64 = 612;
pub const WINDOW_HEIGHT: u64 = 352;
pub const MAX_FRAMERATE: u64 = 60;

#[derive(Debug, Default)]
pub struct PTuber {
    config: Config
}

impl PTuber {
    pub fn new() -> Self {
        let args = Self::parse_args();
        println!("Skin path: {:?}", args.clone().skin_dir());
        println!("Config path: {:?}", args.clone().config_path());
        let config = Config::new(&args.config_path());
        Self {
            config
        }
    }
    pub fn start_ptuber(self) -> Result<()> {
        println!("Config: {:?}", self.config.clone());
        Ok(())
    }

    fn parse_args() -> Args {
        Args::parse()
    }
}

