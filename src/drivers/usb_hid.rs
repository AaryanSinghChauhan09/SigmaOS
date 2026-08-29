extern crate alloc;
use alloc::vec;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS USB HID Keyboard Driver
// Hardware abstraction for USB HID devices + PeripheralDevice OOP integration

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::CapabilityToken;

/// Keyboard Layouts inspired by multi-distro Linux/BSD keyboard subsystem
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLayout {
    UsQwerty,
    UkQwerty,
    DeQwertz,
    FrAzerty,
}

/// USB HID report type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidReportType {
    Input,
    Output,
    Feature,
}

/// USB HID keyboard event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidKeyboardEvent {
    pub keycode: u8,
    pub pressed: bool,
    pub modifiers: u8,
}

/// USB HID driver interface
pub struct UsbHidDriver {
    pub vendor_id: u16,
    pub product_id: u16,
    pub capabilities: CapabilityToken,
    pub event_buffer: Vec<HidKeyboardEvent>,
    pub connected: bool,

    // Typematic auto-repeat control inspired by Linux Input subsystem
    pub repeat_delay_ms: u32,
    pub repeat_rate_ms: u32,
    pub last_pressed_keycode: Option<u8>,
    pub last_press_time_ms: u32,
    pub last_repeat_time_ms: u32,

    // Lock key state indicators
    pub caps_lock_active: bool,
    pub num_lock_active: bool,
    pub scroll_lock_active: bool,

    // Rollover tracking (N-Key Rollover / NKRO)
    pub active_held_keys: Vec<u8>,

    // Keyboard layout selector
    pub layout: KeyboardLayout,
}

impl UsbHidDriver {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        Self {
            vendor_id,
            product_id,
            capabilities: CapabilityToken::new(),
            event_buffer: Vec::new(),
            connected: false,
            repeat_delay_ms: 250, // standard Linux default (250ms delay)
            repeat_rate_ms: 33,   // standard Linux default (30Hz repeat rate / ~33ms interval)
            last_pressed_keycode: None,
            last_press_time_ms: 0,
            last_repeat_time_ms: 0,
            caps_lock_active: false,
            num_lock_active: false,
            scroll_lock_active: false,
            active_held_keys: Vec::new(),
            layout: KeyboardLayout::UsQwerty,
        }
    }

    pub fn set_layout(&mut self, layout: KeyboardLayout) {
        self.layout = layout;
    }

    pub fn connect(&mut self) -> Result<(), HidError> {
        // Simulate USB connection
        self.connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.active_held_keys.clear();
        self.last_pressed_keycode = None;
    }

    pub fn poll_event(&mut self) -> Option<HidKeyboardEvent> {
        if !self.connected {
            return None;
        }
        self.event_buffer.pop()
    }

    pub fn push_event(&mut self, event: HidKeyboardEvent) {
        // Track N-Key Rollover state
        if event.pressed {
            if !self.active_held_keys.contains(&event.keycode) {
                self.active_held_keys.push(event.keycode);
            }
            // Update typematic repeat state
            self.last_pressed_keycode = Some(event.keycode);
            self.last_press_time_ms = 0; // Simulated relative start
            self.last_repeat_time_ms = 0;
        } else {
            self.active_held_keys.retain(|&k| k != event.keycode);
            if self.last_pressed_keycode == Some(event.keycode) {
                self.last_pressed_keycode = None;
            }
        }

        // Toggle Lock States when keycode matches standard keyboard locks
        // Keycode 0x39 = Caps Lock, 0x53 = Num Lock, 0x47 = Scroll Lock
        if event.pressed {
            if event.keycode == 0x39 {
                self.toggle_caps_lock();
            } else if event.keycode == 0x53 {
                self.toggle_num_lock();
            } else if event.keycode == 0x47 {
                self.toggle_scroll_lock();
            }
        }

        self.event_buffer.push(event);
    }

    /// Simulates passing CPU timer ticks to evaluate and trigger typematic repeats
    pub fn tick_repeat(&mut self, current_time_ms: u32) -> Option<HidKeyboardEvent> {
        if !self.connected {
            return None;
        }
        let keycode = self.last_pressed_keycode?;
        if self.last_press_time_ms == 0 {
            self.last_press_time_ms = current_time_ms;
            self.last_repeat_time_ms = current_time_ms;
            return None;
        }

        let elapsed = current_time_ms - self.last_press_time_ms;
        if elapsed >= self.repeat_delay_ms {
            let repeat_elapsed = current_time_ms - self.last_repeat_time_ms;
            if repeat_elapsed >= self.repeat_rate_ms {
                self.last_repeat_time_ms = current_time_ms;
                // Dispatch repeat input event
                return Some(HidKeyboardEvent {
                    keycode,
                    pressed: true,
                    modifiers: 0,
                });
            }
        }
        None
    }

    pub fn send_report(
        &mut self,
        report_type: HidReportType,
        _data: &[u8],
    ) -> Result<(), HidError> {
        if !self.connected {
            return Err(HidError::NotConnected);
        }

        match report_type {
            HidReportType::Output => {
                // Send output report (e.g., LED state)
                Ok(())
            }
            HidReportType::Feature => {
                // Send feature report
                Ok(())
            }
            HidReportType::Input => Err(HidError::InvalidReportType),
        }
    }

    pub fn set_leds(&mut self, leds: u8) -> Result<(), HidError> {
        self.send_report(HidReportType::Output, &[leds])
    }

    /// Toggles Caps Lock state and formats LED report (Bit 1 = Caps Lock LED)
    pub fn toggle_caps_lock(&mut self) {
        self.caps_lock_active = !self.caps_lock_active;
        self.update_led_report().ok();
    }

    /// Toggles Num Lock state and formats LED report (Bit 0 = Num Lock LED)
    pub fn toggle_num_lock(&mut self) {
        self.num_lock_active = !self.num_lock_active;
        self.update_led_report().ok();
    }

    /// Toggles Scroll Lock state and formats LED report (Bit 2 = Scroll Lock LED)
    pub fn toggle_scroll_lock(&mut self) {
        self.scroll_lock_active = !self.scroll_lock_active;
        self.update_led_report().ok();
    }

    fn update_led_report(&mut self) -> Result<(), HidError> {
        let mut leds = 0u8;
        if self.num_lock_active {
            leds |= 0x01;
        }
        if self.caps_lock_active {
            leds |= 0x02;
        }
        if self.scroll_lock_active {
            leds |= 0x04;
        }
        self.set_leds(leds)
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

impl Default for UsbHidDriver {
    fn default() -> Self {
        Self::new(0x0000, 0x0000)
    }
}

/// HID USB Scancode to ASCII mapping (US QWERTY, first 58 scancodes)
const HID_SCANCODE_TO_ASCII: [u8; 57] = [
    0, 0, 0, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'1', b'2', b'3', b'4',
    b'5', b'6', b'7', b'8', b'9', b'0', b'\n', 0, b'\x08', b'\t', b' ', b'-', b'=', b'[', b']',
    b'\\', 0, b';', b'\'', b'`', b',', b'.', b'/',
];

/// Standalone HID Keyboard implementing PeripheralDevice for PeripheralManager
pub struct HidKeyboard {
    inner: UsbHidDriver,
    power_state: PowerState,
}

impl HidKeyboard {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        Self {
            inner: UsbHidDriver::new(vendor_id, product_id),
            power_state: PowerState::Off,
        }
    }

    /// Converts a USB HID scancode to ASCII character
    pub fn scancode_to_ascii(scancode: u8, shift: bool) -> Option<u8> {
        let idx = scancode as usize;
        if idx >= HID_SCANCODE_TO_ASCII.len() {
            return None;
        }
        let ch = HID_SCANCODE_TO_ASCII[idx];
        if ch == 0 {
            return None;
        }
        if shift && ch.is_ascii_alphabetic() {
            Some(ch.to_ascii_uppercase())
        } else {
            Some(ch)
        }
    }

    /// Converts a USB HID scancode to ASCII character based on KeyboardLayout
    pub fn scancode_to_ascii_layout(scancode: u8, shift: bool, layout: KeyboardLayout) -> Option<u8> {
        let ascii = Self::scancode_to_ascii(scancode, shift)?;
        match layout {
            KeyboardLayout::UsQwerty | KeyboardLayout::UkQwerty => Some(ascii),
            KeyboardLayout::DeQwertz => match ascii {
                b'y' => Some(b'z'),
                b'Y' => Some(b'Z'),
                b'z' => Some(b'y'),
                b'Z' => Some(b'Y'),
                _ => Some(ascii),
            },
            KeyboardLayout::FrAzerty => match ascii {
                b'q' => Some(b'a'),
                b'Q' => Some(b'A'),
                b'a' => Some(b'q'),
                b'A' => Some(b'Q'),
                b'w' => Some(b'z'),
                b'W' => Some(b'Z'),
                b'z' => Some(b'w'),
                b'Z' => Some(b'W'),
                _ => Some(ascii),
            },
        }
    }

    /// Decode a keyboard event to a printable ASCII char
    pub fn decode_event(event: &HidKeyboardEvent) -> Option<char> {
        let shift = (event.modifiers & 0x22) != 0;
        Self::scancode_to_ascii(event.keycode, shift).map(|b| b as char)
    }

    /// Decode a keyboard event to a printable ASCII char with layout awareness
    pub fn decode_event_layout(event: &HidKeyboardEvent, layout: KeyboardLayout) -> Option<char> {
        let shift = (event.modifiers & 0x22) != 0;
        Self::scancode_to_ascii_layout(event.keycode, shift, layout).map(|b| b as char)
    }
}

impl PeripheralDevice for HidKeyboard {
    fn name(&self) -> &'static str {
        "USB HID Keyboard"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.inner
            .connect()
            .map_err(|_| "USB HID: Failed to connect")?;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("HID Keyboard is not powered on");
        }
        let mut count = 0;
        while count < buffer.len() {
            match self.inner.poll_event() {
                Some(event) if event.pressed => {
                    if let Some(decoded) = Self::decode_event(&event) {
                        buffer[count] = decoded as u8;
                        count += 1;
                    }
                }
                _ => break,
            }
        }
        Ok(count)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if let Some(&led_byte) = data.first() {
            self.inner
                .set_leds(led_byte)
                .map_err(|_| "HID: LED set failed")?;
        }
        Ok(data.len().min(1))
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.inner.disconnect();
        self.power_state = PowerState::Off;
        Ok(())
    }
}

/// HID errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HidError {
    NotConnected,
    InvalidReportType,
    PermissionDenied,
    DeviceError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hid_creation() {
        let hid = UsbHidDriver::new(0x1234, 0x5678);
        assert_eq!(hid.vendor_id, 0x1234);
        assert_eq!(hid.product_id, 0x5678);
        assert!(!hid.connected);
        assert_eq!(hid.repeat_delay_ms, 250);
    }

    #[test]
    fn test_connect() {
        let mut hid = UsbHidDriver::new(0x1234, 0x5678);
        assert!(hid.connect().is_ok());
        assert!(hid.connected);
    }

    #[test]
    fn test_disconnect() {
        let mut hid = UsbHidDriver::new(0x1234, 0x5678);
        hid.connect().unwrap();
        hid.disconnect();
        assert!(!hid.connected);
    }

    #[test]
    fn test_nkro_and_locks() {
        let mut hid = UsbHidDriver::new(0x1234, 0x5678);
        hid.connect().unwrap();

        // Caps Lock Toggle keycode (0x39)
        let caps_event = HidKeyboardEvent {
            keycode: 0x39,
            pressed: true,
            modifiers: 0,
        };
        hid.push_event(caps_event.clone());
        assert!(hid.caps_lock_active);
        assert_eq!(hid.active_held_keys, vec![0x39]);

        // Key Release
        let caps_release = HidKeyboardEvent {
            keycode: 0x39,
            pressed: false,
            modifiers: 0,
        };
        hid.push_event(caps_release);
        assert!(hid.caps_lock_active); // remains true (toggle)
        assert!(hid.active_held_keys.is_empty());
    }

    #[test]
    fn test_multi_layout_decoding() {
        let event_z = HidKeyboardEvent {
            keycode: 0x1D, // 'z' in US QWERTY
            pressed: true,
            modifiers: 0,
        };

        let us_char = HidKeyboard::decode_event_layout(&event_z, KeyboardLayout::UsQwerty).unwrap();
        assert_eq!(us_char, 'z');

        let de_char = HidKeyboard::decode_event_layout(&event_z, KeyboardLayout::DeQwertz).unwrap();
        assert_eq!(de_char, 'y'); // 'z' becomes 'y' in QWERTZ

        let event_q = HidKeyboardEvent {
            keycode: 0x14, // 'q' in US QWERTY
            pressed: true,
            modifiers: 0,
        };
        let fr_char = HidKeyboard::decode_event_layout(&event_q, KeyboardLayout::FrAzerty).unwrap();
        assert_eq!(fr_char, 'a'); // 'q' becomes 'a' in AZERTY
    }

    #[test]
    fn test_typematic_auto_repeat() {
        let mut hid = UsbHidDriver::new(0x1234, 0x5678);
        hid.connect().unwrap();

        let key_event = HidKeyboardEvent {
            keycode: 0x04, // 'a'
            pressed: true,
            modifiers: 0,
        };
        hid.push_event(key_event);

        // First tick maps initial timings
        let t1 = hid.tick_repeat(10);
        assert!(t1.is_none());

        // Wait within delay threshold
        let t2 = hid.tick_repeat(100);
        assert!(t2.is_none());

        // Cross delay threshold (delay = 250ms), tick_repeat triggers repeats
        let t3 = hid.tick_repeat(300); // 300 - 10 = 290 > 250
        assert!(t3.is_some());
        assert_eq!(t3.unwrap().keycode, 0x04);
    }
}
