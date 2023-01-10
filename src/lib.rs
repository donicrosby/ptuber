use clap::Parser;
use log::debug;
use std::sync::Arc;

pub mod args;
mod avatar;
mod models;
mod user_input;
mod view_models;

pub mod config;
mod errors;
mod os_ui;

use self::user_input::{DeviceEvent, KeyboardEvent, UserInputMonitor, UtilError};
use self::view_models::{
    DeviceViewModelImpl, KeyboardState, KeyboardViewModelImpl, MouseButtonState,
};
use models::{Keyboard, MouseModel};

use self::models::DeviceButton;

pub(crate) use self::args::{default_config, default_skin_dir};
pub(crate) use self::args::{DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME};
pub use self::errors::PTuberError;
pub use self::errors::Result as PtuberResult;
pub(crate) use self::os_ui::{get_window_finder, WindowFinderError, WindowFinderImpl};

use self::args::Args;
use self::avatar::PtuberWindow;
use self::config::Config;

pub const MAX_FRAMERATE: u32 = 60;

pub struct PTuber<'a> {
    config: Config,
    display: PtuberWindow<'a>,
    user_input_monitor: UserInputMonitor,
}

impl<'a> PTuber<'a> {
    pub fn new() -> PtuberResult<Self> {
        let args = Self::parse_args();
        debug!("Skin path: {:?}", args.skin_dir());
        debug!("Config path: {:?}", args.config_path());
        let user_input_monitor = UserInputMonitor::new();
        let config = Config::new(&args.config_path(), &args.skin_dir());
        let display = PtuberWindow::new(&args.skin_dir(), config.clone())?;
        Ok(Self {
            config,
            display,
            user_input_monitor,
        })
    }
    pub fn start_ptuber(&mut self) -> PtuberResult<()> {
        debug!("Current Config: {:?}", self.config);

        let keyboard_viewmodel = Arc::new(KeyboardViewModelImpl::new());
        let device_viewmodel = Arc::new(DeviceViewModelImpl::new());
        
        let keyboard_callback = keyboard_viewmodel.clone();
        let device_callback = device_viewmodel.clone();
        let _device_guard = self
            .user_input_monitor
            .add_device_callback(move |e| device_callback.handle_event(e))
            .expect("adding device callback");
        
        let _keyboard_guard = self
            .user_input_monitor
            .add_keyboard_callback(move |e| keyboard_callback.handle_event(e))
            .expect("adding keyboard callback");

        match self
            .display
            .display(&keyboard_viewmodel, &device_viewmodel, &mut self.user_input_monitor)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn parse_args() -> Args {
        Args::parse()
    }
}
