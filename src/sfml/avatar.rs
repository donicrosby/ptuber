use std::path::{PathBuf, Path};
use log::debug;
use sfml::graphics::{Texture, Sprite, RenderWindow, RenderTarget};
use sfml::SfBox;
use crate::{WindowFinder, get_window_finder};
use crate::errors::Result;

use super::{SfmlError, SfmlResult};


const UP_IMAGE: &'static str = "up.png";
const DOWN_IMAGE: &'static str = "down.png";
const BACKGROUND_IMAGE: &'static str = "bg.png";
const MOUSE: &'static str = "mouse.png";
const MOUSE_L: &'static str = "mousel.png";
const MOUSE_R: &'static str = "mouser.png";
const MOUSE_LR: &'static str = "mouselr.png";

#[derive(Debug, Clone)]
pub(crate) struct TextureContainer {
    pub up: SfBox<Texture>,
    pub down: SfBox<Texture>,
    pub background: SfBox<Texture>,
    pub mouse: MouseTextures
}

impl TextureContainer {
    pub fn new(image_path: &Path) -> SfmlResult<Self> {
        let image_path = PathBuf::from(image_path);

        let mut up_path = image_path.clone();
        up_path.push(UP_IMAGE);
        let up = Texture::from_file(&up_path.to_str().ok_or(SfmlError::PathConversion)?)?;
        
        let mut down_path = image_path.clone();
        down_path.push(DOWN_IMAGE);
        let down = Texture::from_file(&down_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mut background_path = image_path.clone();
        background_path.push(BACKGROUND_IMAGE);
        let background = Texture::from_file(&background_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mouse = MouseTextures::new(&image_path)?;

        Ok(Self {
            up,
            down,
            background,
            mouse
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MouseTextures {
    pub mouse: SfBox<Texture>,
    pub mouse_l: SfBox<Texture>,
    pub mouse_r: SfBox<Texture>,
    pub mouse_lr: SfBox<Texture>,
}

impl MouseTextures {
    pub fn new(image_path: &Path) -> SfmlResult<Self> {
        let image_path = PathBuf::from(image_path);

        let mut mouse_path = image_path.clone();
        mouse_path.push(MOUSE);
        let mouse = Texture::from_file(&mouse_path.to_str().ok_or(SfmlError::PathConversion)?)?;
        
        let mut mouse_l_path = image_path.clone();
        mouse_l_path.push(MOUSE_L);
        let mouse_l = Texture::from_file(&mouse_l_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mut mouse_r_path = image_path.clone();
        mouse_r_path.push(MOUSE_R);
        let mouse_r = Texture::from_file(&mouse_r_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mut mouse_lr_path = image_path.clone();
        mouse_lr_path.push(MOUSE_LR);
        let mouse_lr = Texture::from_file(&mouse_lr_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        Ok(Self {
            mouse,
            mouse_l,
            mouse_r,
            mouse_lr
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Avatar {
    textures: TextureContainer,
    window_finder: Box<dyn WindowFinder>
}

impl Avatar {
    pub fn new(image_path: &Path) -> Result<Self> {
        let textures = TextureContainer::new(&image_path)?;
        let window_finder = get_window_finder()?;
        Ok(Self {
            textures,
            window_finder
        })
    }

    pub fn background_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.background)
    }

    pub fn up_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.up)
    }

    pub fn down_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.down)
    }

    pub fn mouse_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse)
    }

    pub fn mouse_l_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse_l)
    }

    pub fn mouse_r_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse_r)
    }

    pub fn mouse_lr_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse_lr)
    }

    pub fn draw(&self, window: &mut RenderWindow) -> Result<()> {
        let bg = self.background_sprite();
        let window_dimensions = self.window_finder.get_focused_window_size()?;
        let cursor_pos = self.window_finder.get_cursor_position();
        let screen_size = self.window_finder.get_focused_screen_size()?;
        debug!("Focused Window Dimensions{{ Width: {x}, Height {y} }}", x=window_dimensions.x, y=window_dimensions.y);
        debug!("Mouse Pos{{ Width: {x}, Height {y} }}", x=cursor_pos.x, y=cursor_pos.y);
        debug!("Screen Size{{ Width: {x}, Height {y} }}", x=screen_size.x, y=screen_size.y);

        window.draw(&bg);
        Ok(())
    }
    
}