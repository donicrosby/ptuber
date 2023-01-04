use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_yaml;
use sfml::system::{Vector2, Vector2f};
use sfml::window::VideoMode;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use sfml::graphics::Color as SfmlColor;
use crate::{default_config, default_skin_dir};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(skip)]
    pub config_path: PathBuf,
    #[serde(skip)]
    pub images_path: PathBuf,
    pub window: WindowDimensions,
    pub background: Color,
    pub debug: bool,
    pub anchors: Anchors,
    pub mouse_mark: MouseMark,
    #[serde(with = "VectorDef")]
    pub mouse_scale: Vector2f,
}

impl Config {
    pub fn new(config_path: &Path, images_path: &Path) -> Self {
        Self::load_config_from_file(config_path, images_path)
    }

    fn set_paths_in_config(config: &mut Self, config_path: &Path, images_path: &Path) {
        config.config_path = PathBuf::from(config_path);
        config.images_path = PathBuf::from(images_path);
    }

    fn load_config_from_file(config_path: &Path, images_path: &Path) -> Self {
        let config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_path);
        match config_file {
            Ok(file) => {
                let mut config_reader = BufReader::new(&file);
                let mut config_string = String::new();
                let _bytes_read = config_reader
                    .read_to_string(&mut config_string)
                    .unwrap_or(0);
                if config_string.is_empty() {
                    info!("Config does not exist, creating it now...");
                    let mut default = Self {
                        ..Default::default()
                    };
                    Self::set_paths_in_config(&mut default, config_path, images_path);
                    let mut config_writer = BufWriter::new(&file);
                    let _bytes_written = config_writer
                        .write(serde_yaml::to_string(&default).unwrap().as_bytes())
                        .unwrap_or(0);
                    default
                } else {
                    let mut config = serde_yaml::from_str(&config_string).map_err(|_err| warn!("Could not parse config, using defaults...")).unwrap_or_default();
                    Self::set_paths_in_config(&mut config, config_path, images_path);
                    config
                }
            }
            Err(_err) => {
                let mut default = Default::default();
                Self::set_paths_in_config(&mut default, config_path, images_path);
                default
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let images_path = PathBuf::from(default_skin_dir());
        let config_path = PathBuf::from(default_config());
        let window = Default::default();
        let background = Default::default();
        let debug = false;
        let anchors = Default::default();
        let mouse_mark = Default::default();
        let mouse_scale = Vector2f::new(1.0, 1.0);
        Self { config_path, images_path, window, background, debug, anchors, mouse_mark, mouse_scale }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

impl From<WindowDimensions> for VideoMode {
    fn from(value: WindowDimensions) -> Self {
        VideoMode::new(value.width, value.height, 32)
    }
}

impl Default for WindowDimensions {
    fn default() -> Self {
        Self {
            width: 612,
            height: 467,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(remote = "Vector2")]
struct VectorDef<S> {
    x: S,
    y: S,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<Color> for SfmlColor {
    fn from(value: Color) -> Self {
        SfmlColor::rgba(value.red, value.green, value.blue, value.alpha)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Anchors {
    #[serde(with = "VectorDef")]
    pub anchor: Vector2f,
    #[serde(with = "VectorDef")]
    pub arm_offset: Vector2f,
}

impl Default for Anchors {
    fn default() -> Self {
        Self {
            anchor: Vector2f::new(195.0, 240.0),
            arm_offset: Vector2f::new(67.0, 0.0),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MouseMark {
    #[serde(with = "VectorDef")]
    pub position: Vector2f,
    #[serde(with = "VectorDef")]
    pub size: Vector2f,
    pub rotation: f32,
}

impl Default for MouseMark {
    fn default() -> Self {
        Self {
            position: Vector2f::new(40.0, 290.0),
            size: Vector2f::new(180.0, 90.0),
            rotation: 15.0,
        }
    }
}
