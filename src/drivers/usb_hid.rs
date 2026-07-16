// SigmaOS USB HID Keyboard Driver
// Hardware abstraction for USB HID devices

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
