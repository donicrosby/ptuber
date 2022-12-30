use std::path::{PathBuf, Path};
use log::debug;
use sfml::graphics::{Texture, Sprite, RenderWindow, RenderTarget, Transformable, VertexBuffer, PrimitiveType, VertexBufferUsage, Vertex};
use sfml::SfBox;
use sfml::system::Vector2f;
use sfml::graphics::Color;
use crate::{WindowFinder, get_window_finder, util};
use crate::errors::Result;
use crate::Config;
use super::{SfmlError, SfmlResult};


const UP_IMAGE: &'static str = "up.png";
const DOWN_IMAGE: &'static str = "down.png";
const BACKGROUND_IMAGE: &'static str = "bg.png";
const MOUSE: &'static str = "mouse.png";
const MOUSE_L: &'static str = "mousel.png";
const MOUSE_R: &'static str = "mouser.png";
const MOUSE_LR: &'static str = "mouselr.png";

const PAW_START_POINT: Vector2f = Vector2f::new(211.0, 159.0);
const PAW_END_POINT: Vector2f = Vector2f::new(258.0, 228.0);
const OOF: usize = 6;
const PUSH: usize = 20;
const DX: f32 = -38.0;
const DY: f32 = -50.0;
const ITER: usize = 25;

#[derive(Debug, Clone)]
pub(crate) struct TextureContainer {
    pub up: SfBox<Texture>,
    pub down: SfBox<Texture>,
    pub background: SfBox<Texture>,
    pub mouse: MouseTextures
}

impl TextureContainer {
    pub fn new(image_path: &Path) -> SfmlResult<Self> {
        let image_path = PathBuf::from(image_path);

        let mut up_path = image_path.clone();
        up_path.push(UP_IMAGE);
        let up = Texture::from_file(&up_path.to_str().ok_or(SfmlError::PathConversion)?)?;
        
        let mut down_path = image_path.clone();
        down_path.push(DOWN_IMAGE);
        let down = Texture::from_file(&down_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mut background_path = image_path.clone();
        background_path.push(BACKGROUND_IMAGE);
        let background = Texture::from_file(&background_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mouse = MouseTextures::new(&image_path)?;

        Ok(Self {
            up,
            down,
            background,
            mouse
        })
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
    pub fn new(image_path: &Path) -> SfmlResult<Self> {
        let image_path = PathBuf::from(image_path);

        let mut mouse_path = image_path.clone();
        mouse_path.push(MOUSE);
        let mouse = Texture::from_file(&mouse_path.to_str().ok_or(SfmlError::PathConversion)?)?;
        
        let mut mouse_l_path = image_path.clone();
        mouse_l_path.push(MOUSE_L);
        let mouse_l = Texture::from_file(&mouse_l_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mut mouse_r_path = image_path.clone();
        mouse_r_path.push(MOUSE_R);
        let mouse_r = Texture::from_file(&mouse_r_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        let mut mouse_lr_path = image_path.clone();
        mouse_lr_path.push(MOUSE_LR);
        let mouse_lr = Texture::from_file(&mouse_lr_path.to_str().ok_or(SfmlError::PathConversion)?)?;

        Ok(Self {
            mouse,
            mouse_l,
            mouse_r,
            mouse_lr
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Avatar {
    textures: TextureContainer,
    window_finder: Box<dyn WindowFinder>
}

impl Avatar {
    pub fn new(image_path: &Path) -> Result<Self> {
        let textures = TextureContainer::new(&image_path)?;
        let window_finder = get_window_finder()?;
        Ok(Self {
            textures,
            window_finder
        })
    }

    pub fn background_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.background)
    }

    pub fn up_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.up)
    }

    pub fn down_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.down)
    }

    pub fn mouse_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse)
    }

    pub fn mouse_l_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse_l)
    }

    pub fn mouse_r_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse_r)
    }

    pub fn mouse_lr_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.textures.mouse.mouse_lr)
    }

    /// Returns PSS Vec, AB Vec, and Mouse Pos Vec
    fn get_pss(&self, mouse_pos: Vector2f) -> (Vec<f32>, Vector2f, Vector2f) {
        let mut pss = vec![PAW_START_POINT.x, PAW_START_POINT.y];
        let dist = (PAW_START_POINT.x - mouse_pos.x).hypot(PAW_START_POINT.y - mouse_pos.y);
        debug!("First dist: {}", dist);
        let ctr_left = Vector2f::new(PAW_START_POINT.x - (0.7237 * (dist/2.0)), PAW_START_POINT.y + (0.69 * (dist/2.0)));
        debug!("Center Left: {:?}", ctr_left);
        let bez = vec![PAW_START_POINT.x, PAW_START_POINT.y, ctr_left.x, ctr_left.y, mouse_pos.x, mouse_pos.y];
        let mut p;
        for i in 0..OOF {
            p = util::bezier(1.0 * (i as f32) / (OOF as f32), &bez, bez.len());
            pss.push(p.x);
            pss.push(p.y);
        };
        pss.push(mouse_pos.x);
        pss.push(mouse_pos.y);

        let mut ab = Vector2f::new(mouse_pos.y - ctr_left.y,ctr_left.x - mouse_pos.x);
        debug!("AB Vec: {:?}", ab);
        let le = ab.x.hypot(ab.y);

        ab.x = mouse_pos.x + ab.x / le * 60.0;
        ab.y = mouse_pos.y + ab.y / le * 60.0;

        debug!("AB Vec: {:?}, LE: {}", ab, le);

        let dist = (PAW_END_POINT.x - ab.x).hypot(PAW_END_POINT.y - ab.y);
        let ctr_right = Vector2f::new(PAW_END_POINT.x - 0.6 * dist / 2.0, PAW_END_POINT.y + 0.8 * dist / 2.0);
        
        let mut st = Vector2f::new(mouse_pos.x - ctr_left.x, mouse_pos.y - ctr_left.y);
        let le = st.x.hypot(st.y);
        st.x *= (PUSH as f32)/le;
        st.y *= (PUSH as f32)/le;
        let st2 = Vector2f::new(ab.x - ctr_right.x, ab.y - ctr_right.y);

        let bez = vec![mouse_pos.x, mouse_pos.y, mouse_pos.x + st.x, mouse_pos.y + st.y, ab.x + st2.x, ab.y + st.y, ab.x, ab.y];
        for i in 0..OOF {
            p = util::bezier(1.0 * (i as f32) / (OOF as f32), &bez, bez.len());
            pss.push(p.x);
            pss.push(p.y);
        }
        pss.push(ab.x);
        pss.push(ab.y);

        let bez = vec![PAW_END_POINT.x, PAW_END_POINT.y, ctr_right.x, ctr_right.y, ab.x, ab.y];
        for i in (0..OOF).rev() {
            p = util::bezier(1.0 * (i as f32) / (OOF as f32), &bez, bez.len());
            pss.push(p.x);
            pss.push(p.y);
        }
        pss.push(PAW_END_POINT.x);
        pss.push(PAW_END_POINT.y);

        (pss, ab, mouse_pos)
    }

    /// Returns PSS2 Vec
    fn get_pss2(&self, pss: Vec<f32>) -> Vec<f32> {
        let mut pss2 = vec![pss[0] + DX, pss[1] + DY];
        debug!("PSS LEN: {}", pss.len());
        let mut p;
        for i in 0..ITER {
            p = util::bezier(1.0 * (i as f32) / (ITER as f32), &pss, 38); //weird 38 for some reason
            pss2.push(p.x);
            pss2.push(p.y);
        }
        pss2.push(pss[36] + DX);
        pss2.push(pss[37] + DY);
        pss2
    }

    fn get_fill(&self, pss: Vec<f32>, color: Color) -> VertexBuffer {
        let mut fill = VertexBuffer::new(PrimitiveType::TRIANGLE_STRIP, 26, VertexBufferUsage::DYNAMIC);
        let mut vert_vec = Vec::new();
        for i in 0..(pss.len()/2) {
            let vert = Vertex::with_pos_color(Vector2f::new(pss[i], pss[i+1]), color.clone());
            let vert2 = Vertex::with_pos_color(Vector2f::new(pss[52 - i - 2], pss[52 - i - 1]), color.clone());
            vert_vec.push(vert);
            vert_vec.push(vert2);
        }
        fill.update(&vert_vec, 0);
        fill
    }

    pub fn draw(&self, window: &mut RenderWindow, config: &Config) -> Result<()> {
        let bg = self.background_sprite();
        let mouse_pos = self.window_finder.get_cursor_position()?;
        debug!("Mouse Pos: {:?}", mouse_pos);
        let (pss, ab, mouse_pos) = self.get_pss(mouse_pos);
        debug!("AB Vec: {:?}", ab);
        let mpos = Vector2f::new((ab.x + mouse_pos.x)/ 2.0 - 52.0 - 15.0, (ab.y + mouse_pos.y) - 34.0 + 5.0);
        let pss2 = self.get_pss2(pss);

        let mut device = self.mouse_sprite();
        let dev_pos = Vector2f::new(mpos.x, mpos.y);
        debug!("Mouse Pos Final: {:?}", dev_pos);
        device.set_position(dev_pos);
        device.set_scale(Vector2f::new(1.0, 1.0));

        let fill = self.get_fill(pss2, config.flipper.base.clone().into());

        window.draw(&bg);
        window.draw(&device);
        window.draw(&fill);
        Ok(())
    }
    
}