#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Input Driver
// Hardware abstraction for input devices

use crate::security::CapabilityToken;

/// Input device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Keyboard,
    Mouse,
    Touchscreen,
}

/// Input event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    KeyPress {
        keycode: u8,
        ascii: Option<char>,
    },
    KeyRelease {
        keycode: u8,
    },
    MouseMove {
        delta_x: i32,
        delta_y: i32,
    },
    MouseClick {
        left: bool,
        right: bool,
        middle: bool,
    },
    Touch {
        x: u32,
        y: u32,
    },
}

/// PS/2 Scancode to ASCII translator (Set 1)
pub struct ScancodeTranslator {}

impl ScancodeTranslator {
    pub fn translate(scancode: u8) -> Option<char> {
        match scancode {
            0x1E => Some('a'),
            0x30 => Some('b'),
            0x2E => Some('c'),
            0x20 => Some('d'),
            0x12 => Some('e'),
            0x21 => Some('f'),
            0x22 => Some('g'),
            0x23 => Some('h'),
            0x17 => Some('i'),
            0x24 => Some('j'),
            0x25 => Some('k'),
            0x26 => Some('l'),
            0x32 => Some('m'),
            0x31 => Some('n'),
            0x18 => Some('o'),
            0x19 => Some('p'),
            0x10 => Some('q'),
            0x13 => Some('r'),
            0x1F => Some('s'),
            0x14 => Some('t'),
            0x16 => Some('u'),
            0x2F => Some('v'),
            0x11 => Some('w'),
            0x2D => Some('x'),
            0x15 => Some('y'),
            0x2C => Some('z'),
            0x39 => Some(' '),  // Space
            0x1C => Some('\n'), // Enter
            _ => None,
        }
    }
}

/// PS/2 3-byte Mouse Packet Parser
pub struct MousePacketParser {
    state: u8,
    bytes: [u8; 3],
    pub sensitivity: f32,
}

impl MousePacketParser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            state: 0,
            bytes: [0; 3],
            sensitivity: 1.0,
        }
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }

    /// Push a raw byte from the PS/2 controller and return an event if packet is complete
    pub fn push_byte(&mut self, byte: u8) -> Option<InputEvent> {
        // Sync check: bit 3 of first byte must be 1
        if self.state == 0 && (byte & 0x08) == 0 {
            return None;
        }

        self.bytes[self.state as usize] = byte;
        self.state += 1;

        if self.state == 3 {
            self.state = 0;

            let flags = self.bytes[0];
            let raw_x = self.bytes[1];
            let raw_y = self.bytes[2];

            let left = (flags & 0x01) != 0;
            let right = (flags & 0x02) != 0;
            let middle = (flags & 0x04) != 0;

            // X and Y delta sign-extensions with sensitivity scaling
            let delta_x = if (flags & 0x10) != 0 {
                (((raw_x as i32 - 256) as f32) * self.sensitivity) as i32
            } else {
                ((raw_x as i32) as f32 * self.sensitivity) as i32
            };

            let delta_y = if (flags & 0x20) != 0 {
                -((((raw_y as i32 - 256) as f32) * self.sensitivity) as i32)
            } else {
                -(((raw_y as i32) as f32 * self.sensitivity) as i32)
            };

            if left || right || middle {
                return Some(InputEvent::MouseClick {
                    left,
                    right,
                    middle,
                });
            } else {
                return Some(InputEvent::MouseMove { delta_x, delta_y });
            }
        }

        None
    }
}

/// Input driver interface
pub struct InputDriver {
    pub device_type: InputType,
    pub capabilities: CapabilityToken,
    pub event_buffer: Vec<InputEvent>,
    pub mouse_parser: MousePacketParser,
}

impl InputDriver {
    pub fn new(device_type: InputType) -> Self {
        Self {
            device_type,
            capabilities: CapabilityToken::new(),
            event_buffer: Vec::new(),
            mouse_parser: MousePacketParser::new(),
        }
    }

    pub fn poll_event(&mut self) -> Option<InputEvent> {
        self.event_buffer.pop()
    }

    pub fn push_event(&mut self, event: InputEvent) {
        self.event_buffer.push(event);
    }

    pub fn set_capabilities(&mut self, capabilities: CapabilityToken) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities.bits() & capability) != 0
    }

    pub fn clear_buffer(&mut self) {
        self.event_buffer.clear();
    }
}

impl Default for InputDriver {
    fn default() -> Self {
        Self::new(InputType::Keyboard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_creation() {
        let input = InputDriver::new(InputType::Keyboard);
        assert_eq!(input.device_type, InputType::Keyboard);
    }

    #[test]
    fn test_event_buffer() {
        let mut input = InputDriver::new(InputType::Keyboard);
        let event = InputEvent::KeyPress {
            keycode: 65,
            ascii: None,
        };
        input.push_event(event.clone());
        let polled = input.poll_event();
        assert!(polled.is_some());
    }

    #[test]
    fn test_clear_buffer() {
        let mut input = InputDriver::new(InputType::Mouse);
        input.push_event(InputEvent::MouseMove {
            delta_x: 10,
            delta_y: -5,
        });
        input.clear_buffer();
        assert!(input.poll_event().is_none());
    }

    #[test]
    fn test_scancode_translation() {
        assert_eq!(ScancodeTranslator::translate(0x1E), Some('a'));
        assert_eq!(ScancodeTranslator::translate(0x39), Some(' '));
    }

    #[test]
    fn test_mouse_parsing() {
        let mut parser = MousePacketParser::new();
        // Send a complete mouse packet (0x08 sync flag, delta X = 5, delta Y = 10)
        assert_eq!(parser.push_byte(0x08), None);
        assert_eq!(parser.push_byte(0x05), None);

        let event = parser.push_byte(0x0A).unwrap();
        assert_eq!(
            event,
            InputEvent::MouseMove {
                delta_x: 5,
                delta_y: -10
            }
        );
    }

    #[test]
    fn test_mouse_sensitivity() {
        let mut parser = MousePacketParser::new();
        parser.set_sensitivity(2.5);
        assert_eq!(parser.push_byte(0x08), None);
        assert_eq!(parser.push_byte(0x02), None);

        let event = parser.push_byte(0x04).unwrap();
        assert_eq!(
            event,
            InputEvent::MouseMove {
                delta_x: 5,   // 2 * 2.5
                delta_y: -10  // - (4 * 2.5)
            }
        );
    }
}
