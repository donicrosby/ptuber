use super::MouseButtonImpl;
use device_query::{DeviceQuery, DeviceState, Keycode};
use log::{debug, error, info, warn};
use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::MAX_FRAMERATE;

const POLL_RATE_RATIO: f32 = 2.0;

#[derive(Debug)]
pub enum MouseEvent {
    LeftPressed,
    RightPressed,
    LeftReleased,
    RightReleased,
    UnknownPressed(usize),
    UnknownReleased(usize),
}

#[derive(Debug)]
pub enum KeyboardEvent {
    KeyPressed(Keycode),
    KeyReleased(Keycode),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InputGrabRunFlag {
    Run,
    Halt,
}

#[derive(Debug)]
pub struct InputGrabber {
    thread: Option<JoinHandle<()>>,
}

impl InputGrabber {
    pub fn new() -> Self {
        Self { thread: None }
    }

    fn query_mouse(
        tx: &Sender<MouseEvent>,
        device_state: &DeviceState,
        mouse_state: &mut HashSet<MouseButtonImpl>,
    ) {
        let cur_state = device_state.get_mouse();
        for (b, pressed) in cur_state.button_pressed.iter().enumerate() {
            // First button never used
            if b > 0 {
                let button = b.into();
                if *pressed {
                    // Only trigger on new presses of the button
                    if !mouse_state.insert(button) {
                        let event = match button {
                            MouseButtonImpl::Left => MouseEvent::LeftPressed,
                            MouseButtonImpl::Middle => MouseEvent::UnknownPressed(2),
                            MouseButtonImpl::Right => MouseEvent::RightPressed,
                            MouseButtonImpl::XButton1 => MouseEvent::UnknownPressed(4),
                            MouseButtonImpl::XButton2 => MouseEvent::UnknownPressed(5),
                            MouseButtonImpl::Unknown(v) => MouseEvent::UnknownPressed(v),
                        };
                        tx.send(event)
                            .map_err(|err| warn!("Mouse Input Error: {:?}", err))
                            .expect("Sending mouse down event");
                    }
                } else if mouse_state.remove(&button) {
                    let event = match button {
                        MouseButtonImpl::Left => MouseEvent::LeftReleased,
                        MouseButtonImpl::Middle => MouseEvent::UnknownReleased(2),
                        MouseButtonImpl::Right => MouseEvent::RightReleased,
                        MouseButtonImpl::XButton1 => MouseEvent::UnknownReleased(4),
                        MouseButtonImpl::XButton2 => MouseEvent::UnknownReleased(5),
                        MouseButtonImpl::Unknown(v) => MouseEvent::UnknownReleased(v),
                    };
                    tx.send(event)
                        .map_err(|err| warn!("Mouse Input Error: {:?}", err))
                        .expect("Sending mouse up event");
                }
            }
        }
    }

    fn query_keyboard(
        tx: &Sender<KeyboardEvent>,
        device_state: &DeviceState,
        keyboard_state: &mut HashSet<Keycode>,
    ) {
        let cur_state = device_state.get_keys();
        let mut keys_to_remove = Vec::new();
        for code in keyboard_state.iter() {
            //Previous state doesn't contain key so released
            if !cur_state.contains(code) {
                let event = KeyboardEvent::KeyReleased(*code);
                keys_to_remove.push(*code);
                tx.send(event)
                    .map_err(|err| warn!("Keyboard Input Error: {:?}", err))
                    .expect("Sending keyboard up event");
            }
        }

        for code in keys_to_remove.iter() {
            keyboard_state.remove(code);
        }

        for code in cur_state.iter() {
            //Previous state did not contain key so pressed
            if !keyboard_state.insert(*code) {
                let event = KeyboardEvent::KeyPressed(*code);
                tx.send(event)
                    .map_err(|err| warn!("Keyboard Input Error: {:?}", err))
                    .expect("Sending keyboard down event");
            }
        }
    }

    pub fn start(
        &mut self,
        shutdown_rx: Receiver<InputGrabRunFlag>,
        mouse_tx: Sender<MouseEvent>,
        keyboard_tx: Sender<KeyboardEvent>,
    ) {
        let thread = thread::spawn(move || {
            let mouse_tx = mouse_tx;
            let keyboard_tx = keyboard_tx;
            let mut flag = shutdown_rx
                .recv()
                .map_err(|err| error!("Error getting initial shutdown flag {:?}", err))
                .expect("Grabber init");

            let device_state = DeviceState::new();
            let mut mouse_state = HashSet::new();
            let mut keyboard_state = HashSet::new();

            let mut now = Instant::now();
            let poll_internval =
                Duration::from_secs_f32(1.0 / (MAX_FRAMERATE as f32 * POLL_RATE_RATIO));
            let mut elapsed;

            info!("Starting input grabbing!");
            loop {
                if let Ok(shutdown_flag) = shutdown_rx.try_recv() {
                    flag = shutdown_flag;
                }
                match flag {
                    InputGrabRunFlag::Halt => {
                        debug!("Grabber exiting!");
                        break;
                    }
                    InputGrabRunFlag::Run => {
                        Self::query_mouse(&mouse_tx, &device_state, &mut mouse_state);
                        Self::query_keyboard(&keyboard_tx, &device_state, &mut keyboard_state);
                        elapsed = now.elapsed();
                        if elapsed < poll_internval {
                            thread::sleep(poll_internval - elapsed);
                        }
                    }
                }
                now = Instant::now();
            }
        });
        self.thread = Some(thread)
    }

    pub fn shutdown(&mut self) {
        // rdev doesn't currently have a way to shutdown the listening aspect
        // just take the thread and drop it to clean up :/
        if let Some(thread) = self.thread.take() {
            debug!("Joining input monitor thread!");
            match thread.join() {
                Ok(_) => {
                    debug!("Thread joined!");
                }
                Err(err) => {
                    error!("Error shutting down input thread: {:?}", err);
                }
            }
        }
    }
}
