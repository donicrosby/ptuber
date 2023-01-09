use device_query::MouseButton;
use log::trace;
use sfml::system::Vector2i;
use std::sync::Arc;
use x11rb::protocol::randr::ConnectionExt as randrConnectionExt;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::protocol::xproto::Window;
use x11rb::rust_connection::RustConnection;

use super::MouseButtonImpl;
use super::WindowFinder;
use super::WindowFinderError;

impl From<MouseButton> for MouseButtonImpl {
    fn from(value: MouseButton) -> Self {
        match value {
            1 => MouseButtonImpl::Left,
            2 => MouseButtonImpl::Middle,
            3 => MouseButtonImpl::Right,
            4 => MouseButtonImpl::XButton1,
            5 => MouseButtonImpl::XButton2,
            v => MouseButtonImpl::Unknown(v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinuxWindowFinder {
    connection: Arc<RustConnection>,
}

impl LinuxWindowFinder {
    pub fn new() -> Result<Self, WindowFinderError> {
        let (connection, _screen_num) = x11rb::connect(None)?;
        let connection = Arc::new(connection);
        Ok(Self { connection })
    }

    fn get_focused_window(&self) -> Result<Window, WindowFinderError> {
        let input_focus = self.connection.get_input_focus()?.reply()?;
        Ok(input_focus.focus)
    }
}

impl WindowFinder for LinuxWindowFinder {
    fn get_focused_window_size(&self) -> Result<Vector2i, WindowFinderError> {
        let input_focus = self.get_focused_window()?;
        let geometry = self.connection.get_geometry(input_focus)?.reply()?;

        Ok(Vector2i::new(geometry.width.into(), geometry.height.into()))
    }

    fn get_focused_screen_size(&self) -> Result<Vector2i, WindowFinderError> {
        let input_focus = self.get_focused_window()?;
        trace!("Input Focus Window: {}", input_focus);
        let screen_info = self
            .connection
            .randr_get_screen_resources_current(input_focus)?
            .reply()?;
        trace!("Screen Info: {:?}", screen_info);
        let screen = screen_info.modes[0];
        Ok(Vector2i::new(screen.width.into(), screen.height.into()))
    }
}
