mod keyboard;
mod mouse;
mod traits;
mod types;

use std::sync::{Arc, Mutex};

use sfml::system::Vector2f;
use log::debug;

use self::keyboard::KeyboardViewModel;
use crate::ButtonOrKey;
use self::mouse::MouseViewModel;
pub(crate) use self::traits::{DeviceViewModel, KeysViewModel};
pub(crate) use self::types::{KeyboardState, MouseButtonState};
use super::{DeviceButton, DeviceEvent, Keyboard, KeyboardEvent, MouseModel, DeviceType};

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
                DeviceEvent::MouseMoved(pos) => {
                    debug!("Mouse moved to {:?}", pos);
                    view_model.set_position(pos)
                },
                DeviceEvent::AxisXMoved(x_pos) => {
                    debug!("X Axis moved to {:?}", x_pos);
                    let mut cur_pos = view_model.position();
                    cur_pos.x = *x_pos;
                    view_model.set_position(&cur_pos);
                },
                DeviceEvent::AxisYMoved(y_pos) => {
                    debug!("Y Axis moved to {:?}", y_pos);
                    let mut cur_pos = view_model.position();
                    cur_pos.y = *y_pos;
                    view_model.set_position(&cur_pos);
                },
                DeviceEvent::DeviceChanged(dev) => view_model.set_device_type(dev)
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
                KeyboardEvent::KeyPressed(k) => view_model.key_pressed(&ButtonOrKey::Key(*k)),
                KeyboardEvent::KeyReleased(k) => view_model.key_released(&ButtonOrKey::Key(*k)),
                KeyboardEvent::ButtonPressed(b) => view_model.key_pressed(&ButtonOrKey::Button(b.clone())),
                KeyboardEvent::ButtonReleased(b) => view_model.key_released(&ButtonOrKey::Button(b.clone()))
            }
        }
    }
}
