use log::{info, warn};
use serde::{Deserialize, Serialize};
use toml;
use sfml::system::Vector2;
use sfml::window::VideoMode;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use either::Either;

use sfml::graphics::Color as SfmlColor;
use crate::{default_config, default_skin_dir};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(skip)]
    pub config_path: PathBuf,
    #[serde(skip)]
    pub images_path: PathBuf,
    pub debug: bool,
    pub window: WindowDimensions,
    pub background: Color,
    #[serde(with = "VectorDef")]
    pub mouse_scale: Vector2<IntOrFloat>,
    pub anchors: Anchors,
    pub mouse_mark: MouseMark,
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
                        .write(toml::to_string(&default).unwrap().as_bytes())
                        .unwrap_or(0);
                    default
                } else {
                    let mut config = toml::from_str(&config_string).map_err(|_err| warn!("Could not parse config, using defaults...")).unwrap_or_default();
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
        let mouse_scale = Vector2::new(1.into(), 1.into());
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct IntOrFloat {
    #[serde(with = "either::serde_untagged")]
    inner: Either<f32, usize>
}

impl From<IntOrFloat> for f32 {
    fn from(value: IntOrFloat) -> Self {
        match value.inner {
            Either::Left(float) => float,
            Either::Right(int) => int as f32
        }
    }
}

impl From<f32> for IntOrFloat {
    fn from(value: f32) -> Self {
        let inner = Either::Left(value);
        Self {
            inner
        }
    }
}

impl From<usize> for IntOrFloat {
    fn from(value: usize) -> Self {
        let inner = Either::Right(value);
        Self {
            inner
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
    pub anchor: Vector2<IntOrFloat>,
    #[serde(with = "VectorDef")]
    pub arm_offset: Vector2<IntOrFloat>,
}

impl Default for Anchors {
    fn default() -> Self {
        Self {
            anchor: Vector2::new(195.into(), 240.into()),
            arm_offset: Vector2::new(67.into(), 0.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MouseMark {
    pub rotation: IntOrFloat,
    #[serde(with = "VectorDef")]
    pub position: Vector2<IntOrFloat>,
    #[serde(with = "VectorDef")]
    pub size: Vector2<IntOrFloat>,
}

impl Default for MouseMark {
    fn default() -> Self {
        Self {
            position: Vector2::new(40.into(), 290.into()),
            size: Vector2::new(180.into(), 90.into()),
            rotation: 15.into(),
        }
    }
}
