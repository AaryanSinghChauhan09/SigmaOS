//! Input Module
//!
//! Linux input event (evdev) interface for SigmaOS, providing device event
//! reading from input hardware. Inspired by the Linux kernel's input subsystem
//! and the evdev device interface, allowing SigmaOS to process keyboard, mouse,
//! and other input events from evdev-compatible devices.
//!
//! This module absorbs the Linux input event subsystem idea and provides a
//! no_std-compatible interface for reading input events.

pub mod event;

pub use event::{
    enumerate_devices, InputCode, InputDeviceInfo, InputEvent, InputEventReader, InputEventType,
    InputProductId, InputTimestamp, InputValue, KeyState, ABS_CNT, ABS_CODE_RANGE,
    AVAILABLE_EVENT_TYPES, KEY_CODE_RANGE, KEY_MAX, MAX_DEVICE_NAME, REL_CNT, REL_CODE_RANGE,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_module_imports() {
        // Verify all public items are accessible
        let _event = InputEvent {
            timestamp: InputTimestamp { sec: 0, usec: 0 },
            evtype: InputEventType::Key,
            code: InputCode::Key(0x00),
            value: InputValue(0),
        };
        let _timestamp = InputTimestamp { sec: 0, usec: 0 };
        let _value = InputValue(0);
        let _etype = InputEventType::Key;
        let _code = InputCode::Key(0x00);
        let _reader = InputEventReader::new("/dev/input/event0");
        let _info = InputDeviceInfo {
            name: [0u8; 80],
            product: InputProductId {
                vendor: 1,
                product: 1,
                version: 1,
            },
            version: 1,
        };
        let _kd = KeyState::Pressed;
    }
}
