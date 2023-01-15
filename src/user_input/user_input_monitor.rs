use device_query::{DeviceQuery, DeviceState, Keycode};
use sfml::system::Vector2f;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use log::{debug, warn, trace};
use sfml::window::joystick;
use std::time::{Duration, Instant};

use super::{CallbackGuard, DeviceEvent, KeyboardEvent, Notifier, UtilError, GamePad, GampadDB, SFMLJoystick, JoystickEvent, AxisType, MAX_AXIS_VAL};
use crate::models::DeviceType;
use crate::{get_window_finder, WindowFinderImpl, WindowFinder, GamepadMouseStick};

pub struct UserInputMonitor {
    notifiers: Arc<Mutex<Notifiers>>,
    window_finder: WindowFinderImpl,
    device_state: DeviceState,
    mouse_stick: GamepadMouseStick,
    gamepad: Option<GamePad>,
    prev_keys: HashSet<Keycode>,
    gamepad_poll: Duration,
    last_check_time: Instant,
    last_mouse_pos: Vector2f
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
    pub fn new(joystick_id: u32, gamepad_poll: Duration, mouse_stick: GamepadMouseStick) -> Self {
        let notifiers = Arc::new(Mutex::new(Notifiers::new()));
        let window_finder = get_window_finder().expect("Could not get window finder");
        let device_state = DeviceState::new();
        let prev_keys = HashSet::new();
        let gamepad = Self::get_updated_gamepad(joystick_id);
        let last_check_time = Instant::now();
        let last_mouse_pos = Vector2f::new(0.0, 0.0);
        Self {
            notifiers,
            window_finder,
            device_state,
            prev_keys,
            gamepad,
            gamepad_poll,
            last_check_time,
            mouse_stick,
            last_mouse_pos,
        }
    }

    fn get_updated_gamepad(id: u32) -> Option<GamePad> {
        if let Some(info) = SFMLJoystick::new(id) {
            // Found a connected joystick
            debug!("Updating active gamepad config to use ID {}...", id);
            if let Some(sdl_db) = GampadDB::get("gamecontrollerdb.txt") {
                debug!("Found controller DB!");
                if let Ok(db_str ) = std::str::from_utf8(sdl_db.data.as_ref()) {
                    if let Some(found_gp) = GamePad::from_gamepad_db(info.id, info.vendor, info.product, db_str) {
                        debug!("Found gamepad! id: {}, name: {}, vendor: {:#x}, product: {:#x}", found_gp.id(), found_gp.name(), found_gp.vendor_id(), found_gp.product_id());
                        Some(found_gp)
                    } else {
                        warn!("Could not find controller in Gamepad DB!");
                        None
                    }
                } else {
                    warn!("Could not parse embedded Gamepad DB!");
                    None
                }
            } else {
                warn!("Could not fetch embedded Gamepad DB!");
                None
            }
        } else {
            trace!("Gamepad {} not found!", id);
            None
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

    fn check_if_joystick_is_connected(&mut self) {
        if let Some(gp) = self.gamepad.as_ref() {
            if !joystick::is_connected(gp.id()) {
                debug!("Gamepad {} ({}) disconnected! Searching for updated Gamepad!", gp.id(), gp.name());
                if let Some(mut updated_gp) = Self::get_updated_gamepad(gp.id()) {
                    debug!("Found new Gamepad {} ({})!", updated_gp.id(), updated_gp.name());
                    updated_gp.update_state();
                    self.gamepad = Some(updated_gp);
                } else {
                    debug!("Could not find a new Gamepad!");
                    self.gamepad = None;
                } 
            }
        } else {
            if self.last_check_time.elapsed() >= self.gamepad_poll {
                for id in 0..joystick::COUNT {
                    if let Some(mut new_gp) = Self::get_updated_gamepad(id) {
                        debug!("Gamepad {} ({}) connected!", new_gp.id(), new_gp.name());
                        new_gp.update_state();
                        self.gamepad = Some(new_gp);
                        break
                    }
                }
                self.last_check_time = Instant::now();
            }
        }
    }

    fn get_gamepad_events(gamepad: &mut GamePad, mouse_stick: &GamepadMouseStick) -> (Vec<DeviceEvent>, Vec<KeyboardEvent>){
        gamepad.update_state();
        let mut device_events = Vec::new();
        let mut keyboard_events = Vec::new();
        for event in gamepad.get_events() {
            match event {
                JoystickEvent::Axis(axis_event) => {
                    debug!("Axis {:?} updated to {}", axis_event.event_type(), axis_event.state());
                    match mouse_stick {
                        GamepadMouseStick::Left => {
                            if *axis_event.event_type() == AxisType::LeftX {
                                let pos = (*axis_event.state() + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                device_events.push(DeviceEvent::AxisXMoved(pos));
                            } else if *axis_event.event_type() == AxisType::LeftY {
                                let pos = (*axis_event.state() + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                device_events.push(DeviceEvent::AxisYMoved(pos));
                            }
                        },
                        GamepadMouseStick::Right => {
                            if *axis_event.event_type() == AxisType::RightX {
                                let pos = (*axis_event.state() + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                device_events.push(DeviceEvent::AxisXMoved(pos));
                            } else if *axis_event.event_type() == AxisType::RightY {
                                let pos = (*axis_event.state() + MAX_AXIS_VAL) / (MAX_AXIS_VAL * 2.0); // Double the size to account for negatives
                                device_events.push(DeviceEvent::AxisYMoved(pos));
                            }
                        }
                    }
                },
                JoystickEvent::Button(button_event) => {
                    if *button_event.state() {
                        keyboard_events.push(KeyboardEvent::ButtonPressed(button_event.event_type().clone()));
                    } else {
                        keyboard_events.push(KeyboardEvent::ButtonReleased(button_event.event_type().clone()));
                    }
                    
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

        self.check_if_joystick_is_connected();
        if let Some(gamepad) = self.gamepad.as_mut() {
            (gamepad_device_events, gamepad_keyboard_events) = Self::get_gamepad_events(gamepad, &self.mouse_stick);
        }

        if !gamepad_device_events.is_empty() && !gamepad_keyboard_events.is_empty() {
            mouse_events.push(DeviceEvent::DeviceChanged(DeviceType::GamePad));
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
