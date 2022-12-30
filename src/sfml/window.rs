use super::Avatar;
use crate::PtuberResult;
use crate::{Config, DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME, MAX_FRAMERATE};
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::window::mouse::Button;
use sfml::window::{Event, Style};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

#[derive(Debug)]
pub struct PtuberWindow<'a> {
    window: RenderWindow,
    avatar: Avatar<'a>,
}

pub enum LeftArmState {
    Left,
    Right,
    Up,
}

enum LastArmState {
    Left,
    Right,
}

pub enum MouseState {
    None,
    Left,
    Right,
    Both,
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
        Ok(Self { window, avatar })
    }

    pub fn display(mut self, config: &Config) -> PtuberResult<()> {
        let mut last_state = LastArmState::Right;
        let mut left_arm_state = LeftArmState::Up;
        let mut keys_pressed = 0;
        let mut mouse_state = MouseState::None;
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => {
                        self.window.close();
                        return Ok(());
                    }
                    Event::KeyPressed { .. } => {
                        keys_pressed += 1;
                        match left_arm_state {
                            LeftArmState::Left => {
                                left_arm_state = LeftArmState::Right;
                            }
                            LeftArmState::Right => {
                                left_arm_state = LeftArmState::Left;
                            }
                            LeftArmState::Up => match last_state {
                                LastArmState::Left => left_arm_state = LeftArmState::Right,
                                LastArmState::Right => left_arm_state = LeftArmState::Left,
                            },
                        }
                    }
                    Event::KeyReleased { .. } => {
                        if keys_pressed <= 0 {
                            keys_pressed = 0
                        } else {
                            keys_pressed -= 1;
                        }
                        match left_arm_state {
                            LeftArmState::Left => {
                                if keys_pressed == 0 {
                                    left_arm_state = LeftArmState::Up;
                                } else {
                                    left_arm_state = LeftArmState::Right;
                                }
                            }
                            LeftArmState::Right => {
                                if keys_pressed == 0 {
                                    left_arm_state = LeftArmState::Up;
                                } else {
                                    left_arm_state = LeftArmState::Left;
                                }
                            }
                            LeftArmState::Up => match last_state {
                                LastArmState::Left => left_arm_state = LeftArmState::Right,
                                LastArmState::Right => left_arm_state = LeftArmState::Left,
                            },
                        }
                    }
                    Event::MouseButtonPressed { button, .. } => match button {
                        Button::Left => match mouse_state {
                            MouseState::Left | MouseState::None => {
                                mouse_state = MouseState::Left;
                            }
                            MouseState::Right | MouseState::Both => mouse_state = MouseState::Both,
                        },
                        Button::Right => match mouse_state {
                            MouseState::Right | MouseState::None => {
                                mouse_state = MouseState::Right;
                            }
                            MouseState::Left | MouseState::Both => mouse_state = MouseState::Both,
                        },
                        _ => {}
                    },
                    Event::MouseButtonReleased { button, .. } => match button {
                        Button::Left => match mouse_state {
                            MouseState::Left => {
                                mouse_state = MouseState::None;
                            }
                            MouseState::Both => {
                                mouse_state = MouseState::Right;
                            }
                            MouseState::None | MouseState::Right => {}
                        },
                        Button::Right => match mouse_state {
                            MouseState::Right => {
                                mouse_state = MouseState::None;
                            }
                            MouseState::Both => {
                                mouse_state = MouseState::Left;
                            }
                            MouseState::None | MouseState::Left => {}
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }

            self.window.clear(config.background.clone().into());
            self.avatar
                .draw(&mut self.window, &left_arm_state, &mouse_state)?;

            self.window.display();

            match last_state {
                LastArmState::Left => last_state = LastArmState::Right,
                LastArmState::Right => last_state = LastArmState::Left,
            }
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
