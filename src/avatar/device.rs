use log::warn;
use sfml::graphics::{
    Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Transform, Transformable,
};
use sfml::system::Vector2f;
use std::path::Path;
use std::sync::mpsc::{Receiver, TryRecvError};

use super::{MouseTextures, SfmlResult, TextureContainer};
use crate::errors::Result;
use crate::{Config, MouseEvent};

#[derive(Debug, Copy, Clone)]
pub enum MouseState {
    None,
    Left,
    Right,
    Both,
}

#[derive(Debug)]
pub struct Device<'a> {
    textures: MouseTextures,
    mouse_scale: Vector2f,
    mouse_mark: RectangleShape<'a>,
    mouse_rotation: f32,
    mouse_rx: Receiver<MouseEvent>,
    mouse_state: MouseState,
}

impl<'a> Device<'a> {
    pub fn new(
        images_path: &Path,
        config: &Config,
        mouse_rx: Receiver<MouseEvent>,
    ) -> SfmlResult<Self> {
        let textures = MouseTextures::new(images_path)?;
        let mouse_scale = config.mouse_scale.into_other();
        let mouse_mark = Self::setup_debug(config);
        let mouse_rotation = config.mouse_mark.rotation.into();
        let mouse_state = MouseState::None;
        Ok(Self {
            textures,
            mouse_scale,
            mouse_mark,
            mouse_rotation,
            mouse_rx,
            mouse_state,
        })
    }

    fn setup_debug(config: &Config) -> RectangleShape<'a> {
        let mut mouse_mark = RectangleShape::default();
        mouse_mark.set_fill_color(Color::TRANSPARENT);
        mouse_mark.set_outline_color(Color::YELLOW);
        mouse_mark.set_outline_thickness(2.0);
        mouse_mark.set_position(config.mouse_mark.position.into_other());
        mouse_mark.set_size(config.mouse_mark.size.into_other());
        mouse_mark.set_rotation(config.mouse_mark.rotation.into());

        mouse_mark
    }

    pub fn update_config(&mut self, config: &Config) -> Result<()> {
        self.mouse_scale = config.mouse_scale.into_other();
        self.mouse_rotation = config.mouse_mark.rotation.into();
        self.textures.reload_textures(&config.images_path)?;
        self.mouse_mark = Self::setup_debug(config);
        Ok(())
    }

    pub fn mouse_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse)
    }

    pub fn mouse_l_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse_l)
    }

    pub fn mouse_r_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse_r)
    }

    pub fn mouse_lr_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse_lr)
    }

    pub fn get_hand_transform(&mut self) -> Transform {
        let mut transform: Transform = Default::default();

        let mouse_mark_pos = self.mouse_mark.position();
        let mouse_mark_size = self.mouse_mark.size();

        transform.translate(mouse_mark_pos.x, mouse_mark_pos.y);
        transform.rotate(self.mouse_rotation);
        transform.scale(mouse_mark_size.x, mouse_mark_size.y);

        transform
    }

    fn setup_device(&self, mouse_state: &MouseState) -> Sprite<'_> {
        let mut device = match mouse_state {
            MouseState::None => self.mouse_sprite(),
            MouseState::Left => self.mouse_l_sprite(),
            MouseState::Right => self.mouse_r_sprite(),
            MouseState::Both => self.mouse_lr_sprite(),
        };
        let device_scale = self.mouse_scale;
        device.set_scale(device_scale);
        let bounds = device.local_bounds();
        device.set_origin(Vector2f::new(bounds.width / 2.0, bounds.height / 2.0));

        device
    }

    fn get_mouse_state(&mut self) -> MouseState {
        loop {
            match self.mouse_rx.try_recv() {
                Ok(event) => match event {
                    MouseEvent::LeftPressed => match self.mouse_state {
                        MouseState::Left | MouseState::None => {
                            self.mouse_state = MouseState::Left;
                        }
                        MouseState::Right | MouseState::Both => {
                            self.mouse_state = MouseState::Both;
                        }
                    },
                    MouseEvent::RightPressed => match self.mouse_state {
                        MouseState::Left | MouseState::Both => {
                            self.mouse_state = MouseState::Both;
                        }
                        MouseState::Right | MouseState::None => {
                            self.mouse_state = MouseState::Right;
                        }
                    },
                    MouseEvent::LeftReleased => match self.mouse_state {
                        MouseState::Left | MouseState::None => {
                            self.mouse_state = MouseState::None;
                        }
                        MouseState::Right | MouseState::Both => {
                            self.mouse_state = MouseState::Right;
                        }
                    },
                    MouseEvent::RightReleased => match self.mouse_state {
                        MouseState::Left | MouseState::Both => {
                            self.mouse_state = MouseState::Left;
                        }
                        MouseState::Right | MouseState::None => {
                            self.mouse_state = MouseState::None;
                        }
                    },
                    _ => {}
                },
                Err(err) => {
                    if err == TryRecvError::Disconnected {
                        warn!("Mouse input channel disconnected!");
                    }
                    break;
                }
            }
        }
        self.mouse_state
    }

    pub fn draw(&mut self, hand_pos: Vector2f, window: &mut RenderWindow) {
        let state = self.get_mouse_state();
        let mut device = self.setup_device(&state);
        device.set_position(hand_pos);
        window.draw(&device)
    }

    pub fn draw_debug(&self, window: &mut RenderWindow) {
        window.draw(&self.mouse_mark)
    }
}
