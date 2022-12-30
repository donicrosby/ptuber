use clap::Parser;
use log::debug;

pub mod args;
pub mod config;
pub mod errors;
mod sfml;
pub(crate) mod util;

use self::args::Args;
pub use self::args::{DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME};
use self::config::Config;
pub use self::errors::Result as PtuberResult;
use self::sfml::PtuberWindow;
pub use self::util::{get_window_finder, WindowFinder, WindowFinderError};

pub const MAX_FRAMERATE: u32 = 60;

#[derive(Debug, Default)]
pub struct PTuber<'a> {
    config: Config,
    display: PtuberWindow<'a>,
}

impl<'a> PTuber<'a> {
    pub fn new() -> PtuberResult<Self> {
        let args = Self::parse_args();
        debug!("Skin path: {:?}", args.skin_dir());
        debug!("Config path: {:?}", args.config_path());
        let config = Config::new(&args.config_path());
        let display = PtuberWindow::new(&args.skin_dir(), config.clone())?;
        Ok(Self { config, display })
    }
    pub fn start_ptuber(self) -> PtuberResult<()> {
        debug!("Current Config: {:?}", self.config);
        self.display.display(&self.config)?;
        Ok(())
    }

    fn parse_args() -> Args {
        Args::parse()
    }
}
