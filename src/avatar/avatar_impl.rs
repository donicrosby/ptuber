use sfml::graphics::{RenderTarget, RenderWindow, Sprite};
use std::path::Path;

use super::{Arms, AvatarTextures, TextureContainer};
use crate::errors::Result;
use crate::view_models::{DeviceViewModelImpl, KeyboardViewModelImpl};
use crate::Config;

#[derive(Debug)]
pub(crate) struct Avatar<'a> {
    textures: AvatarTextures,
    arms: Arms<'a>,
    config: Config,
}

impl<'a> Avatar<'a> {
    pub fn new(image_path: &Path, config: Config) -> Result<Self> {
        let textures = AvatarTextures::new(image_path)?;
        let arms = Arms::new(image_path, &config)?;

        Ok(Self {
            textures,
            arms,
            config,
        })
    }

    pub fn update_config(&mut self, config: Config) -> Result<()> {
        //self.textures.reload_textures(&config.images_path)?;
        self.arms.update_config(&config)?;
        self.config = config;
        Ok(())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn background_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.background)
    }

    pub fn avatar_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.avatar)
    }

    pub fn draw(
        &mut self,
        window: &mut RenderWindow,
        keyboard: &KeyboardViewModelImpl,
        mouse: &DeviceViewModelImpl,
    ) -> Result<()> {
        {
            let bg = self.background_sprite();
            window.draw(&bg);
        }
        let mouse_pos = mouse.position();
        if self.config.avatar_below_arm {
            {
                let avatar = self.avatar_sprite();
                window.draw(&avatar);
            }
            self.arms.draw_right_arm(mouse_pos, window, mouse);
        } else {
            self.arms.draw_right_arm(mouse_pos, window, mouse);

            {
                let avatar = self.avatar_sprite();
                window.draw(&avatar);
            }
        }

        self.arms.draw_left_arm(window, keyboard);

        if self.config.debug {
            self.arms.draw_debug(window);
        }

        Ok(())
    }
}
