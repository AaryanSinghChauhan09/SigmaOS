//! USB HID (Human Interface Device) Class Driver
//! Supports keyboards, mice, game controllers, and other HID devices
//! Reference: USB HID specification 1.11 and Linux drivers/hid/

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

// Re-export USB HID stubs
pub use crate::stubs::usb_hid_stubs::*;

/// HID Class descriptor types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidDescriptorType {
    Hid = 0x21,
    Report = 0x22,
    Physical = 0x23,
}

/// HID class-specific requests
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum HidRequest {
    GetReport = 0x01,
    GetIdle = 0x02,
    GetProtocol = 0x03,
    SetReport = 0x09,
    SetIdle = 0x0A,
    SetProtocol = 0x0B,
}

/// HID report types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidReportType {
    Input = 1,
    Output = 2,
    Feature = 3,
}

/// HID descriptor structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct HidDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_hid: u16, // HID version (BCD)
    pub country_code: u8,
    pub num_descriptors: u8,
    pub report_desc_type: u8,
    pub report_desc_length: u16,
}

/// HID Usage Page IDs (from HID Usage Tables)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidUsagePage {
    GenericDesktop = 0x01,
    SimulationControls = 0x02,
    VrControls = 0x03,
    SportControls = 0x04,
    GameControls = 0x05,
    GenericDevice = 0x06,
    Keyboard = 0x07,
    Led = 0x08,
    Button = 0x09,
    Ordinal = 0x0A,
    Telephony = 0x0B,
    Consumer = 0x0C,
    Digitizer = 0x0D,
}

/// HID Generic Desktop usages
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidGenericDesktopUsage {
    Pointer = 0x01,
    Mouse = 0x02,
    Joystick = 0x04,
    Gamepad = 0x05,
    Keyboard = 0x06,
    Keypad = 0x07,
    MultiAxisController = 0x08,
    X = 0x30,
    Y = 0x31,
    Z = 0x32,
    Rx = 0x33,
    Ry = 0x34,
    Rz = 0x35,
    Slider = 0x36,
    Dial = 0x37,
    Wheel = 0x38,
}

/// Keyboard modifier key flags
#[derive(Debug, Clone, Copy)]
pub struct KeyboardModifiers {
    pub left_ctrl: bool,
    pub left_shift: bool,
    pub left_alt: bool,
    pub left_gui: bool,
    pub right_ctrl: bool,
    pub right_shift: bool,
    pub right_alt: bool,
    pub right_gui: bool,
}

impl KeyboardModifiers {
    pub fn from_byte(byte: u8) -> Self {
        Self {
            left_ctrl: (byte & 0x01) != 0,
            left_shift: (byte & 0x02) != 0,
            left_alt: (byte & 0x04) != 0,
            left_gui: (byte & 0x08) != 0,
            right_ctrl: (byte & 0x10) != 0,
            right_shift: (byte & 0x20) != 0,
            right_alt: (byte & 0x40) != 0,
            right_gui: (byte & 0x80) != 0,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;
        if self.left_ctrl {
            byte |= 0x01;
        }
        if self.left_shift {
            byte |= 0x02;
        }
        if self.left_alt {
            byte |= 0x04;
        }
        if self.left_gui {
            byte |= 0x08;
        }
        if self.right_ctrl {
            byte |= 0x10;
        }
        if self.right_shift {
            byte |= 0x20;
        }
        if self.right_alt {
            byte |= 0x40;
        }
        if self.right_gui {
            byte |= 0x80;
        }
        byte
    }
}

/// Standard keyboard input report (boot protocol)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct KeyboardReport {
    pub modifiers: u8,     // Modifier keys bitfield
    pub reserved: u8,      // Reserved (always 0)
    pub keycodes: [u8; 6], // Up to 6 simultaneous keys
}

impl KeyboardReport {
    pub fn new() -> Self {
        Self {
            modifiers: 0,
            reserved: 0,
            keycodes: [0; 6],
        }
    }

    pub fn get_modifiers(&self) -> KeyboardModifiers {
        KeyboardModifiers::from_byte(self.modifiers)
    }
}

/// Standard mouse input report (boot protocol)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MouseReport {
    pub buttons: u8,    // Button states (bits 0-2)
    pub x_movement: i8, // Relative X movement
    pub y_movement: i8, // Relative Y movement
    pub wheel: i8,      // Wheel movement (optional)
}

impl MouseReport {
    pub fn new() -> Self {
        Self {
            buttons: 0,
            x_movement: 0,
            y_movement: 0,
            wheel: 0,
        }
    }

    pub fn left_button(&self) -> bool {
        (self.buttons & 0x01) != 0
    }

    pub fn right_button(&self) -> bool {
        (self.buttons & 0x02) != 0
    }

    pub fn middle_button(&self) -> bool {
        (self.buttons & 0x04) != 0
    }
}

/// HID device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidDeviceType {
    Keyboard,
    Mouse,
    Gamepad,
    Joystick,
    Tablet,
    Touchscreen,
    Generic,
}

/// HID device structure
pub struct HidDevice {
    pub device_type: HidDeviceType,
    pub vendor_id: u16,
    pub product_id: u16,
    pub report_descriptor: Vec<u8>,
    pub input_reports: Vec<Vec<u8>>,
    pub protocol: u8,  // 0 = boot protocol, 1 = report protocol
    pub idle_rate: u8, // Idle rate in 4ms units
}

impl HidDevice {
    pub fn new(device_type: HidDeviceType, vendor_id: u16, product_id: u16) -> Self {
        Self {
            device_type,
            vendor_id,
            product_id,
            report_descriptor: Vec::new(),
            input_reports: Vec::new(),
            protocol: 1,
            idle_rate: 0,
        }
    }

    /// Parse HID report descriptor
    pub fn parse_report_descriptor(&mut self, descriptor: Vec<u8>) -> Result<(), HidError> {
        self.report_descriptor = descriptor;
        // In real implementation: parse descriptor to build report structure
        Ok(())
    }

    /// Set HID protocol (boot or report)
    pub fn set_protocol(&mut self, protocol: u8) -> Result<(), HidError> {
        if protocol > 1 {
            return Err(HidError::InvalidProtocol);
        }
        self.protocol = protocol;
        Ok(())
    }

    /// Set idle rate (0 = infinite, non-zero = 4ms * value)
    pub fn set_idle(&mut self, idle_rate: u8) -> Result<(), HidError> {
        self.idle_rate = idle_rate;
        Ok(())
    }

    /// Process input report
    pub fn process_input_report(&mut self, report: Vec<u8>) -> Result<HidEvent, HidError> {
        self.input_reports.push(report.clone());

        match self.device_type {
            HidDeviceType::Keyboard => {
                if report.len() >= 8 {
                    Ok(HidEvent::Keyboard(KeyboardEvent {
                        modifiers: KeyboardModifiers::from_byte(report[0]),
                        keycodes: [
                            report[2], report[3], report[4], report[5], report[6], report[7],
                        ],
                    }))
                } else {
                    Err(HidError::InvalidReportLength)
                }
            }
            HidDeviceType::Mouse => {
                if report.len() >= 3 {
                    Ok(HidEvent::Mouse(MouseEvent {
                        buttons: report[0],
                        x_movement: report[1] as i8,
                        y_movement: report[2] as i8,
                        wheel: if report.len() > 3 { report[3] as i8 } else { 0 },
                    }))
                } else {
                    Err(HidError::InvalidReportLength)
                }
            }
            _ => Ok(HidEvent::Generic(report)),
        }
    }
}

/// HID event types
#[derive(Debug, Clone)]
pub enum HidEvent {
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    Gamepad(GamepadEvent),
    Generic(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub modifiers: KeyboardModifiers,
    pub keycodes: [u8; 6],
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub buttons: u8,
    pub x_movement: i8,
    pub y_movement: i8,
    pub wheel: i8,
}

#[derive(Debug, Clone)]
pub struct GamepadEvent {
    pub buttons: u16,
    pub left_stick_x: i16,
    pub left_stick_y: i16,
    pub right_stick_x: i16,
    pub right_stick_y: i16,
    pub left_trigger: u8,
    pub right_trigger: u8,
}

/// HID driver manager
pub struct HidDriver {
    devices: BTreeMap<u32, HidDevice>,
    next_device_id: u32,
}

impl HidDriver {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            next_device_id: 1,
        }
    }

    /// Register new HID device
    pub fn register_device(&mut self, device: HidDevice) -> u32 {
        let device_id = self.next_device_id;
        self.next_device_id += 1;
        self.devices.insert(device_id, device);
        device_id
    }

    /// Unregister HID device
    pub fn unregister_device(&mut self, device_id: u32) -> Result<(), HidError> {
        self.devices
            .remove(&device_id)
            .ok_or(HidError::DeviceNotFound)?;
        Ok(())
    }

    /// Get device by ID
    pub fn get_device(&self, device_id: u32) -> Option<&HidDevice> {
        self.devices.get(&device_id)
    }

    /// Get mutable device by ID
    pub fn get_device_mut(&mut self, device_id: u32) -> Option<&mut HidDevice> {
        self.devices.get_mut(&device_id)
    }

    /// Process input from device
    pub fn process_input(&mut self, device_id: u32, report: Vec<u8>) -> Result<HidEvent, HidError> {
        let device = self
            .devices
            .get_mut(&device_id)
            .ok_or(HidError::DeviceNotFound)?;
        device.process_input_report(report)
    }
}

/// HID error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidError {
    DeviceNotFound,
    InvalidProtocol,
    InvalidReportLength,
    InvalidDescriptor,
    TransferFailed,
    ParseError,
}

/// USB HID scan codes (subset of common keys)
pub mod scancodes {
    pub const KEY_A: u8 = 0x04;
    pub const KEY_B: u8 = 0x05;
    pub const KEY_C: u8 = 0x06;
    pub const KEY_D: u8 = 0x07;
    pub const KEY_E: u8 = 0x08;
    pub const KEY_F: u8 = 0x09;
    pub const KEY_G: u8 = 0x0A;
    pub const KEY_H: u8 = 0x0B;
    pub const KEY_I: u8 = 0x0C;
    pub const KEY_J: u8 = 0x0D;
    pub const KEY_K: u8 = 0x0E;
    pub const KEY_L: u8 = 0x0F;
    pub const KEY_M: u8 = 0x10;
    pub const KEY_N: u8 = 0x11;
    pub const KEY_O: u8 = 0x12;
    pub const KEY_P: u8 = 0x13;
    pub const KEY_Q: u8 = 0x14;
    pub const KEY_R: u8 = 0x15;
    pub const KEY_S: u8 = 0x16;
    pub const KEY_T: u8 = 0x17;
    pub const KEY_U: u8 = 0x18;
    pub const KEY_V: u8 = 0x19;
    pub const KEY_W: u8 = 0x1A;
    pub const KEY_X: u8 = 0x1B;
    pub const KEY_Y: u8 = 0x1C;
    pub const KEY_Z: u8 = 0x1D;
    pub const KEY_1: u8 = 0x1E;
    pub const KEY_2: u8 = 0x1F;
    pub const KEY_3: u8 = 0x20;
    pub const KEY_4: u8 = 0x21;
    pub const KEY_5: u8 = 0x22;
    pub const KEY_6: u8 = 0x23;
    pub const KEY_7: u8 = 0x24;
    pub const KEY_8: u8 = 0x25;
    pub const KEY_9: u8 = 0x26;
    pub const KEY_0: u8 = 0x27;
    pub const KEY_ENTER: u8 = 0x28;
    pub const KEY_ESC: u8 = 0x29;
    pub const KEY_BACKSPACE: u8 = 0x2A;
    pub const KEY_TAB: u8 = 0x2B;
    pub const KEY_SPACE: u8 = 0x2C;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_modifiers() {
        let mods = KeyboardModifiers::from_byte(0x03); // Ctrl + Shift
        assert!(mods.left_ctrl);
        assert!(mods.left_shift);
        assert!(!mods.left_alt);
    }

    #[test]
    fn test_mouse_report() {
        let report = MouseReport {
            buttons: 0x01,
            x_movement: 10,
            y_movement: -5,
            wheel: 0,
        };
        assert!(report.left_button());
        assert!(!report.right_button());
    }

    #[test]
    fn test_hid_driver() {
        let mut driver = HidDriver::new();
        let device = HidDevice::new(HidDeviceType::Keyboard, 0x1234, 0x5678);
        let device_id = driver.register_device(device);
        assert!(driver.get_device(device_id).is_some());
    }
}
