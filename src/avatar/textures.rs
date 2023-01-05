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

pub trait TextureContainer {
    fn reload_textures(&mut self, images_path: &Path) -> SfmlResult<()>;
    fn load_texture_from_file(images_path: &Path, image: &str) -> SfmlResult<SfBox<Texture>> {
        let mut image_path = PathBuf::from(images_path);
        image_path.push(image);
        let texture = Texture::from_file(image_path.to_str().ok_or(SfmlError::PathConversion)?)?;
        Ok(texture)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AvatarTextures {
    pub background: SfBox<Texture>,
    pub avatar: SfBox<Texture>,
}

impl AvatarTextures {
    pub fn new(images_path: &Path) -> SfmlResult<Self> {
        let background = Self::load_texture_from_file(images_path, BACKGROUND_IMAGE)?;
        let avatar = Self::load_texture_from_file(images_path, AVATAR_IMAGE)?;

        Ok(Self { background, avatar })
    }
}

impl TextureContainer for AvatarTextures {
    fn reload_textures(&mut self, images_path: &Path) -> SfmlResult<()> {
        let background = Self::load_texture_from_file(images_path, BACKGROUND_IMAGE)?;
        let avatar = Self::load_texture_from_file(images_path, AVATAR_IMAGE)?;
        self.background = background;
        self.avatar = avatar;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ArmTextures {
    pub left: LeftArmTextures,
    pub right: SfBox<Texture>,
}

impl ArmTextures {
    pub fn new(images_path: &Path) -> SfmlResult<Self> {
        let right = Self::load_texture_from_file(images_path, RIGHT_ARM_IMAGE)?;

        let left = LeftArmTextures::new(images_path)?;
        Ok(Self { right, left })
    }
}

impl TextureContainer for ArmTextures {
    fn reload_textures(&mut self, images_path: &Path) -> SfmlResult<()> {
        let right = Self::load_texture_from_file(images_path, RIGHT_ARM_IMAGE)?;
        self.right = right;
        self.left.reload_textures(images_path)?;
        Ok(())
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
        let left = Self::load_texture_from_file(images_path, LEFT_ARM_LEFT_IMAGE)?;
        let right = Self::load_texture_from_file(images_path, LEFT_ARM_RIGHT_IMAGE)?;
        let up = Self::load_texture_from_file(images_path, LEFT_ARM_UP_IMAGE)?;

        Ok(Self { left, right, up })
    }
}

impl TextureContainer for LeftArmTextures {
    fn reload_textures(&mut self, images_path: &Path) -> SfmlResult<()> {
        let left = Self::load_texture_from_file(images_path, LEFT_ARM_LEFT_IMAGE)?;
        let right = Self::load_texture_from_file(images_path, LEFT_ARM_RIGHT_IMAGE)?;
        let up = Self::load_texture_from_file(images_path, LEFT_ARM_UP_IMAGE)?;

        self.left = left;
        self.right = right;
        self.up = up;
        Ok(())
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
        let mouse = Self::load_texture_from_file(images_path, MOUSE)?;
        let mouse_l = Self::load_texture_from_file(images_path, MOUSE_L)?;
        let mouse_r = Self::load_texture_from_file(images_path, MOUSE_R)?;
        let mouse_lr = Self::load_texture_from_file(images_path, MOUSE_LR)?;

        Ok(Self {
            mouse,
            mouse_l,
            mouse_r,
            mouse_lr,
        })
    }
}

impl TextureContainer for MouseTextures {
    fn reload_textures(&mut self, images_path: &Path) -> SfmlResult<()> {
        let mouse = Self::load_texture_from_file(images_path, MOUSE)?;
        let mouse_l = Self::load_texture_from_file(images_path, MOUSE_L)?;
        let mouse_r = Self::load_texture_from_file(images_path, MOUSE_R)?;
        let mouse_lr = Self::load_texture_from_file(images_path, MOUSE_LR)?;

        self.mouse = mouse;
        self.mouse_l = mouse_l;
        self.mouse_r = mouse_r;
        self.mouse_lr = mouse_lr;

        Ok(())
    }
}
