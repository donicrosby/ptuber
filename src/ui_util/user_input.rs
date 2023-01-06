use log::{debug, warn, info};
use rdev::{listen, Button, Event, EventType, Key};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::panic;

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
    KeyPressed(Key),
    KeyReleased(Key),
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

    pub fn start(
        &mut self,
        shutdown_rx: Receiver<InputGrabRunFlag>,
        mouse_tx: Sender<MouseEvent>,
        keyboard_tx: Sender<KeyboardEvent>,
    ) {
        let thread = thread::spawn(move || {
            let shutdown_rx = Arc::new(Mutex::new(shutdown_rx));
            let flag = shutdown_rx.lock().unwrap().recv().unwrap();
            let flag = Arc::new(Mutex::new(flag));
            let mouse_tx = mouse_tx;
            let keyboard_tx = keyboard_tx;
            let listen_flag = flag.clone();

            let event_callback = move |e: Event| {
                match e.event_type {
                    EventType::ButtonPress(b) => {
                        let mouse_event = match b {
                            Button::Left => MouseEvent::LeftPressed,
                            Button::Right => MouseEvent::RightPressed,
                            Button::Middle => MouseEvent::UnknownPressed(2),
                            Button::Unknown(u) => MouseEvent::UnknownPressed(u.into()),
                        };
                        mouse_tx
                            .send(mouse_event)
                            .map_err(|err| warn!("Mouse Input Error: {:?}", err))
                            .expect("mouse down TX");
                    }
                    EventType::ButtonRelease(b) => {
                        let mouse_event = match b {
                            Button::Left => MouseEvent::LeftReleased,
                            Button::Right => MouseEvent::RightReleased,
                            Button::Middle => MouseEvent::UnknownReleased(2),
                            Button::Unknown(u) => MouseEvent::UnknownReleased(u.into()),
                        };
                        mouse_tx
                            .send(mouse_event)
                            .map_err(|err| warn!("Mouse Input Error: {:?}", err))
                            .expect("mouse up TX");
                    }
                    EventType::KeyPress(k) => {
                        keyboard_tx
                            .send(KeyboardEvent::KeyPressed(k))
                            .map_err(|err| warn!("Keyboard Input Error: {:?}", err))
                            .expect("keyboard down TX");
                    }
                    EventType::KeyRelease(k) => {
                        keyboard_tx
                            .send(KeyboardEvent::KeyReleased(k))
                            .map_err(|err| warn!("Keyboard Input Error: {:?}", err))
                            .expect("keyboard up TX");
                    }
                    _ => {}
                }
                let shutting_down = {
                    if let Ok(mut flag) = listen_flag.lock() {
                        if let Ok(shutdown_rx) = shutdown_rx.lock() {
                            if let Ok(recvd_flag) = shutdown_rx.try_recv() {
                                *flag = recvd_flag;
                            }
                        }
                        match *flag {
                            InputGrabRunFlag::Run => false,
                            InputGrabRunFlag::Halt => true,
                        }
                    } else {
                        false
                    }
                };

                if shutting_down {
                    panic!("Shutting down")
                }
            };
            loop {
                let cur_flag = { *flag.lock().unwrap() };

                match cur_flag {
                    InputGrabRunFlag::Halt => {
                        debug!("Grabber exiting!");
                        break;
                    }
                    InputGrabRunFlag::Run => {
                        if let Err(error) = listen(event_callback.clone()) {
                            warn!("Error: {:?}", error)
                        }
                    }
                }
            }
        });
        self.thread = Some(thread)
    }

    pub fn shutdown(&mut self) {
        if let Some(thread) = self.thread.take() {
            debug!("Joining input monitor thread!");
            if let Err(_) = thread.join() {
                info!("Shutting down ignoring input thread panic...");
            }
        
            debug!("Thread joined!");
        }
    }
}
