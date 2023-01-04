use sfml::graphics::{
    RenderTarget, 
    RenderWindow,
    Sprite,
};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};

use super::{TextureContainer, Arms};
use crate::errors::Result;
use crate::Config;
use crate::{get_window_finder, WindowFinder};
use crate::{InputGrabber, InputGrabRunFlag};


#[derive(Debug)]
pub(crate) struct Avatar<'a> {
    textures: TextureContainer,
    arms: Arms<'a>,
    window_finder: Box<dyn WindowFinder>,
    config: Config,
    input_grabber: InputGrabber,
    shutdown_tx: Sender<InputGrabRunFlag>
}


impl<'a> Avatar<'a> {
    pub fn new(image_path: &Path, config: Config) -> Result<Self> {
        let textures = TextureContainer::new(image_path)?;
        let (mouse_tx, mouse_rx) = channel();
        let (keyboard_tx, keyboard_rx) = channel();
        let (shutdown_tx, shutdown_rx) = channel();
        let mut input_grabber = InputGrabber::new();
        input_grabber.start(shutdown_rx, mouse_tx, keyboard_tx);
        let window_finder = get_window_finder()?;
        let arms = Arms::new(image_path, &config, keyboard_rx, mouse_rx)?;
        
        Ok(Self {
            textures,
            window_finder,
            arms,
            config,
            input_grabber,
            shutdown_tx
        })
    }

    pub fn update_config(&mut self, config: Config) {
        self.arms.update_config(&config);
        self.config = config;
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

    pub fn start_input_grabbing(&self) {
        self.shutdown_tx.send(InputGrabRunFlag::Run).expect("Could not start input grabber");
    }

    pub fn stop_input_grabbing(&mut self) {
        self.shutdown_tx.send(InputGrabRunFlag::Halt).expect("Could not shutdown input grabber");
        self.input_grabber.shutdown();
    }

    pub fn draw(
        &mut self,
        window: &mut RenderWindow,
    ) -> Result<()> {
        {
            let bg = self.background_sprite();
            window.draw(&bg);
        }
        let mouse_pos = self.window_finder.get_cursor_position()?;
        self.arms.draw_right_arm(mouse_pos, window);

        {
            let avatar = self.avatar_sprite();
            window.draw(&avatar);
        }

        self.arms.draw_left_arm(window);

        if self.config.debug {
            self.arms.draw_debug(window);
        }

        Ok(())
    }
}
