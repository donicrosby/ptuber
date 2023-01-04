use clap::Parser;
use log::debug;

pub mod args;
pub mod config;
pub mod errors;
mod avatar;
mod ui_util;

pub(crate) use self::args::{DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME};
pub use self::errors::Result as PtuberResult;
pub(crate) use self::ui_util::{get_window_finder, WindowFinderImpl, WindowFinder, WindowFinderError, InputGrabber, InputGrabRunFlag, KeyboardEvent, MouseEvent};
pub(crate) use self::args::{default_config, default_skin_dir};

use self::args::Args;
use self::avatar::PtuberWindow;
use self::config::Config;

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
        let config = Config::new(&args.config_path(), &args.skin_dir());
        let display = PtuberWindow::new(&args.skin_dir(), config.clone())?;
        Ok(Self { config, display })
    }
    pub fn start_ptuber(&mut self) -> PtuberResult<()> {
        debug!("Current Config: {:?}", self.config);
        self.display.display()?;
        Ok(())
    }

    fn parse_args() -> Args {
        Args::parse()
    }
}
