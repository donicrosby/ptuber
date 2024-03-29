mod errors;
#[cfg(all(unix, target_os = "linux"))]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub(crate) use self::errors::WindowFinderError;

#[cfg(all(unix, target_os = "linux"))]
pub(crate) use self::linux::LinuxWindowFinder as WindowFinderImpl;
#[cfg(windows)]
pub(crate) use self::windows::WindowsWindowFinder as WindowFinderImpl;

use dyn_clone::{clone_trait_object, DynClone};
use sfml::system::{Vector2f, Vector2i};
use std::fmt::Debug;

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

pub fn get_window_finder() -> Result<WindowFinderImpl, WindowFinderError> {
    let finder = WindowFinderImpl::new()?;
    Ok(finder)
}
