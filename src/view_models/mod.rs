mod keyboard;
mod mouse;
mod traits;
mod types;

use std::sync::{Arc, Mutex};

use sfml::system::Vector2f;

use self::keyboard::KeyboardViewModel;
use self::mouse::MouseViewModel;
pub(crate) use self::traits::{DeviceViewModel, KeysViewModel};
pub(crate) use self::types::{KeyboardState, MouseButtonState};
use super::{DeviceButton, DeviceEvent, Keyboard, KeyboardEvent, MouseModel};

pub struct DeviceViewModelImpl {
    view_model: Arc<Mutex<MouseViewModel>>,
}

impl DeviceViewModelImpl {
    pub fn new() -> Self {
        let view_model = Arc::new(Mutex::new(MouseViewModel::new()));
        Self { view_model }
    }

    pub fn position(&self) -> Vector2f {
        if let Ok(view_model) = self.view_model.lock() {
            view_model.position()
        } else {
            Default::default()
        }
    }

    pub fn button_state(&self) -> MouseButtonState {
        if let Ok(view_model) = self.view_model.lock() {
            view_model.button_state()
        } else {
            Default::default()
        }
    }

    pub fn handle_event(&self, event: &DeviceEvent) {
        if let Ok(mut view_model) = self.view_model.lock() {
            match event {
                DeviceEvent::ButtonPressed(b) => view_model.button_pressed(b),
                DeviceEvent::ButtonReleased(b) => view_model.button_released(b),
                DeviceEvent::MouseMoved(pos) => view_model.set_position(pos),
            }
        }
    }
}

pub struct KeyboardViewModelImpl {
    view_model: Arc<Mutex<KeyboardViewModel>>,
}

impl KeyboardViewModelImpl {
    pub fn new() -> Self {
        let view_model = Arc::new(Mutex::new(KeyboardViewModel::new()));
        Self { view_model }
    }

    pub fn keyboard_state(&self) -> KeyboardState {
        if let Ok(view_model) = self.view_model.lock() {
            view_model.keyboard_state()
        } else {
            Default::default()
        }
    }

    pub fn handle_event(&self, event: &KeyboardEvent) {
        if let Ok(mut view_model) = self.view_model.lock() {
            match event {
                KeyboardEvent::KeyPressed(k) => view_model.key_pressed(k),
                KeyboardEvent::KeyReleased(k) => view_model.key_released(k),
            }
        }
    }
}
