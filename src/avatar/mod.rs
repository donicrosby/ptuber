use bevy::prelude::*;
mod arms;
mod avatar_impl;
mod device;
mod errors;
mod textures;
mod window;

pub(crate) use self::arms::Arms;
pub(crate) use self::avatar_impl::Avatar;
pub(crate) use self::device::Device;
pub(crate) use self::textures::{ArmTextures, AvatarTextures, MouseTextures, TextureContainer};
use crate::ImageHandle;
pub(crate) use errors::{SfmlError, SfmlResult};

pub(crate) struct ImageContainer {
    pub size: Vec2,
    pub handle: ImageHandle,
}

impl ImageContainer {
    pub fn new(size: Vec2, handle: ImageHandle) -> Self {
        Self { size, handle }
    }

    pub fn update_size(&mut self, size: Vec2) {
        self.size = size;
    }
}

pub(crate) struct LeftArmImageContainer {
    pub up: ImageContainer,
    pub left: ImageContainer,
    pub right: ImageContainer,
}

impl LeftArmImageContainer {
    pub fn update_size(&mut self, handle: &Handle<Image>, size: &Vec2) -> bool {
        if *handle == self.up.handle {
            self.up.update_size(*size);
            true
        } else if *handle == self.left.handle {
            self.left.update_size(*size);
            true
        } else if *handle == self.right.handle {
            self.right.update_size(*size);
            true
        } else {
            false
        }
    }
}

pub(crate) struct ArmImageContainer {
    pub left: LeftArmImageContainer,
    pub right: ImageContainer,
}

impl ArmImageContainer {
    pub fn update_size(&mut self, handle: &Handle<Image>, size: &Vec2) -> bool {
        if *handle == self.right.handle {
            self.right.update_size(*size);
            true
        } else {
            self.left.update_size(handle, size)
        }
    }
}

pub(crate) struct MouseImageContainer {
    pub norm: ImageContainer,
    pub left: ImageContainer,
    pub right: ImageContainer,
    pub both: ImageContainer,
}

impl MouseImageContainer {
    pub fn update_size(&mut self, handle: &Handle<Image>, size: &Vec2) -> bool {
        if *handle == self.norm.handle {
            self.norm.update_size(*size);
            true
        } else if *handle == self.left.handle {
            self.left.update_size(*size);
            true
        } else if *handle == self.right.handle {
            self.right.update_size(*size);
            true
        } else if *handle == self.both.handle {
            self.both.update_size(*size);
            true
        } else {
            false
        }
    }
}

pub(crate) struct DeviceImageContainer {
    pub mouse: MouseImageContainer,
}

impl DeviceImageContainer {
    pub fn update_size(&mut self, handle: &Handle<Image>, size: &Vec2) -> bool {
        self.mouse.update_size(handle, size)
    }
}

#[derive(Resource)]
pub(crate) struct AvatarImageContainer {
    pub avatar: ImageContainer,
    pub background: ImageContainer,
    pub arms: ArmImageContainer,
    pub devices: DeviceImageContainer,
}

impl AvatarImageContainer {
    pub fn update_size(&mut self, handle: &Handle<Image>, size: &Vec2) {
        if self.arms.update_size(handle, size) || self.devices.update_size(handle, size) {
            return;
        } else if *handle == self.avatar.handle {
            self.avatar.update_size(*size);
        } else if *handle == self.background.handle {
            self.background.update_size(*size)
        }
    }
}

pub(crate) fn image_asset_event_system(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    assets: Res<Assets<Image>>,
    mut asset_container: ResMut<AvatarImageContainer>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                // a texture was just loaded or changed!

                // WARNING: this mutable access will cause another
                // AssetEvent (Modified) to be emitted!
                let texture = assets.get(handle).unwrap();
                // ^ unwrap is OK, because we know it is loaded now

                asset_container.update_size(handle, &texture.size());
            }
            AssetEvent::Removed { handle } => {
                asset_container.update_size(handle, &Vec2::ZERO);
            }
        }
    }
}
