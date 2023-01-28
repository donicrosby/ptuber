use bevy::{
    prelude::*,
    window::{CreateWindow, PresentMode, WindowId},
};
use bevy_common_assets::toml::TomlAssetPlugin;
use clap::Parser;
use log::debug;
use std::sync::Arc;
use std::time::Duration;

pub mod args;
mod avatar;
mod consts;
mod models;
mod user_input;
mod view_models;

pub mod config;
mod errors;
mod os_ui;

use self::user_input::{DeviceEvent, KeyboardEvent, UserInputMonitor, UtilError};
use self::view_models::{
    DeviceViewModelImpl, KeyboardState, KeyboardViewModelImpl, MouseButtonState,
};
use models::{ButtonOrKey, Keyboard, MouseModel};

use self::models::{DeviceButton, DeviceType, GamepadMouseStick};

pub(crate) use self::args::{default_config, default_skin_dir};
pub(crate) use self::args::{DEFAULT_CONFIG_NAME, DEFAULT_SKIN_DIR_NAME};
pub use self::errors::PTuberError;
pub use self::errors::Result as PtuberResult;
pub(crate) use self::os_ui::{
    get_window_finder, WindowFinder, WindowFinderError, WindowFinderImpl,
};

use self::args::Args;
use self::avatar::{
    image_asset_event_system, ArmImageContainer, AvatarImageContainer, DeviceImageContainer,
    ImageContainer, LeftArmImageContainer, MouseImageContainer,
};
use self::config::{Config, ConfigHandle};

pub type ImageHandle = Handle<Image>;

pub const MAX_FRAMERATE: u32 = 60;
pub const GAMEPAD_POLL_DURATION: Duration = Duration::from_millis(200);
pub use self::consts::WINDOW_DIMENTIONS;

pub struct PTuberPlugin;

impl Plugin for PTuberPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(TomlAssetPlugin::<Config>::new(&["toml"]))
            .add_system(window_update_system)
            .add_system(image_asset_event_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = ConfigHandle(asset_server.load("config.toml"));
    commands.insert_resource(config);

    commands.spawn(Camera2dBundle::default());

    let up = ImageContainer::new(Vec2::ZERO, asset_server.load("up.png"));

    let left = ImageContainer::new(Vec2::ZERO, asset_server.load("left.png"));
    let right = ImageContainer::new(Vec2::ZERO, asset_server.load("right.png"));

    let left = LeftArmImageContainer { up, left, right };

    let right = ImageContainer::new(Vec2::ZERO, asset_server.load("arm.png"));

    let arms = ArmImageContainer { left, right };

    let norm = ImageContainer::new(Vec2::ZERO, asset_server.load("mouse.png"));
    let left = ImageContainer::new(Vec2::ZERO, asset_server.load("mousel.png"));
    let right = ImageContainer::new(Vec2::ZERO, asset_server.load("mouser.png"));
    let both = ImageContainer::new(Vec2::ZERO, asset_server.load("mouselr.png"));

    let mouse = MouseImageContainer {
        norm,
        left,
        right,
        both,
    };

    let devices = DeviceImageContainer { mouse };

    let avatar = ImageContainer::new(Vec2::ZERO, asset_server.load("avatar.png"));
    let background = ImageContainer::new(Vec2::ZERO, asset_server.load("background.png"));

    let images = AvatarImageContainer {
        avatar,
        background,
        arms,
        devices,
    };

    commands.spawn(SpriteBundle {
        texture: images.avatar.handle.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: images.background.handle.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 2.),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(images);
}

fn setup_static_images(mut commands: Commands, avatar_images: Res<AvatarImageContainer>) {}

fn window_update_system(
    mut windows: ResMut<Windows>,
    config_handle: Res<ConfigHandle>,
    assets: Res<Assets<Config>>,
    mut clear_color: ResMut<ClearColor>,
) {
    if let Some(config) = assets.get(&config_handle.0) {
        let bg_color = Color::rgba_u8(
            config.background.red,
            config.background.green,
            config.background.blue,
            config.background.alpha,
        );
        for window in windows.iter_mut() {
            let (cur_w, cur_h) = (window.requested_width(), window.requested_height());
            if config.window.width as f32 != cur_w || config.window.height as f32 != cur_h {
                window.set_resolution(config.window.width as f32, config.window.height as f32);
            }
        }
        let color = clear_color.as_mut();
        if bg_color != color.as_rgba() {
            color
                .set_r(bg_color.r())
                .set_g(bg_color.g())
                .set_b(bg_color.b())
                .set_a(bg_color.a());
        }
    }
}
