mod errors;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;
mod user_input;

pub(crate) use self::errors::WindowFinderError;
pub(crate) use self::user_input::{InputGrabber, InputGrabRunFlag, KeyboardEvent, MouseEvent};

#[cfg(target_os = "linux")]
use self::linux::LinuxWindowFinder;
#[cfg(target_os = "windows")]
use self::windows::WindowsWindowFinder;

use cfg_if::cfg_if;
use core::fmt::Debug;
use dyn_clone::{clone_trait_object, DynClone};
use sfml::system::{Vector2f, Vector2i};

pub trait WindowFinder: Debug + DynClone {
    fn get_focused_window_size(&self) -> Result<Vector2i, WindowFinderError>;
    fn get_cursor_position(&self) -> Result<Vector2f, WindowFinderError> {
        let curs_pos = sfml::window::mouse::desktop_position();
        let screen_size = self.get_focused_screen_size()?;
        let x = curs_pos.x as f32 / screen_size.x as f32;
        let y = curs_pos.y as f32 / screen_size.y as f32;
        Ok(Vector2f::new(x, y))
    }
    fn get_focused_screen_size(&self) -> Result<Vector2i, WindowFinderError>;
}

clone_trait_object!(WindowFinder);

#[cfg(target_os = "linux")]
fn get_linux_finder() -> Result<impl WindowFinder, WindowFinderError> {
    let l = LinuxWindowFinder::new()?;
    Ok(l)
}

#[cfg(target_os = "windows")]
fn get_windows_finder() -> Result<impl WindowFinder, WindowFinderError> {
    let l = WindowsWindowFinder::new()?;
    Ok(l)
}

pub fn get_window_finder() -> Result<Box<dyn WindowFinder>, WindowFinderError> {
    cfg_if! {
        if #[cfg(target_os="windows")] {
            let t = get_windows_finder()?;
        } else {
            let t = get_linux_finder()?;
        }
    }
    Ok(Box::new(t))
}
