use sfml::window::joystick::{self, Axis as SFMLAxis} ;
use getset::{Getters, MutGetters, CopyGetters};
use super::{Guid, AxisOrButton, parse_axis_or_button as sdl_axis_or_button_parse};
use std::fmt::Debug;
use log::{debug, warn, trace};
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;

pub const MAX_AXIS_VAL: f32 = 100.0;
const DPAD_TRIGGER_PERCENT: f32 = 0.75;

#[cfg(windows)]
const DB_PLATFORM_STR: &str = "# Windows";
#[cfg(all(unix, target_os = "linux"))]
const DB_PLATFORM_STR: &str = "# Linux";

#[derive(Debug, Clone, EnumString, Hash, Eq, PartialEq)]
pub enum AxisType {
    #[strum(ascii_case_insensitive)]
    LeftX,
    #[strum(ascii_case_insensitive)]
    LeftY,
    #[strum(ascii_case_insensitive)]
    LeftTrigger,
    #[strum(ascii_case_insensitive)]
    RightX,
    #[strum(ascii_case_insensitive)]
    RightY,
    #[strum(ascii_case_insensitive)]
    RightTrigger,
    #[strum(default)]
    UnknownAxis(String)
}

#[derive(Debug, Clone, EnumString, Hash, Eq, PartialEq)]
pub enum ButtonType {
    #[strum(ascii_case_insensitive)]
    A,
    #[strum(ascii_case_insensitive)]
    B,
    #[strum(ascii_case_insensitive)]
    X,
    #[strum(ascii_case_insensitive)]
    Y,
    #[strum(ascii_case_insensitive)]
    Back,
    #[strum(ascii_case_insensitive)]
    Start,
    #[strum(ascii_case_insensitive)]
    DPDown,
    #[strum(ascii_case_insensitive)]
    DPLeft,
    #[strum(ascii_case_insensitive)]
    DPRight,
    #[strum(ascii_case_insensitive)]
    DPUp,
    #[strum(ascii_case_insensitive)]
    LeftShoulder,
    #[strum(ascii_case_insensitive)]
    LeftStick,
    #[strum(ascii_case_insensitive)]
    RightShoulder,
    #[strum(ascii_case_insensitive)]
    RightStick,
    #[strum(default)]
    UnknownButton(String)
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct ButtonEvent {
    event_type: ButtonType,
    state: bool
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct AxisEvent {
    event_type: AxisType,
    state: f32
}


#[derive(Debug, Clone)]
pub enum JoystickEvent {
    Button(ButtonEvent),
    Axis(AxisEvent)
}

#[derive(Debug, Getters, MutGetters, CopyGetters)]
pub struct GamePad {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    vendor_id: u16,
    #[getset(get_copy = "pub")]
    product_id: u16,
    #[getset(get_mut)]
    button_state: HashMap<ButtonType, bool>,
    #[getset(get_mut)]
    axis_state: HashMap<AxisType, f32>,
    #[getset(get_mut)]
    button_events: Vec<ButtonEvent>,
    #[getset(get_mut)]
    axis_events: Vec<AxisEvent>,
    #[getset(get_mut, get)]
    buttons: Vec<Box<dyn Buttonish>>,
    #[getset(get_mut, get)]
    axis: Vec<Box<dyn Axisish>>
}

impl GamePad {
    pub fn from_gamepad_db(sfml_joystick_id: u32, vendor_id: u32, product_id: u32, db_lines: &str) -> Option<Self> {
        let mut finding_platform = true;
        for line in db_lines.lines() {
            if finding_platform && line != DB_PLATFORM_STR {
                continue;
            } else {
                if finding_platform {
                    debug!("Found platform in DB file");
                    finding_platform = false;
                    continue;
                }
                if (line.contains(char::is_whitespace) && !line.contains(char::is_alphanumeric)) || line.is_empty() {
                    debug!("Didn't find the controller in platform");
                    return None;
                } else {
                    trace!("{}", line);
                    match Self::from_gamepad_db_line(sfml_joystick_id, vendor_id, product_id, line) {
                        Some(gp) => {
                            debug!("Found a matching gamepad!");
                            return Some(gp)
                        },
                        None => continue
                    }
                }
            }
        }
        debug!("Reached the end of the file without a match");
        None
    }

    fn new(id: u32, name: String, vendor_id: u16, product_id: u16, buttons: Vec<Box<dyn Buttonish>>, axis: Vec<Box<dyn Axisish>>) -> Self {
        let button_state = HashMap::new();
        let axis_state = HashMap::new();
        let button_events = Vec::new();
        let axis_events = Vec::new();
        Self { id, name,  vendor_id, product_id, buttons, axis, button_state, button_events, axis_state, axis_events }
    }

    pub fn update_state(&mut self) {
        joystick::update();
        self.update_buttons();
        self.update_axis();
    }

    pub fn get_events(&mut self) -> Vec<JoystickEvent> {
        let mut events = Vec::new();
        let mut button_events = self.button_events_mut().iter().cloned().map(|e| JoystickEvent::Button(e)).collect();
        let mut axis_events = self.axis_events_mut().iter().cloned().map(|e| JoystickEvent::Axis(e)).collect();
        self.button_events_mut().clear();
        self.axis_events_mut().clear();
        
        events.append(&mut button_events);
        events.append(&mut axis_events);
        events
    }

    fn add_axis_event(&mut self, event: AxisEvent) {
        let events = self.axis_events_mut();
        events.push(event);
    }

    fn add_button_event(&mut self, event: ButtonEvent) {
        let events = self.button_events_mut();
        events.push(event);
    }

    fn get_axis_state(&self) -> Vec<AxisEvent> {
        let id = self.id;
        self.axis().iter().map(|axis| {
            let state = axis.position(id);
            let axis_type = AxisType::from_str(axis.name()).unwrap_or_else(|_| {
                warn!("Could not convert button name to axis type");
                AxisType::UnknownAxis(String::from("unknown_axis"))
            });
            AxisEvent{ event_type: axis_type.clone(), state: state }
        }).collect()
    }

    fn update_axis(&mut self) {
        let state = self.get_axis_state();
        let axis_state = self.axis_state_mut();
        let axis_events: Vec<_> = state.iter()
        .filter_map(|event| {
                let axis_type = event.event_type.clone();
                let state = event.state;
                match axis_state.insert(axis_type, state) {
                    Some(prev_state) => {
                        if state != prev_state {
                            Some(event.clone())
                        } else {
                            None
                        }
                    },
                    None => {
                        Some(event.clone())
                    }
                }
            }).collect();
        for event in axis_events {
            self.add_axis_event(event)
        }
        
    }

    fn get_button_state(&self) -> Vec<ButtonEvent> {
        let id = self.id;
        self.buttons().iter().map(|button| {
            let state = button.is_pressed(id);
            let button_type = ButtonType::from_str(button.name()).unwrap_or_else(|_| {
                warn!("Could not convert button name to axis type");
                ButtonType::UnknownButton(String::from("unknown_button"))
            });
            ButtonEvent{ event_type: button_type.clone(), state: state }
        }).collect()
    }

    fn update_buttons(&mut self) {
        let state= self.get_button_state();
        let button_state = self.button_state_mut();
        let button_events: Vec<_> = state.iter()
        .filter_map(|event| {
                let button_type = event.event_type.clone();
                let state = event.state;
                match button_state.insert(button_type, state) {
                    Some(prev_state) => {
                        if state != prev_state {
                            Some(event.clone())
                        } else {
                            None
                        }
                    },
                    None => {
                        Some(event.clone())
                    }
                }
            }).collect();
        for event in button_events {
            self.add_button_event(event);
        }
    }

    pub fn from_gamepad_db_line(sfml_joystick_id: u32, vendor_id: u32, product_id: u32, line: &str) -> Option<Self> {
        let mut parts: Vec<&str> = line.split(",").collect();
        let guid_str = parts.remove(0); // guid is first item
        let gamepad_name = parts.remove(0); // the name is the second item
 
        
        if let Some(guid) = Guid::from_guid_string(guid_str) {
            if vendor_id == *guid.vendor() as u32 && product_id == *guid.product() as u32 {
                let mut axis_vec = Vec::new();
                let mut button_vec = Vec::new();
                for button_or_axs in parts.iter() {
                    if button_or_axs.contains("platform") {
                        debug!("Found the end of the config");
                        break;
                    }
                   if let Some(b_or_a) =  sdl_axis_or_button_parse(*button_or_axs) {
                        match b_or_a {
                            AxisOrButton::Axis(a) => axis_vec.push(a),
                            AxisOrButton::Button(b) => button_vec.push(b)
                        };
                   }
                }
                let gamepad = GamePad::new(sfml_joystick_id, gamepad_name.to_string(), *guid.vendor(), *guid.product(), button_vec, axis_vec);
                Some(gamepad)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct SFMLJoystick {
    pub id: u32,
    pub vendor: u32,
    pub product: u32
}

impl SFMLJoystick {
    pub fn new(id: u32) -> Option<Self> {
        joystick::update(); // update joystick so that we can search for them
        if joystick::is_connected(id) {
            return Some(Self::parse_joystick_info(id))
        } else {
            for id in 0..joystick::COUNT {
                // Search for the first connected joystick and use that
                if joystick::is_connected(id) {
                    return Some(Self::parse_joystick_info(id))
                }
            }
        }
        // Could not find a connected joystick
        None
    }

    fn parse_joystick_info(id: u32) -> Self {
        let info = joystick::identification(id);
        let vendor = info.vendor_id();
        let product = info.product_id(); 
        Self {
            id,
            vendor,
            product
        }
    }
}


pub trait Buttonish: Debug {
    fn is_pressed(&self, gamepad_id: u32) -> bool;
    fn name(&self) -> &str;
}

pub trait Axisish: Debug {
    fn position(&self, gamepad_id: u32) -> f32;
    fn name(&self) -> &str;
    fn invert(&self, pos: f32) -> f32 {
        0.0 - pos
    }
    fn axis_id(&self) -> SFMLAxis;
}

#[derive(Debug,  Clone, PartialEq)]
pub struct Button {
    id: u32,
    button_name: String
}

impl Button {
    pub fn new(id: u32, button_name: String) -> Self {
        Self { id, button_name }
    }
}

impl Buttonish for Button {
    fn name(&self) -> &str {
        &self.button_name
    }
    fn is_pressed(&self, gamepad_id: u32) -> bool {
        joystick::is_button_pressed(gamepad_id, self.id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Axis {
    id: SFMLAxis,
    inverted: bool,
    axis_name: String
}

impl Axis {
    pub fn new(id: SFMLAxis, inverted: bool, axis_name: String) -> Self {
        Self {
            id, inverted, axis_name
        }
    }
    pub fn axis_name(&self) -> &str {
        &self.axis_name
    }
    pub fn id(&self) -> SFMLAxis {
        self.id
    }
}

impl Axisish for Axis {
    fn name(&self) -> &str {
        self.axis_name()
    }
    fn position(&self, gamepad_id: u32) -> f32 {
        let pos = joystick::axis_position(gamepad_id, self.id());
        if self.inverted {
            self.invert(pos)
        } else {
            pos
        }
    }
    fn axis_id(&self) -> SFMLAxis {
        self.id()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DPad {
    axis_id: SFMLAxis,
    is_negative: bool,
    direction_name: String,
}

impl DPad {
    pub fn new(axis_id: SFMLAxis, is_negative: bool, direction_name: String) -> Self {
        Self {
            axis_id, is_negative, direction_name
        }
    }
}

impl Buttonish for DPad {
    fn name(&self) -> &str {
        &self.direction_name
    }
    fn is_pressed(&self, gamepad_id: u32) -> bool {
        let pos = joystick::axis_position(gamepad_id, self.axis_id);
        let is_neg = pos < 0.0;
        if !self.is_negative {
            if !is_neg {
                pos > MAX_AXIS_VAL * DPAD_TRIGGER_PERCENT
            } else {
                false
            }
        } else {
            let pos = f32::abs(pos);
            if is_neg {
                pos > MAX_AXIS_VAL * DPAD_TRIGGER_PERCENT
            } else {
                false
            }
        }
    }
}