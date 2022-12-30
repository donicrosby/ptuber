use super::{SfmlError, SfmlResult};
use sfml::graphics::Texture;
use sfml::SfBox;
use std::path::{Path, PathBuf};

const RIGHT_ARM_IMAGE: &str = "arm.png";
const LEFT_ARM_LEFT_IMAGE: &str = "left.png";
const LEFT_ARM_RIGHT_IMAGE: &str = "right.png";
const LEFT_ARM_UP_IMAGE: &str = "up.png";
const BACKGROUND_IMAGE: &str = "background.png";
const AVATAR_IMAGE: &str = "avatar.png";
const MOUSE: &str = "mouse.png";
const MOUSE_L: &str = "mousel.png";
const MOUSE_R: &str = "mouser.png";
const MOUSE_LR: &str = "mouselr.png";

fn load_texture_from_file(images_path: &Path, image: &str) -> SfmlResult<SfBox<Texture>> {
    let mut image_path = PathBuf::from(images_path);
    image_path.push(image);
    let texture = Texture::from_file(image_path.to_str().ok_or(SfmlError::PathConversion)?)?;
    Ok(texture)
}

#[derive(Debug, Clone)]
pub(crate) struct TextureContainer {
    pub background: SfBox<Texture>,
    pub avatar: SfBox<Texture>,
    pub arms: ArmTextures,
    pub mouse: MouseTextures,
}

impl TextureContainer {
    pub fn new(images_path: &Path) -> SfmlResult<Self> {
        let background = load_texture_from_file(images_path, BACKGROUND_IMAGE)?;
        let avatar = load_texture_from_file(images_path, AVATAR_IMAGE)?;

        let arms = ArmTextures::new(images_path)?;
        let mouse = MouseTextures::new(images_path)?;

        Ok(Self {
            background,
            avatar,
            arms,
            mouse,
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ArmTextures {
    pub left: LeftArmTextures,
    pub right: SfBox<Texture>,
}

impl ArmTextures {
    pub fn new(images_path: &Path) -> SfmlResult<Self> {
        let right = load_texture_from_file(images_path, RIGHT_ARM_IMAGE)?;

        let left = LeftArmTextures::new(images_path)?;
        Ok(Self { right, left })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LeftArmTextures {
    pub left: SfBox<Texture>,
    pub right: SfBox<Texture>,
    pub up: SfBox<Texture>,
}

impl LeftArmTextures {
    pub fn new(images_path: &Path) -> SfmlResult<Self> {
        let left = load_texture_from_file(images_path, LEFT_ARM_LEFT_IMAGE)?;
        let right = load_texture_from_file(images_path, LEFT_ARM_RIGHT_IMAGE)?;
        let up = load_texture_from_file(images_path, LEFT_ARM_UP_IMAGE)?;

        Ok(Self { left, right, up })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MouseTextures {
    pub mouse: SfBox<Texture>,
    pub mouse_l: SfBox<Texture>,
    pub mouse_r: SfBox<Texture>,
    pub mouse_lr: SfBox<Texture>,
}

impl MouseTextures {
    pub fn new(images_path: &Path) -> SfmlResult<Self> {
        let mouse = load_texture_from_file(images_path, MOUSE)?;
        let mouse_l = load_texture_from_file(images_path, MOUSE_L)?;
        let mouse_r = load_texture_from_file(images_path, MOUSE_R)?;
        let mouse_lr = load_texture_from_file(images_path, MOUSE_LR)?;

        Ok(Self {
            mouse,
            mouse_l,
            mouse_r,
            mouse_lr,
        })
    }
}
