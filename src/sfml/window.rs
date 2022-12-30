use sfml::window::{Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH, MAX_FRAMERATE, DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME, Config};
use std::path::{PathBuf, MAIN_SEPARATOR, Path};
use super::{Avatar};
use crate::PtuberResult;

#[derive(Debug)]
pub struct PtuberWindow {
    window: RenderWindow,
    avatar: Avatar,
}

impl PtuberWindow {
    pub fn new(skin_path: &Path)-> PtuberResult<Self> {
        let mut window = RenderWindow::new((WINDOW_WIDTH, WINDOW_HEIGHT), "Ptuber Rigger!", Style::TITLEBAR | Style::CLOSE, &Default::default());
        window.set_framerate_limit(MAX_FRAMERATE);
        let avatar = Avatar::new(skin_path)?;
        Ok(Self {
            window,
            avatar
        })
    }

    pub fn display(mut self, config: &Config) -> PtuberResult<()> {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => {
                        self.window.close();
                        return Ok(())
                    },
                    _ => {

                    }
                }
            }

            self.window.clear(config.background.clone().into());
            self.avatar.draw(&mut self.window, &config)?;

            self.window.display();
        }
        Ok(())
    }
}

impl Default for PtuberWindow {
    fn default() -> Self {
        Self::new(&PathBuf::from(format!(".{}{}{}{}", MAIN_SEPARATOR, DEFAULT_SKIN_DIR_NAME, MAIN_SEPARATOR, DEFAULT_CONFIG_NAME))).unwrap()
    }
}