use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};

use log::{warn, debug};
use rdev::{listen, Event, EventType, Button, Key};

#[derive(Debug)]
pub enum MouseEvent {
    LeftPressed,
    RightPressed,
    LeftReleased,
    RightReleased
}

#[derive(Debug)]
pub enum KeyboardEvent {
    KeyPressed(Key),
    KeyReleased(Key)
}

#[derive(Debug)]
pub enum AvatarMoveEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    None
}

#[derive(Debug)]
pub enum InputGramRunFlag {
    Run,
    Halt
}

#[derive(Debug)]
pub struct InputGrabber {
    thread: Option<JoinHandle<()>>,
}

fn convert_to_move_event(event: Event) -> AvatarMoveEvent {
    match event.event_type {
        EventType::KeyPress(k) => {
           AvatarMoveEvent::Keyboard(KeyboardEvent::KeyPressed(k))
        },
        EventType::KeyRelease(k) => {
            AvatarMoveEvent::Keyboard(KeyboardEvent::KeyReleased(k))
        },
        EventType::ButtonPress(b ) => {
            match b {
                Button::Left => AvatarMoveEvent::Mouse(MouseEvent::LeftPressed),
                Button::Right => AvatarMoveEvent::Mouse(MouseEvent::RightPressed),
                _ => AvatarMoveEvent::None
            }
        },
        EventType::ButtonRelease(b) => {
            match b {
                Button::Left => AvatarMoveEvent::Mouse(MouseEvent::LeftReleased),
                Button::Right => AvatarMoveEvent::Mouse(MouseEvent::RightReleased),
                _ => AvatarMoveEvent::None
            }
        },
        _ => AvatarMoveEvent::None
    }
}

impl InputGrabber {
    pub fn new() -> Self {
        
        Self {
            thread: None
        }
    }
    
    pub fn start(&mut self, shutdown_rx: Receiver<InputGramRunFlag>, mouse_tx: Sender<MouseEvent>, keyboard_tx: Sender<KeyboardEvent>) {
        let thread = thread::spawn(move || {
            let callback = move |event: Event| {
                match convert_to_move_event(event) {
                    AvatarMoveEvent::Keyboard(e) => {
                        keyboard_tx.send(e).map_err(|err| warn!("Keyboard Input Error: {:?}", err)).expect("keyboard RX");
                    },
                    AvatarMoveEvent::Mouse(e) => {
                        mouse_tx.send(e).map_err(|err| warn!("Mouse Input Error: {:?}", err)).expect("mouse RX");
                    },
                    AvatarMoveEvent::None => {}
                }
                
            };

            let mut flag = shutdown_rx.recv().unwrap();
            loop {
                match flag {
                    InputGramRunFlag::Halt => {
                        debug!("Grabber exiting!");
                        return;
                    },
                    InputGramRunFlag::Run => {
                        debug!("Listening for input!");
                        match listen(callback.clone()) {
                            Err(err) => {
                                warn!("Input Event Error: {:?}", err);
                            },
                            Ok(_) => {
                                debug!("Listening Okay!");
                            }
                        }
                        if let Ok(recvd_flag) = shutdown_rx.try_recv(){
                            flag = recvd_flag;
                        }
                        debug!("Grab Flag: {:?}", flag);
                    }
                }
            }
        });
        self.thread = Some(thread)
    }

    pub fn shutdown(&mut self) {
        if let Some(thread) = self.thread.take() {
            debug!("Joining input monitor thread!");
            // thread.join().expect("joining input thread");
            debug!("Thread joined!");
        }
    }
}