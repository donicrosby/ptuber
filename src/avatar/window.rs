use super::{Avatar};
use crate::PtuberResult;
use crate::{Config, DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME, MAX_FRAMERATE};
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::window::{Event, Style, Key};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use log::debug;

#[derive(Debug)]
pub struct PtuberWindow<'a> {
    window: RenderWindow,
    avatar: Avatar<'a>,
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
        window.set_framerate_limit(MAX_FRAMERATE);
        let avatar = Avatar::new(skin_path, config)?;
        Ok(Self {
            window,
            avatar
        })
    }

    pub fn display(&mut self) -> PtuberResult<()> {
        let mut reload_config = false;
        let mut background_color = self.avatar.config().background.clone();
        self.avatar.start_input_grabbing();
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => {
                        self.window.close();
                        debug!("Stopping input grabbing thread!");
                        self.avatar.stop_input_grabbing();
                        debug!("Stopped input grabbing!");
                        return Ok(());
                    },
                    Event::KeyPressed { code , alt: _alt, ctrl, shift: _shift, system: _system } => {
                        if code == Key::R && ctrl {
                            reload_config = true;
                        }
                    }
                    _ => {}
                }
            }
            if reload_config {
                let old_config = self.avatar.config();
                let new_config = Config::new(&old_config.config_path, &old_config.images_path);
                background_color = new_config.background.clone();
                self.avatar.update_config(new_config);
            }

            self.window.clear(background_color.clone().into());
            self.avatar
                .draw(&mut self.window)?;

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
