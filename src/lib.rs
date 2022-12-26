use clap::Parser;

pub mod config;
pub mod errors;
pub mod args;
mod sfml;

use self::args::Args;
pub use self::args::{DEFAULT_SKIN_DIR_NAME, DEFAULT_CONFIG_NAME};
pub use self::errors::Result;
use self::config::Config;
use self::sfml::{PtuberWindow};

pub const WINDOW_WIDTH: u32 = 612;
pub const WINDOW_HEIGHT: u32 = 352;
pub const MAX_FRAMERATE: u32 = 60;

#[derive(Debug, Default)]
pub struct PTuber {
    config: Config,
    display: PtuberWindow,
}

impl PTuber {
    pub fn new() -> Result<Self> {
        let args = Self::parse_args();
        println!("Skin path: {:?}", args.clone().skin_dir());
        println!("Config path: {:?}", args.clone().config_path());
        let config = Config::new(&args.config_path());
        let display = PtuberWindow::new(&args.skin_dir())?;
        Ok(Self {
            config,
            display
        })
    }
    pub fn start_ptuber(self) -> Result<()> {
        println!("Config: {:?}", self.config.clone());
        self.display.display(&self.config)?;
        Ok(())
    }

    fn parse_args() -> Args {
        Args::parse()
    }
}

