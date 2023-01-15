use log::{debug, warn};
use getset::Getters;
use sfml::window::joystick::Axis as SFMLAxis;

use super::types::{Buttonish, Axisish, DPad, Button, Axis};

const GUID_CHUNKS: usize = 4; // 4 chars make up a 16 bit uint in hex
const HEX_RADIX: u32 = 16;

const AXIS_INDICATOR: &str = "a";
const BUTTON_INDICATOR: &str = "b";
const DPAD_INDICATOR: &str = "h";
const DPAD_SPLIT_VALUE: &str = "h0."; 

pub enum AxisOrButton {
    Button(Box<dyn Buttonish>),
    Axis(Box<dyn Axisish>)
}


#[derive(Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct Guid {
    vendor: u16,
    product: u16,
    version: u16,
    crc: u16,
}

impl Guid {
    pub fn from_guid_string(guid_str: &str) -> Option<Self> {
        parse_guid(guid_str)
    }
}

// Parses SDL GUID based on the standard form:
//    16-bit bus
//    16-bit CRC16 of the joystick name (can be zero)
//    16-bit vendor ID
//    16-bit zero
//    16-bit product ID
//    16-bit zero
//    16-bit version
//    8-bit driver identifier ('h' for HIDAPI, 'x' for XInput, etc.)
//    8-bit driver-dependent type info
// Returns None if not in standard form
fn parse_guid(guid_str: &str) -> Option<Guid> {
    // Break the guid into 4 char strings
    let chunks: Vec<String> = guid_str.chars().collect::<Vec<char>>().chunks(GUID_CHUNKS).map(|c| c.iter().collect::<String>()).collect();
    
    // Convert the Big Endian form to Little Endian
    let chunks: Vec<Option<u16>> = chunks.iter().map(|c| {
        let res = u16::from_str_radix(c, HEX_RADIX).ok();
        if let Some(u) = res {
            Some(u.swap_bytes())
        } else {
            res
        }
    }).collect();

    if !chunks.iter().all(|u| u.is_some()) {
        // radix parsing failed
        return None
    }

    let chunks: Vec<u16> = chunks.iter().map(|u| u.unwrap()).collect();

    let (zero1, zero2) = (chunks[3], chunks[5]);
    if zero1 != 0 || zero2 != 0 {
        debug!("SDL GUID Parse: zero bytes not actually zero! Got: ({}, {})", zero1, zero2);
        None
    } else {
        let (vendor, product, version, crc) = (chunks[2], chunks[4], chunks[6], chunks[1]);
        Some(Guid {vendor, product, version, crc})
    }
}

fn parse_axis(name: &str, mapping: &str) -> Option<Axis> {
    let axis_info: Vec<&str> = mapping.split(AXIS_INDICATOR).collect(); // split -a0
        let sign;
        let axis_mapping;

        if axis_info.len() > 1 { 
            // from -a0 split gave us [-, 0]
            sign = axis_info[0];
            axis_mapping = axis_info[1];
        } else { 
            //from a0 split gave us [0]
            sign = "";
            axis_mapping = axis_info[0]; 
        }
        
        let mut inverted = false;
        if sign == "-" {
            inverted = true;
        }
        
        if let Some(int) = axis_mapping.parse().ok() {
            let axis = match int {
                0 => Some(SFMLAxis::X),
                1 => Some(SFMLAxis::Y),
                2 => Some(SFMLAxis::Z),
                3 => Some(SFMLAxis::U),
                4 => Some(SFMLAxis::V),
                5 => Some(SFMLAxis::R),
                6 => {
                    warn!("Got PovX for an axis!");
                    None
                },
                7 => {
                    warn!("Got PovY for an axis!");
                    None
                }
                _ => None
            };
            if let Some(axis) = axis {
                let axis = Axis::new(axis, inverted, name.to_string());
                Some(axis)
            } else {
                None
            }   
        } else {
            None
        }
}

fn parse_button(name: &str, mapping: &str) -> Option<Button> {
    let button_info: Vec<&str> = mapping.split(BUTTON_INDICATOR).collect();
    let button_id;
    if button_info.len() > 1 {
        if !button_info[0].is_empty() {
            warn!("Got button that has a possible sign: {}", button_info[0]);
        }
        button_id = button_info[1];
    } else {
        button_id = button_info[0]
    }

    if let Some(id) = button_id.parse().ok() {
        let button = Button::new(id, name.to_string());
        Some(button)
    } else {
        None
    }
}

fn parse_dpad(name: &str, mapping: &str) -> Option<DPad> {
    let dpad_info: Vec<&str> = mapping.split(DPAD_SPLIT_VALUE).collect();
    let dpad_id;
    if dpad_info.len() > 1 {
        if !dpad_info[0].is_empty() {
            warn!("Got dpad that has a possible sign: {}", dpad_info[0]);
        }
        dpad_id = dpad_info[1];
    } else {
        dpad_id = dpad_info[0];
    }
    let axis;
    let negative;
    if let Some(id) = dpad_id.parse().ok() {
        match id {
            1 => {
                axis = SFMLAxis::PovY;
                negative = false;
            },
            2 => {
                axis = SFMLAxis::PovX;
                negative = false;
            },
            4 => {
                axis = SFMLAxis::PovY;
                negative = true;
            },
            8 => {
                axis = SFMLAxis::PovX;
                negative = true;
            },
            _ => return None
        }
        let dpad = DPad::new(axis, negative, name.to_string());
        Some(dpad)
    } else {
        None
    }
    
}

pub fn parse_axis_or_button(btn_axis_str: &str) -> Option<AxisOrButton> {
    let parts:Vec<&str> = btn_axis_str.split(":").collect(); // split a:b0
    if parts.len() != 2 {
        warn!("Button or Axis string incorrect format! Got: {:?}", parts);
        return None
    }
    let (name, mapping) = (parts[0], parts[1]); // gets name (a) and mapping (b0)
    if mapping.contains(AXIS_INDICATOR) {
        if let Some(axis) = parse_axis(name, mapping) {
            Some(AxisOrButton::Axis(Box::new(axis)))
        } else {
            None
        }
    } else if mapping.contains(BUTTON_INDICATOR) {
        if let Some(button) = parse_button(name, mapping) {
            Some(AxisOrButton::Button(Box::new(button)))
        } else {
            None
        }
    } else if mapping.contains(DPAD_INDICATOR) {
        if let Some(dpad) = parse_dpad(name, mapping) {
            Some(AxisOrButton::Button(Box::new(dpad)))
        } else {
            None
        }
    } else {
        None
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_guid() {
        let guid_str = "0300509d5e040000130b000013057200";
        let expected_guid = Guid { vendor: 0x045e, product: 0x0b13, version: 0x0513, crc: 0x9d50};
        let result_guid = parse_guid(guid_str);
        assert!(result_guid.is_some());
        assert_eq!(expected_guid, result_guid.unwrap());
    }

    #[test]
    fn test_parse_guid_zeros_not_zero() {
        let guid_str_1 = "0300509d5e040100130b000013057200";
        let guid_str_2 = "0300509d5e040000130b0a0013057200";

        let result_guid = parse_guid(guid_str_1);
        assert!(result_guid.is_none());

        let result_guid = parse_guid(guid_str_2);
        assert!(result_guid.is_none());
    }

    #[test]
    fn test_parse_guid_radix_fails() {
        let guid_str = "030z509d5e040000130b000013057200";

        let result_guid = parse_guid(guid_str);
        assert!(result_guid.is_none());
    }

    #[test]
    fn test_parse_axis() {
        let axis_str = "a2";
        let expected_axis = Axis::new(SFMLAxis::Z, false, String::from("rightx"));
        let result_axis = parse_axis("rightx", axis_str);

        assert!(result_axis.is_some());
        assert_eq!(expected_axis, result_axis.unwrap());
    }

    #[test]
    fn test_parse_negative_axis() {
        let axis_str = "-a2";
        let expected_axis = Axis::new(SFMLAxis::Z, true, String::from("rightx"));
        let result_axis = parse_axis("rightx", axis_str);

        assert!(result_axis.is_some());
        assert_eq!(expected_axis, result_axis.unwrap());
    }


    #[test]
    fn test_parse_positive_axis() {
        let axis_str = "+a2";
        let expected_axis = Axis::new(SFMLAxis::Z, false, String::from("rightx"));
        let result_axis = parse_axis("rightx", axis_str);

        assert!(result_axis.is_some());
        assert_eq!(expected_axis, result_axis.unwrap());
    }

    #[test]
    fn test_parse_wrong_axis() {
        let axis_str = "a10";
        let result_axis = parse_axis("rightx", axis_str);

        assert!(result_axis.is_none());

        let axis_str = "b10";
        let result_axis = parse_axis("rightx", axis_str);

        assert!(result_axis.is_none());        
    }

    #[test]
    fn test_parse_button() {
        let axis_str = "b2";
        let expected_button= Button::new(2, String::from("a"));
        let result_button= parse_button("a", axis_str);

        assert!(result_button.is_some());
        assert_eq!(expected_button, result_button.unwrap());
    }

    #[test]
    fn test_parse_wrong_button() {
        let axis_str = "a2";
        let result_button= parse_button("a", axis_str);

        assert!(result_button.is_none());
    }


    #[test]
    fn test_parse_dpad() {
        let dpad_str = "h0.4";
        let expected = DPad::new(SFMLAxis::PovY,true,  String::from("dpdown"));
        let result = parse_dpad("dpdown", dpad_str);

        assert!(result.is_some());
        assert_eq!(expected, result.unwrap());

        let dpad_str = "h0.8";
        let expected = DPad::new(SFMLAxis::PovX,true,  String::from("dpleft"));
        let result = parse_dpad("dpleft", dpad_str);

        assert!(result.is_some());
        assert_eq!(expected, result.unwrap());

        let dpad_str = "h0.2";
        let expected = DPad::new(SFMLAxis::PovX, false,  String::from("dpright"));
        let result = parse_dpad("dpright", dpad_str);

        assert!(result.is_some());
        assert_eq!(expected, result.unwrap());

        let dpad_str = "h0.1";
        let expected = DPad::new(SFMLAxis::PovY, false,  String::from("dpup"));
        let result = parse_dpad("dpup", dpad_str);

        assert!(result.is_some());
        assert_eq!(expected, result.unwrap());
    }

}