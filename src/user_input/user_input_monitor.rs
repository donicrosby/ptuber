use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use super::{CallbackGuard, DeviceEvent, KeyboardEvent, Notifier, UtilError};
use crate::{get_window_finder, WindowFinderImpl};
use crate::os_ui::{WindowFinder};

pub struct UserInputMonitor {
    notifiers: Arc<Mutex<Notifiers>>,
    window_finder: WindowFinderImpl,
    device_state: DeviceState,
    prev_keys: HashSet<Keycode>
}

struct Notifiers {
    mouse_notifier: Notifier<DeviceEvent>,
    keyboard_notifier: Notifier<KeyboardEvent>,
}

impl Notifiers {
    pub fn new() -> Self {
        let mouse_notifier = Notifier::new();
        let keyboard_notifier = Notifier::new();
        Self {
            mouse_notifier,
            keyboard_notifier,
        }
    }

    pub fn add_device_callback<F>(&mut self, callback: F) -> CallbackGuard<F>
    where
        F: 'static + Fn(&DeviceEvent) + Send + Sync,
    {
        let _callback = Arc::new(callback);
        self.mouse_notifier.register::<F>(_callback.clone());
        CallbackGuard { _callback }
    }

    pub fn add_keyboard_callback<F>(&mut self, callback: F) -> CallbackGuard<F>
    where
        F: 'static + Fn(&KeyboardEvent) + Send + Sync,
    {
        let _callback = Arc::new(callback);
        self.keyboard_notifier.register::<F>(_callback.clone());
        CallbackGuard { _callback }
    }
}

impl UserInputMonitor {
    pub fn new() -> Self {
        let notifiers = Arc::new(Mutex::new(Notifiers::new()));
        let window_finder = get_window_finder().expect("Could not get window finder");
        let device_state = DeviceState::new();
        let prev_keys = HashSet::new();
        Self {
            notifiers,
            window_finder,
            device_state,
            prev_keys
        }
    }

    pub fn add_device_callback<F>(&mut self, callback: F) -> Result<CallbackGuard<F>, UtilError>
    where
        F: 'static + Fn(&DeviceEvent) + Send + Sync,
    {
        if let Ok(mut notifiers) = self.notifiers.lock() {
            let guard = notifiers.add_device_callback(callback);
            Ok(guard)
        } else {
            Err(UtilError::MouseCallbackRegister)
        }
    }

    pub fn add_keyboard_callback<F>(&mut self, callback: F) -> Result<CallbackGuard<F>, UtilError>
    where
        F: 'static + Fn(&KeyboardEvent) + Send + Sync,
    {
        if let Ok(mut notifiers) = self.notifiers.lock() {
            let guard = notifiers.add_keyboard_callback(callback);
            Ok(guard)
        } else {
            Err(UtilError::KeyboardCallbackRegister)
        }
    }

    // fn sleep_for_next_poll(now: &Instant, poll_duration: Duration) {
    //     let elapsed = now.elapsed();
    //     if elapsed < poll_duration {
    //         thread::sleep(poll_duration - elapsed);
    //     }
    // }

    fn get_mouse_events(state: &DeviceState) -> Vec<DeviceEvent> {
        let mut events = Vec::new();

        let cur_state = state.get_mouse();
        for (b, pressed) in cur_state.button_pressed.iter().enumerate() {
            // First button never used
            if b > 0 {
                let button = b.into();
                if *pressed {
                    events.push(DeviceEvent::ButtonPressed(button));
                } else {
                    events.push(DeviceEvent::ButtonReleased(button));
                }
            }
        }
        events
    }

    pub fn get_events(&mut self) {
        let mut mouse_events = Vec::new();
        let mut keyboard_events = Vec::new();
        
        mouse_events.append(&mut Self::get_mouse_events(&self.device_state));
        
        if let Ok(pos) = self.window_finder.get_cursor_position() {
            mouse_events.push(DeviceEvent::MouseMoved(pos));
        }
        
        keyboard_events.append(&mut Self::get_keyboard_events(&self.device_state, &mut self.prev_keys));
        if let Ok(notifier) = self.notifiers.lock() {
            for event in mouse_events.into_iter() {
                notifier.mouse_notifier.notify(&event);
            }
            for event in keyboard_events.into_iter() {
                notifier.keyboard_notifier.notify(&event);
            }
        }
    }

    fn get_keyboard_events(
        state: &DeviceState,
        prev_keys: &mut HashSet<Keycode>,
    ) -> Vec<KeyboardEvent> {
        let cur_state = state.get_keys();
        let mut keys_to_remove = Vec::new();
        let mut events = Vec::new();
        for code in prev_keys.iter() {
            //Previous state doesn't contain key so released
            if !cur_state.contains(code) {
                let event = KeyboardEvent::KeyReleased(*code);
                keys_to_remove.push(*code);
                events.push(event);
            }
        }

        for code in keys_to_remove.iter() {
            prev_keys.remove(code);
        }

        for code in cur_state.iter() {
            //Previous state did not contain key so pressed
            if !prev_keys.insert(*code) {
                let event = KeyboardEvent::KeyPressed(*code);
                events.push(event);
            }
        }
        events
    }
}
