use rgb::RGBA8;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

use sfml::graphics::Color;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {
    pub mouse: Mouse,
    pub background: Background,
    pub flipper: Flipper,
}

impl Config {
    pub fn new(config_path: &Path) -> Self {
        let config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_path);
        match config_file {
            Ok(file) => {
                let mut config_reader = BufReader::new(&file);
                let mut config_string = String::new();
                let _bytes_read= config_reader.read_to_string(&mut config_string).unwrap_or(0);
                if config_string.is_empty() {
                    let default = Self {
                        ..Default::default()
                    };
                    let mut config_writer = BufWriter::new(&file);
                    let _bytes_written = config_writer.write(serde_yaml::to_string(&default).unwrap().as_bytes()).unwrap_or(0);
                    default
                } else {
                    serde_yaml::from_str(&config_string).unwrap_or(Default::default())
                }
            }
            Err(_err) => {
                Default::default()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mouse {
    pub x_offset: u32,
    pub y_offset: u32,
    pub scalar: f32,
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse {
            x_offset: 10,
            y_offset: 5,
            scalar: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RGBAWrapper(RGBA8);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Background {
    #[serde(flatten)]
    pub color: RGBAWrapper,
}

impl Into<Color> for Background {
    fn into(self) -> Color {
        self.color.into()
    }
}

impl Into<Color> for RGBAWrapper {
    fn into(self) -> Color {
        Color::rgba(self.0.r, self.0.g, self.0.b, self.0.a)
    }
}

impl Default for Background {
    fn default() -> Self {
        Background {
            color: RGBAWrapper(RGBA8::new(255, 255, 255, 255)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flipper {
    pub base: RGBAWrapper,
    pub edge: RGBAWrapper,
}

impl Default for Flipper {
    fn default() -> Self {
        Flipper {
            base: RGBAWrapper(RGBA8::new(0, 0, 0, 255)),
            edge: RGBAWrapper(RGBA8::new(0, 0, 0, 255)),
        }
    }
}
