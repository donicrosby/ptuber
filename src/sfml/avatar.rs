use log::debug;
use sfml::graphics::{
    CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Transform,
    Transformable,
};
use sfml::system::Vector2f;
use std::path::Path;

use super::window::MouseState;
use super::{LeftArmState, TextureContainer};
use crate::errors::Result;
use crate::Config;
use crate::{get_window_finder, WindowFinder};

const TO_DEGREE: f32 = 180.0 / std::f32::consts::PI;

#[derive(Debug, Clone)]
pub(crate) struct Avatar<'a> {
    textures: TextureContainer,
    window_finder: Box<dyn WindowFinder>,
    debug_shapes: DebugShapes<'a>,
    config: Config,
}

#[derive(Debug, Clone, Default)]
struct DebugShapes<'a> {
    pub anchor_mark: CircleShape<'a>,
    pub hand_mark: CircleShape<'a>,
    pub mouse_mark: RectangleShape<'a>,
}

impl<'a> DebugShapes<'a> {
    pub fn setup_debug(&mut self, config: &Config) {
        self.anchor_mark.set_position(config.anchors.anchor);
        self.anchor_mark.set_radius(5.0);
        self.anchor_mark.set_fill_color(Color::BLUE);

        self.hand_mark.set_radius(5.0);
        self.hand_mark.set_fill_color(Color::RED);

        self.mouse_mark.set_fill_color(Color::TRANSPARENT);
        self.mouse_mark.set_outline_color(Color::YELLOW);
        self.mouse_mark.set_outline_thickness(2.0);
        self.mouse_mark.set_position(config.mouse_mark.position);
        self.mouse_mark.set_size(config.mouse_mark.size);
        self.mouse_mark.set_rotation(config.mouse_mark.rotation);
    }
}

impl<'a> Avatar<'a> {
    pub fn new(image_path: &Path, config: Config) -> Result<Self> {
        let textures = TextureContainer::new(&image_path)?;
        let window_finder = get_window_finder()?;
        let mut debug_shapes: DebugShapes = Default::default();
        debug_shapes.setup_debug(&config);
        Ok(Self {
            textures,
            window_finder,
            debug_shapes,
            config,
        })
    }

    pub fn background_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.background)
    }

    pub fn right_arm_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.arms.right)
    }

    pub fn left_arm_left_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.arms.left.left)
    }

    pub fn left_arm_right_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.arms.left.right)
    }

    pub fn left_arm_up_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.arms.left.up)
    }

    pub fn avatar_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.avatar)
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

    fn setup_device(&self, mouse_state: &MouseState) -> Sprite<'_> {
        let mut device = match mouse_state {
            MouseState::None => self.mouse_sprite(),
            MouseState::Left => self.mouse_l_sprite(),
            MouseState::Right => self.mouse_r_sprite(),
            MouseState::Both => self.mouse_lr_sprite()
        };
        let device_scale = self.config.mouse_scale;
        device.set_scale(device_scale);
        let bounds = device.local_bounds();
        device.set_origin(Vector2f::new(bounds.width / 2.0, bounds.height / 2.0));

        device
    }

    fn draw_arm(&mut self, window: &mut RenderWindow, mouse_state: &MouseState) -> Result<()> {
        let mouse_pos = self.window_finder.get_cursor_position()?;
        debug!("Mouse Pos{{ X: {}, Y: {} }}", mouse_pos.x, mouse_pos.y);

        let mut transform: Transform = Default::default();

        let mouse_mark_pos = self.debug_shapes.mouse_mark.position();
        let mouse_mark_size = self.debug_shapes.mouse_mark.size();

        transform.translate(mouse_mark_pos.x, mouse_mark_pos.y);
        transform.rotate(self.config.mouse_mark.rotation);
        transform.scale(mouse_mark_size.x, mouse_mark_size.y);

        let hand_pos = transform.transform_point(mouse_pos.clone());
        debug!("Hand Pos{{ X: {}, Y: {} }}", hand_pos.x, hand_pos.y);

        self.debug_shapes.hand_mark.set_position(hand_pos);

        let arm_origin = self.config.anchors.arm_offset;
        let mut arm = self.right_arm_sprite();
        arm.set_origin(arm_origin);
        arm.set_position(self.config.anchors.anchor);

        let displacement = hand_pos - self.debug_shapes.anchor_mark.position();
        let dist = displacement.x.hypot(displacement.y);
        let arm_bounds = arm.local_bounds().clone();
        let scale = dist / arm_bounds.height;

        arm.set_scale((1.0, scale));

        let alpha = (-displacement.x / dist).asin();
        let deg = alpha * TO_DEGREE;

        arm.set_rotation(deg);
        let mut device = self.setup_device(mouse_state);
        device.set_position(hand_pos);
        window.draw(&device);
        window.draw(&arm);

        Ok(())
    }

    fn draw_debug(&mut self, window: &mut RenderWindow) {
        window.draw(&self.debug_shapes.hand_mark);
        window.draw(&self.debug_shapes.anchor_mark);
        window.draw(&self.debug_shapes.mouse_mark);
    }

    fn draw_left_arm(&mut self, window: &mut RenderWindow, left_arm_state: &LeftArmState) {
        let sprite;
        match left_arm_state {
            LeftArmState::Up => {
                sprite = self.left_arm_up_sprite();
            }
            LeftArmState::Left => {
                sprite = self.left_arm_left_sprite();
            }
            LeftArmState::Right => {
                sprite = self.left_arm_right_sprite();
            }
        }
        window.draw(&sprite)
    }

    pub fn draw(&mut self, window: &mut RenderWindow, left_arm_state: &LeftArmState, mouse_state: &MouseState) -> Result<()> {
        {
            let bg = self.background_sprite();
            window.draw(&bg);
        }

        self.draw_arm(window, mouse_state)?;

        {
            let avatar = self.avatar_sprite();
            window.draw(&avatar);
        }

        self.draw_left_arm(window, left_arm_state);

        if self.config.debug {
            self.draw_debug(window);
        }

        Ok(())
    }
}
