use super::Avatar;
use crate::user_input::UserInputMonitor;
use crate::{Config, DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME, MAX_FRAMERATE};
use crate::{DeviceViewModelImpl, KeyboardViewModelImpl, PTuberError, PtuberResult};
use log::debug;
use rust_embed::RustEmbed;
use sfml::graphics::{Image, RenderTarget, RenderWindow};
use sfml::window::{Event, Key, Style};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

const EMBEDDED_ICON_PATH: &str = "icon.png";

#[derive(RustEmbed)]
#[folder = "assets/"]
#[include = "*.png"]
struct Assets;

#[derive(Debug)]
pub struct PtuberWindow<'a> {
    window: RenderWindow,
    avatar: Avatar<'a>,
    icon: Image,
}

// fn is_left_key(key: Key) -> bool {
//     match key {
//         Key::Tilde | Key::Num1 | Key::Q | Key::A | Key::Z | Key::Num2 | Key::W | Key::S | Key::X => {
//             true
//         },
//         _ => false
//     }
// }

// fn is_right_key(key: Key) -> bool {
//     match key {
//         Key::Num0 | Key::P => {
//             true
//         },
//         _ => false
//     }
// }

impl<'a> PtuberWindow<'a> {
    pub fn new(skin_path: &Path, config: Config) -> PtuberResult<Self> {
        let mut window = RenderWindow::new(
            config.window.clone(),
            "Ptuber Rigger!",
            Style::TITLEBAR | Style::CLOSE,
            &Default::default(),
        );
        let icon_bytes = Assets::get(EMBEDDED_ICON_PATH).ok_or(PTuberError::AssetGet)?;
        debug!("Icon Bytes: {}", icon_bytes.data.len());
        let icon = Image::from_memory(&icon_bytes.data).ok_or(PTuberError::AssetLoad)?;
        window.set_framerate_limit(MAX_FRAMERATE);
        let avatar = Avatar::new(skin_path, config)?;
        Ok(Self {
            window,
            avatar,
            icon,
        })
    }

    pub fn display(
        &mut self,
        keyboard: &KeyboardViewModelImpl,
        mouse: &DeviceViewModelImpl,
        events: &mut UserInputMonitor,
    ) -> PtuberResult<()> {
        let mut reload_config = false;
        let mut background_color = self.avatar.config().background.clone();
        let icon_size = self.icon.size();
        unsafe {
            self.window
                .set_icon(icon_size.x, icon_size.y, self.icon.pixel_data());
        }
        while self.window.is_open() {
            events.get_events();
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => {
                        self.window.close();
                        return Ok(());
                    }
                    Event::KeyPressed {
                        code,
                        alt: _alt,
                        ctrl,
                        shift: _shift,
                        system: _system,
                    } => {
                        if code == Key::R && ctrl {
                            reload_config = true;
                        }
                    }
                    _ => {}
                }
            }

            if reload_config {
                let old_config = self.avatar.config();
                //let new_config = Config::new(&old_config.config_path, &old_config.images_path);
                //events.update_config(&new_config);
                //background_color = new_config.background.clone();
                //self.avatar.update_config(new_config)?;
                reload_config = false;
            }

            self.window.clear(background_color.clone().into());
            self.avatar.draw(&mut self.window, keyboard, mouse)?;
            self.window.display();
        }
        Ok(())
    }
}

impl<'a> Default for PtuberWindow<'a> {
    fn default() -> Self {
        let default_config: Config = Default::default();
        Self::new(
            &PathBuf::from(format!(
                ".{}{}{}{}",
                MAIN_SEPARATOR, DEFAULT_SKIN_DIR_NAME, MAIN_SEPARATOR, DEFAULT_CONFIG_NAME
            )),
            default_config,
        )
        .unwrap()
    }
}
