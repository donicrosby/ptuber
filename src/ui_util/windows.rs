use sfml::system::Vector2i;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, HMONITOR, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect};
use device_query::MouseButton;

use super::WindowFinder;
use super::WindowFinderError;
pub(crate) use super::MouseButtonType;

#[derive(Debug, Clone)]
pub struct WindowsWindowFinder {}

impl WindowsWindowFinder {
    pub fn new() -> Result<Self, WindowFinderError> {
        Ok(Self {})
    }

    fn get_focused_window(&self) -> HWND {
        let window;
        unsafe {
            window = GetForegroundWindow();
        }
        window
    }

    fn get_monitor(&self, w: HWND) -> Result<HMONITOR, WindowFinderError> {
        let monitor;
        unsafe {
            monitor = MonitorFromWindow(w, MONITOR_DEFAULTTONEAREST);
        }
        if !monitor.is_invalid() {
            Ok(monitor)
        } else {
            Err(WindowFinderError::WindowsMonitorInvalid)
        }
    }
}

impl WindowFinder for WindowsWindowFinder {
    fn get_focused_window_size(&self) -> Result<Vector2i, WindowFinderError> {
        let mut rect = Default::default();
        let window = self.get_focused_window();
        unsafe {
            GetWindowRect(window, &mut rect);
        }

        Ok(Vector2i::new(
            rect.right - rect.left,
            rect.bottom - rect.top,
        ))
    }

    fn get_focused_screen_size(&self) -> Result<Vector2i, WindowFinderError> {
        let window = self.get_focused_window();
        let monitor = self.get_monitor(window)?;
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        unsafe {
            GetMonitorInfoW(monitor, &mut info);
        }
        Ok(Vector2i::new(
            info.rcMonitor.right - info.rcMonitor.left,
            info.rcMonitor.bottom - info.rcMonitor.top,
        ))
    }
}

impl From<MouseButton> for MouseButtonType {
    fn from(value: MouseButton) -> Self {
        match value {
            1 => MouseButtonType::Left,
            2 => MouseButtonType::Right,
            3 => MouseButtonType::Middle,
            u => MouseButtonType::Unknown(u),
        }
    }
}