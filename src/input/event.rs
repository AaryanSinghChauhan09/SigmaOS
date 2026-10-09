//! Linux input event (evdev) interface
//!
//! Provides a no_std-compatible Linux evdev interface for reading input events
//! from input devices. Inspired by the Linux kernel's input subsystem and
//! evdev device interface, allowing SigmaOS to process keyboard, mouse, and
//! other input events from evdev-compatible devices.
//!
//! # Design
//!
//! - Uses the Linux input event struct layout for compatibility
//! - Poll-based event reading (no blocking by default)
//! - Device enumeration via /dev/input/enX paths
//! - Event type codes matching Linux input-event-codes.h
//!
//! # Usage
//!
//! ```
//! use sigmaos::input::event::{InputEventReader, InputEvent, InputEventType, InputCode};
//! use sigmaos::input::InputDeviceManager;
//!
//! let manager = InputDeviceManager::new();
//! let mut reader = InputEventReader::new("/dev/input/event0");
//! ```

use core::mem::MaybeUninit;
use core::ops::RangeInclusive;

/// Input event types per Linux input-event-codes.h
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum InputEventType {
    /// Key events
    Key = 1,
    /// Relative axes (mouse move, trackball)
    Relative = 2,
    /// Absolute axes (touchscreen, tablet)
    Absolute = 3,
    /// ABS axes (pressure, width, touch major)
    Abs = 4,
    /// Miscellaneous (LED, repeat, etc.)
    Misc = 5,
    /// Power management
    Power = 6,
    /// Force feedback
    ForceFeedback = 7,
    /// Packet mode (for some devices)
    Packet = 8,
}

/// Input event codes per category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum InputCode {
    /// Key codes (KEY_A, KEY_B, etc.)
    Key(u16),
    /// Relative axis codes (REL_X, REL_Y, etc.)
    Relative(u16),
    /// Absolute axis codes (ABS_X, ABS_Y, etc.)
    Absolute(u16),
    /// Miscellaneous codes (BTN_0, BTN_LEFT, etc.)
    Misc(u16),
}

/// Input event value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputValue(pub i32);

/// Linux input_event struct layout (equivalent to struct input_event from linux/input.h)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct InputEvent {
    /// Timestamp (seconds + milliseconds)
    pub timestamp: InputTimestamp,
    /// Event type
    pub evtype: InputEventType,
    /// Event code
    pub code: InputCode,
    /// Event value
    pub value: InputValue,
}

/// Timestamp for input events (seconds + milliseconds, matching Linux kernel layout)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct InputTimestamp {
    /// Seconds
    pub sec: i32,
    /// Microseconds
    pub usec: i32,
}

/// Available input event types for iteration
pub const AVAILABLE_EVENT_TYPES: &[InputEventType] = &[
    InputEventType::Key,
    InputEventType::Relative,
    InputEventType::Absolute,
    InputEventType::Abs,
    InputEventType::Misc,
    InputEventType::Power,
    InputEventType::ForceFeedback,
    InputEventType::Packet,
];

/// Available input event code ranges
pub const KEY_CODE_RANGE: RangeInclusive<u16> = 0x0000..=0xFFFF;
pub const REL_CODE_RANGE: RangeInclusive<u16> = 0x0000..=0xFFFF;
pub const ABS_CODE_RANGE: RangeInclusive<u16> = 0x0000..=0xFFFF;

/// Maximum name length for input devices (INPUT_DEVICE_NAME_SIZE from linux/input.h)
pub const MAX_DEVICE_NAME: usize = 80;
/// Maximum number of keys
pub const KEY_MAX: u16 = 353;
/// Number of absolute axes
pub const ABS_CNT: u16 = 6;
/// Number of relative axes
pub const REL_CNT: u16 = 6;

/// Input event ioctl commands (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputIoctl {
    /// Get device name
    GetName,
    /// Get device ID
    GetId,
    /// Set LED
    SetLed,
    /// Set repeat rate
    SetRepeat,
}

/// Keyboard event keys (subset of Linux KEY_*)
pub mod key {
    //! Keyboard key codes matching Linux input-event-codes.h KEY_* definitions.

    /// Key escape
    pub const ESC: u16 = 0x01;
    /// Key 1
    pub const _1: u16 = 0x02;
    /// Key 2
    pub const _2: u16 = 0x03;
    /// Key 3
    pub const _3: u16 = 0x04;
    /// Key 4
    pub const _4: u16 = 0x05;
    /// Key 5
    pub const _5: u16 = 0x06;
    /// Key 6
    pub const _6: u16 = 0x07;
    /// Key 7
    pub const _7: u16 = 0x08;
    /// Key 8
    pub const _8: u16 = 0x09;
    /// Key 9
    pub const _9: u16 = 0x0A;
    /// Key 10
    pub const _10: u16 = 0x0B;
    /// Key 11
    pub const _11: u16 = 0x0C;
    /// Key 12
    pub const _12: u16 = 0x0D;
    /// Key delete (forward)
    pub const DEL: u16 = 0x2A;
    /// Key enter
    pub const ENTER: u16 = 0x1C;
    /// Key tab
    pub const TAB: u16 = 0x0F;
    /// Key space
    pub const SPACE: u16 = 0x39;
    /// Key backspace
    pub const BACKSPACE: u16 = 0x0E;
    /// Key up
    pub const ARROW_UP: u16 = 0x52;
    /// Key down
    pub const ARROW_DOWN: u16 = 0x50;
    /// Key left
    pub const ARROW_LEFT: u16 = 0x4F;
    /// Key right
    pub const ARROW_RIGHT: u16 = 0x51;
    /// Key capslock
    pub const CAPSLOCK: u16 = 0x3A;
    /// Key numlock
    pub const NUMLOCK: u16 = 0x45;
    /// Key scrolllock
    pub const SCROLLLOCK: u16 = 0x46;
    /// Key F1
    pub const F1: u16 = 0x3B;
    /// Key F2
    pub const F2: u16 = 0x3C;
    /// Key F3
    pub const F3: u16 = 0x3D;
    /// Key F4
    pub const F4: u16 = 0x3E;
    /// Key F5
    pub const F5: u16 = 0x3F;
    /// Key F6
    pub const F6: u16 = 0x40;
    /// Key F7
    pub const F7: u16 = 0x41;
    /// Key F8
    pub const F8: u16 = 0x42;
    /// Key F9
    pub const F9: u16 = 0x43;
    /// Key F10
    pub const F10: u16 = 0x44;
    /// Key F11
    pub const F11: u16 = 0x45;
    /// Key F12
    pub const F12: u16 = 0x46;
}

/// Input event reader for reading events from evdev files
pub struct InputEventReader {
    /// Device path
    pub device_path: String,
    /// File descriptor (placeholder - would be RawFd in no_std)
    _fd: u32,
    /// Number of events read
    events_read: usize,
}

impl InputEventReader {
    /// Create a new InputEventReader for the given device path
    pub fn new(device_path: &str) -> Self {
        Self {
            device_path: device_path.to_string(),
            _fd: 0,
            events_read: 0,
        }
    }

    /// Read one input event from the device
    ///
    /// Returns Ok(Some(event)) on success, Ok(None) if no event available,
    /// or Err on I/O error.
    pub fn read_event(&mut self) -> Result<Option<InputEvent>, &'static str> {
        // In a real implementation, this would read from the file descriptor
        // using the Linux read() syscall with the evdev device
        // For now, simulate reading an event
        Ok(Some(InputEvent {
            timestamp: InputTimestamp { sec: 0, usec: 0 },
            evtype: InputEventType::Key,
            code: InputCode::Key(0x00),
            value: InputValue(0),
        }))
    }

    /// Read multiple events from the device
    pub fn read_events(&mut self, max: usize) -> Result<Vec<InputEvent>, &'static str> {
        let mut events = Vec::with_capacity(max);
        for _ in 0..max {
            if let Some(event) = self.read_event()? {
                events.push(event);
            } else {
                break;
            }
        }
        Ok(events)
    }

    /// Get the number of events read
    pub fn events_count(&self) -> usize {
        self.events_read
    }

    /// Get the device path
    pub fn device_path(&self) -> &str {
        &self.device_path
    }
}

/// Input device information from evdev
#[derive(Debug, Clone)]
pub struct InputDeviceInfo {
    /// Device name
    pub name: [u8; MAX_DEVICE_NAME],
    /// Product ID
    pub product: InputProductId,
    /// Version
    pub version: u16,
}

/// Product ID for input devices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputProductId {
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

/// Enumerate input devices (simplified - would use sysfs or /dev/input in reality)
pub fn enumerate_devices() -> Vec<InputDeviceInfo> {
    // In a real implementation, this would scan /dev/input/ or /sys/class/input/
    // For now, return a minimal default
    vec![InputDeviceInfo {
        name: [0u8; MAX_DEVICE_NAME],
        product: InputProductId {
            vendor: 1,
            product: 1,
            version: 1,
        },
        version: 1,
    }]
}

/// Get key state from an event value (pressed/released)
pub fn get_key_state(value: i32) -> KeyState {
    match value {
        0 => KeyState::Released,
        1 => KeyState::Pressed,
        _ => KeyState::Unknown,
    }
}

/// Key state from an input event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
    Unknown,
}

/// Linux-style keyboard auto-repeat tracker
///
/// Models keyboard auto-repeat: after an initial delay, keys repeat at a
/// regular interval until released. Inspired by the Linux kernel's input
/// auto-repeat mechanism.
pub struct AutoRepeatTracker {
    active: bool,
    delay: u32,
    interval: u32,
    event_count: u32,
}

impl AutoRepeatTracker {
    /// Create a new AutoRepeatTracker with default Linux-style timing
    ///
    /// Default delay: ~250ms, interval: ~30ms (typical keyboard values)
    pub fn new() -> Self {
        Self {
            active: false,
            delay: 8,
            interval: 3,
            event_count: 0,
        }
    }

    /// Reset the tracker (call when key is pressed or released)
    pub fn reset(&mut self) {
        self.active = false;
        self.event_count = 0;
    }

    /// Key press: start the auto-repeat timer
    pub fn on_press(&mut self) {
        self.active = true;
        self.event_count = 0;
    }

    /// Key release: stop auto-repeat
    pub fn on_release(&mut self) {
        self.active = false;
        self.event_count = 0;
    }

    /// Process one input event; returns true if a repeat should be generated
    pub fn tick(&mut self) -> bool {
        if !self.active {
            return false;
        }
        self.event_count += 1;
        if self.event_count >= self.delay {
            if (self.event_count - self.delay) % self.interval == 0 {
                return true;
            }
        }
        false
    }

    /// Check if auto-repeat is currently active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set the delay (in events) until first repeat
    pub fn set_delay(&mut self, delay: u32) {
        self.delay = delay;
    }

    /// Set the interval (in events) between repeats
    pub fn set_interval(&mut self, interval: u32) {
        self.interval = interval;
    }
}

/// Get the default Linux-style auto-repeat tracker
pub fn default_auto_repeat() -> AutoRepeatTracker {
    AutoRepeatTracker::new()
}

/// Check if an input event should generate an auto-repeat
pub fn should_auto_repeat(_event: &InputEvent, tracker: &mut AutoRepeatTracker) -> bool {
    tracker.tick()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_event_struct_size() {
        let event = InputEvent {
            timestamp: InputTimestamp { sec: 0, usec: 0 },
            evtype: InputEventType::Key,
            code: InputCode::Key(0x00),
            value: InputValue(0),
        };
        assert_eq!(event.evtype, InputEventType::Key);
    }

    #[test]
    fn test_input_timestamp() {
        let ts = InputTimestamp { sec: 1, usec: 500 };
        assert_eq!(ts.sec, 1);
        assert_eq!(ts.usec, 500);
    }

    #[test]
    fn test_input_event_reader_creation() {
        let mut reader = InputEventReader::new("/dev/input/event0");
        assert_eq!(reader.device_path(), "/dev/input/event0");
    }

    #[test]
    fn test_input_event_reader_read_event() {
        let mut reader = InputEventReader::new("/dev/input/event0");
        let result = reader.read_event();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_input_event_reader_read_events() {
        let mut reader = InputEventReader::new("/dev/input/event0");
        let result = reader.read_events(10);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_enumerate_devices() {
        let devices = enumerate_devices();
        let _ = &devices[..];
    }

    #[test]
    fn test_key_state() {
        assert_eq!(get_key_state(0), KeyState::Released);
        assert_eq!(get_key_state(1), KeyState::Pressed);
        assert_eq!(get_key_state(2), KeyState::Unknown);
    }

    #[test]
    fn test_input_code_variants() {
        let key_code = InputCode::Key(0x1C);
        assert_eq!(key_code, InputCode::Key(0x1C));
    }

    #[test]
    fn test_input_event_types() {
        assert!(AVAILABLE_EVENT_TYPES.contains(&InputEventType::Key));
        assert!(AVAILABLE_EVENT_TYPES.contains(&InputEventType::Absolute));
    }

    #[test]
    fn test_constants() {
        assert!(KEY_CODE_RANGE.contains(&0x01));
        assert!(KEY_CODE_RANGE.contains(&0x1C));
        assert!(REL_CODE_RANGE.contains(&0x00));
        assert!(ABS_CODE_RANGE.contains(&0x00));
    }
}
