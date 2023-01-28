use device_query::{DeviceQuery, DeviceState, Keycode};
use gilrs::{Axis, Event, EventType, Gilrs};
use log::debug;
use sfml::system::Vector2f;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use super::{CallbackGuard, DeviceEvent, KeyboardEvent, Notifier, UtilError, MAX_AXIS_VAL};
use crate::models::DeviceType;
use crate::Config;
use crate::{get_window_finder, GamepadMouseStick, WindowFinder, WindowFinderImpl};

pub struct UserInputMonitor {
    notifiers: Arc<Mutex<Notifiers>>,
    window_finder: WindowFinderImpl,
    device_state: DeviceState,
    mouse_stick: GamepadMouseStick,
    gilrs: Option<Gilrs>,
    prev_keys: HashSet<Keycode>,
    joystick: Option<usize>,
    last_mouse_pos: Vector2f,
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
    pub fn new(joystick_id: Option<usize>, mouse_stick: GamepadMouseStick) -> Self {
        let notifiers = Arc::new(Mutex::new(Notifiers::new()));
        let window_finder = get_window_finder().expect("Could not get window finder");
        let device_state = DeviceState::new();
        let prev_keys = HashSet::new();
        let gilrs = Gilrs::new().ok();
        let joystick = joystick_id;
        let last_mouse_pos = Vector2f::new(0.0, 0.0);
        Self {
            notifiers,
            window_finder,
            device_state,
            prev_keys,
            mouse_stick,
            gilrs,
            joystick,
            last_mouse_pos,
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

    pub fn update_config(&mut self, config: &Config) {
        if config.gamepad.enabled {
            self.joystick = Some(config.gamepad.gamepad_id);
        } else {
            self.joystick = None;
        }
        self.mouse_stick = config.gamepad.mouse_move_joystick;
    }

    fn get_gamepad_events(
        gilrs: &mut Gilrs,
        joystick_id: usize,
        mouse_stick: &GamepadMouseStick,
    ) -> (Vec<DeviceEvent>, Vec<KeyboardEvent>) {
        let mut device_events = Vec::new();
        let mut keyboard_events = Vec::new();
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            if joystick_id == 0 {
                match event {
                    EventType::ButtonPressed(button, _code) => {
                        debug!("Gamepad {} button pressed {:?}", joystick_id, button);
                        keyboard_events.push(KeyboardEvent::ButtonPressed(button));
                    }
                    EventType::ButtonReleased(button, _code) => {
                        debug!("Gamepad {} button released {:?}", joystick_id, button);
                        keyboard_events.push(KeyboardEvent::ButtonReleased(button));
                    }
                    EventType::AxisChanged(axis, pos, _code) => {
                        match mouse_stick {
                            GamepadMouseStick::Left => {
                                if axis == Axis::LeftStickX {
                                    let pos = (pos + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                    debug!("Gamepad {} mouse X moved {:?}", joystick_id, pos);
                                    device_events.push(DeviceEvent::AxisXMoved(pos));
                                } else if axis == Axis::LeftStickY {
                                    debug!("Gamepad {} mouse Y moved {:?}", joystick_id, pos);
                                    let pos = (pos + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                    device_events.push(DeviceEvent::AxisYMoved(pos));
                                }
                            }
                            GamepadMouseStick::Right => {
                                if axis == Axis::RightStickX {
                                    let pos = (pos + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                    debug!("Gamepad {} mouse X moved {:?}", joystick_id, pos);
                                    device_events.push(DeviceEvent::AxisXMoved(pos));
                                } else if axis == Axis::RightStickY {
                                    let pos = (pos + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                    debug!("Gamepad {} mouse Y moved {:?}", joystick_id, pos);
                                    device_events.push(DeviceEvent::AxisYMoved(pos));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        (device_events, keyboard_events)
    }

    pub fn get_events(&mut self) {
        let mut mouse_events = Vec::new();
        let mut keyboard_events = Vec::new();
        let mut gamepad_device_events = Vec::new();
        let mut gamepad_keyboard_events = Vec::new();
        let mut raw_mouse_events;
        let mut raw_keyboard_events;

        if let (Some(gilrs), Some(joystick_id)) = (self.gilrs.as_mut(), self.joystick.as_ref()) {
            (gamepad_device_events, gamepad_keyboard_events) =
                Self::get_gamepad_events(gilrs, *joystick_id, &self.mouse_stick);
            if !gamepad_device_events.is_empty() || !gamepad_keyboard_events.is_empty() {
                mouse_events.push(DeviceEvent::DeviceChanged(DeviceType::GamePad));
            }
        }

        raw_mouse_events = Self::get_mouse_events(&self.device_state);

        if let Ok(pos) = self.window_finder.get_cursor_position() {
            if pos != self.last_mouse_pos {
                raw_mouse_events.push(DeviceEvent::MouseMoved(pos));
            }
            self.last_mouse_pos = pos;
        }

        raw_keyboard_events = Self::get_keyboard_events(&self.device_state, &mut self.prev_keys);

        if !raw_mouse_events.is_empty() && !raw_keyboard_events.is_empty() {
            mouse_events.push(DeviceEvent::DeviceChanged(DeviceType::Mouse));
        }

        mouse_events.append(&mut gamepad_device_events);
        mouse_events.append(&mut raw_mouse_events);
        keyboard_events.append(&mut gamepad_keyboard_events);
        keyboard_events.append(&mut raw_keyboard_events);

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
