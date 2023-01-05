use device_query::Keycode;
use log::trace;
use sfml::graphics::{
    CircleShape, Color, RenderTarget, RenderWindow, Shape, Sprite, Transformable,
};
use sfml::system::Vector2f;
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc::Receiver;

use super::{ArmTextures, Device, SfmlResult, TextureContainer};
use crate::errors::Result;
use crate::Config;
use crate::{KeyboardEvent, MouseEvent};

const TO_DEGREE: f32 = 180.0 / std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum LeftArmState {
    Left,
    Right,
    Up,
}

#[derive(Debug)]
pub struct Arms<'a> {
    textures: ArmTextures,
    device: Device<'a>,
    arm_offset: Vector2f,
    anchor: Vector2f,
    hand_mark: CircleShape<'a>,
    anchor_mark: CircleShape<'a>,
    keyboard_rx: Receiver<KeyboardEvent>,
    keys_currently_pressed: HashSet<Keycode>,
    left_arm_state: LeftArmState,
}

impl<'a> Arms<'a> {
    pub fn new(
        images_path: &Path,
        config: &Config,
        keyboard_rx: Receiver<KeyboardEvent>,
        mouse_rx: Receiver<MouseEvent>,
    ) -> SfmlResult<Self> {
        let textures = ArmTextures::new(images_path)?;
        let device = Device::new(images_path, config, mouse_rx)?;
        let (anchor_mark, hand_mark) = Self::setup_debug(config);
        let arm_offset = config.anchors.arm_offset.into_other();
        let anchor = config.anchors.anchor.into_other();
        let keys_currently_pressed = HashSet::new();
        let left_arm_state = LeftArmState::Up;
        Ok(Self {
            textures,
            device,
            arm_offset,
            anchor,
            anchor_mark,
            hand_mark,
            keyboard_rx,
            keys_currently_pressed,
            left_arm_state,
        })
    }

    fn setup_debug(config: &Config) -> (CircleShape<'a>, CircleShape<'a>) {
        let mut anchor_mark = CircleShape::default();
        let mut hand_mark = CircleShape::default();
        anchor_mark.set_position(config.anchors.anchor.into_other());
        anchor_mark.set_radius(5.0);
        anchor_mark.set_fill_color(Color::BLUE);

        hand_mark.set_radius(5.0);
        hand_mark.set_fill_color(Color::RED);
        (anchor_mark, hand_mark)
    }

    pub fn update_config(&mut self, config: &Config) -> Result<()> {
        self.arm_offset = config.anchors.arm_offset.into_other();
        self.anchor = config.anchors.anchor.into_other();
        let (anchor_mark, hand_mark) = Self::setup_debug(config);
        self.anchor_mark = anchor_mark;
        self.hand_mark = hand_mark;
        self.textures.reload_textures(&config.images_path)?;
        self.device.update_config(config)?;
        Ok(())
    }

    pub fn right_arm_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.right)
    }

    pub fn left_arm_left_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.left.left)
    }

    pub fn left_arm_right_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.left.right)
    }

    pub fn left_arm_up_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.left.up)
    }

    fn get_right_arm(&self, hand_pos: Vector2f) -> Sprite {
        let arm_origin = self.arm_offset;
        let mut arm = self.right_arm_sprite();
        arm.set_origin(arm_origin);
        arm.set_position(self.anchor);

        let displacement = hand_pos - self.anchor_mark.position();
        let dist = displacement.x.hypot(displacement.y);
        let arm_bounds = arm.local_bounds();
        let scale = dist / arm_bounds.height;

        arm.set_scale((1.0, scale));

        let alpha = (-displacement.x / dist).asin();
        let deg = alpha * TO_DEGREE;

        arm.set_rotation(deg);
        arm.clone()
    }

    pub fn draw_right_arm(&mut self, mouse_pos: Vector2f, window: &mut RenderWindow) {
        trace!("Mouse Pos{{ X: {}, Y: {} }}", mouse_pos.x, mouse_pos.y);
        let transform = { self.device.get_hand_transform() };

        let hand_pos = transform.transform_point(mouse_pos);
        trace!("Hand Pos{{ X: {}, Y: {} }}", hand_pos.x, hand_pos.y);

        self.hand_mark.set_position(hand_pos);
        self.device.draw(hand_pos, window);
        window.draw(&self.get_right_arm(hand_pos))
    }

    fn get_left_arm_state(&mut self) -> LeftArmState {
        let mut arm = LeftArmState::Up;
        match self.keyboard_rx.try_recv() {
            Ok(event) => {
                match event {
                    KeyboardEvent::KeyPressed(key) => {
                        self.keys_currently_pressed.insert(key);
                    }
                    KeyboardEvent::KeyReleased(key) => {
                        self.keys_currently_pressed.remove(&key);
                    }
                }
                if !self.keys_currently_pressed.is_empty() {
                    if self.keys_currently_pressed.len() % 2 == 0 {
                        arm = LeftArmState::Left;
                    } else {
                        arm = LeftArmState::Right;
                    }
                }
                self.left_arm_state = arm;
                arm
            }
            Err(_err) => self.left_arm_state,
        }
    }

    pub fn draw_left_arm(&mut self, window: &mut RenderWindow) {
        let sprite = match self.get_left_arm_state() {
            LeftArmState::Up => self.left_arm_up_sprite(),
            LeftArmState::Left => self.left_arm_left_sprite(),
            LeftArmState::Right => self.left_arm_right_sprite(),
        };
        window.draw(&sprite)
    }

    pub fn draw_debug(&self, window: &mut RenderWindow) {
        window.draw(&self.hand_mark);
        window.draw(&self.anchor_mark);
        self.device.draw_debug(window)
    }
}
