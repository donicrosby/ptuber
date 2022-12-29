use clap::Parser;
use log::{debug};


pub mod config;
pub mod errors;
pub mod args;
mod sfml;
pub(crate) mod util;

use self::args::Args;
pub use self::args::{DEFAULT_SKIN_DIR_NAME, DEFAULT_CONFIG_NAME};
pub use self::errors::Result as PtuberResult;
use self::config::Config;
pub use self::util::{WindowFinder, WindowFinderError, get_window_finder};
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
    pub fn new() -> PtuberResult<Self> {
        let args = Self::parse_args();
        debug!("Skin path: {:?}", args.clone().skin_dir());
        debug!("Config path: {:?}", args.clone().config_path());
        let config = Config::new(&args.config_path());
        let display = PtuberWindow::new(&args.skin_dir())?;
        Ok(Self {
            config,
            display
        })
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

