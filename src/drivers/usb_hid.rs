// SigmaOS USB HID Keyboard Driver
// Hardware abstraction for USB HID devices + PeripheralDevice OOP integration

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::CapabilityToken;

/// USB HID report type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidReportType {
    Input,
    Output,
    Feature,
}

/// USB HID keyboard event
#[derive(Debug, Clone)]
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
}

impl UsbHidDriver {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        Self {
            vendor_id,
            product_id,
            capabilities: CapabilityToken::new(),
            event_buffer: Vec::new(),
            connected: false,
        }
    }

    pub fn connect(&mut self) -> Result<(), HidError> {
        // Simulate USB connection
        self.connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn poll_event(&mut self) -> Option<HidKeyboardEvent> {
        if !self.connected {
            return None;
        }
        self.event_buffer.pop()
    }

    pub fn push_event(&mut self, event: HidKeyboardEvent) {
        self.event_buffer.push(event);
    }

    pub fn send_report(&mut self, report_type: HidReportType, data: &[u8]) -> Result<(), HidError> {
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

    /// Decode a keyboard event to a printable ASCII char
    pub fn decode_event(event: &HidKeyboardEvent) -> Option<char> {
        let shift = (event.modifiers & 0x22) != 0;
        Self::scancode_to_ascii(event.keycode, shift).map(|b| b as char)
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
    fn test_event_buffer() {
        let mut hid = UsbHidDriver::new(0x1234, 0x5678);
        hid.connect().unwrap();
        let event = HidKeyboardEvent {
            keycode: 0x04,
            pressed: true,
            modifiers: 0,
        };
        hid.push_event(event.clone());
        let polled = hid.poll_event();
        assert!(polled.is_some());
    }

    #[test]
    fn test_not_connected_error() {
        let mut hid = UsbHidDriver::new(0x1234, 0x5678);
        let result = hid.send_report(HidReportType::Output, &[0x01]);
        assert!(result.is_err());
    }
}
