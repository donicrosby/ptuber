use bevy::prelude::*;
use bevy::window::PresentMode;
use ptuber::{PTuberPlugin, WINDOW_DIMENTIONS};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "PTuber Rigger!".to_string(),
                        width: WINDOW_DIMENTIONS.0,
                        height: WINDOW_DIMENTIONS.1,
                        resizable: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        )
        .add_plugin(PTuberPlugin)
        .run();
}
