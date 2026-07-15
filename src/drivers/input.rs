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
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress { keycode: u8 },
    KeyRelease { keycode: u8 },
    MouseMove { x: u32, y: u32 },
    MouseClick { button: u8 },
    Touch { x: u32, y: u32 },
}

/// Input driver interface
pub struct InputDriver {
    pub device_type: InputType,
    pub capabilities: CapabilityToken,
    pub event_buffer: Vec<InputEvent>,
}

impl InputDriver {
    pub fn new(device_type: InputType) -> Self {
        Self {
            device_type,
            capabilities: CapabilityToken::new(),
            event_buffer: Vec::new(),
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
        (self.capabilities.bits & capability) != 0
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
        let event = InputEvent::KeyPress { keycode: 65 };
        input.push_event(event.clone());
        let polled = input.poll_event();
        assert!(polled.is_some());
    }

    #[test]
    fn test_clear_buffer() {
        let mut input = InputDriver::new(InputType::Mouse);
        input.push_event(InputEvent::MouseMove { x: 100, y: 200 });
        input.clear_buffer();
        assert!(input.poll_event().is_none());
    }
}
