use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[cfg(target_os = "linux")]
use super::linux::MouseButtonType;
#[cfg(target_os = "windows")]
use super::windows::MouseButtonType;
use device_query::{DeviceEvents, DeviceState, Keycode, MouseButton};
use log::{debug, trace, warn};

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

#[derive(Debug)]
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

    pub fn start(
        &mut self,
        shutdown_rx: Receiver<InputGrabRunFlag>,
        mouse_tx: Sender<MouseEvent>,
        keyboard_tx: Sender<KeyboardEvent>,
    ) {
        let thread = thread::spawn(move || {
            let mouse_tx = Arc::new(Mutex::new(mouse_tx));
            let mouse_down_tx = mouse_tx.clone();
            let mouse_up_tx = mouse_tx;

            let mouse_down_callback = move |b: &MouseButton| {
                debug!("Got Mouse Down Button: {}", b);
                let button_type = (*b).into();
                let e = match button_type {
                    MouseButtonType::Left => MouseEvent::LeftPressed,
                    MouseButtonType::Right => MouseEvent::RightPressed,
                    MouseButtonType::Middle => MouseEvent::UnknownPressed(2),
                    MouseButtonType::Unknown(u) => MouseEvent::UnknownPressed(u),
                };
                if let Ok(tx) = mouse_down_tx.lock() {
                    debug!("Sending Mouse Event: {:?}", e);
                    tx.send(e)
                        .map_err(|err| warn!("Mouse Input Error: {:?}", err))
                        .expect("mouse down TX");
                }
            };

            let mouse_up_callback = move |b: &MouseButton| {
                debug!("Got Mouse Up Button: {}", b);
                let button_type = (*b).into();
                let e = match button_type {
                    MouseButtonType::Left => MouseEvent::LeftReleased,
                    MouseButtonType::Right => MouseEvent::RightReleased,
                    MouseButtonType::Middle => MouseEvent::UnknownReleased(2),
                    MouseButtonType::Unknown(u) => MouseEvent::UnknownReleased(u),
                };
                if let Ok(tx) = mouse_up_tx.lock() {
                    debug!("Sending Mouse Event: {:?}", e);
                    tx.send(e)
                        .map_err(|err| warn!("Mouse Input Error: {:?}", err))
                        .expect("mouse up TX");
                }
            };

            let keyboard_tx = Arc::new(Mutex::new(keyboard_tx));
            let keyboard_down_tx = keyboard_tx.clone();
            let keyboard_up_tx = keyboard_tx;

            let keyboard_down_callback = move |k: &Keycode| {
                debug!("Got Key Down: {:?}", k);
                if let Ok(tx) = keyboard_down_tx.lock() {
                    tx.send(KeyboardEvent::KeyPressed(*k))
                        .map_err(|err| warn!("Keyboard Input Error: {:?}", err))
                        .expect("keyboard down TX");
                }
            };

            let keyboard_up_callback = move |k: &Keycode| {
                debug!("Got Key Up: {:?}", k);
                if let Ok(tx) = keyboard_up_tx.lock() {
                    tx.send(KeyboardEvent::KeyReleased(*k))
                        .map_err(|err| warn!("Keyboard Input Error: {:?}", err))
                        .expect("keyboard up TX");
                }
            };

            let device_state = DeviceState::new();
            let _m_down_guard = device_state.on_mouse_down(mouse_down_callback);
            let _m_up_guard = device_state.on_mouse_up(mouse_up_callback);
            let _k_down_guard = device_state.on_key_down(keyboard_down_callback);
            let _k_up_guard = device_state.on_key_up(keyboard_up_callback);

            let mut flag = shutdown_rx.recv().unwrap();
            loop {
                match flag {
                    InputGrabRunFlag::Halt => {
                        debug!("Grabber exiting!");
                        break;
                    }
                    InputGrabRunFlag::Run => {
                        if let Ok(recvd_flag) = shutdown_rx.try_recv() {
                            flag = recvd_flag;
                        }
                        trace!("Grab Flag: {:?}", flag);
                    }
                }
            }
        });
        self.thread = Some(thread)
    }

    pub fn shutdown(&mut self) {
        if let Some(thread) = self.thread.take() {
            debug!("Joining input monitor thread!");
            thread.join().expect("joining input thread");
            debug!("Thread joined!");
        }
    }
}
