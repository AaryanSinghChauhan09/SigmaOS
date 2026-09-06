#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SPDX-License-Identifier: MIT
// SigmaOS HID Input Device Driver
// Supports USB HID devices: keyboards, mice, touchpads

use std::boxed::Box;
use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::driver::pci_enumeration::{PciDeviceInfo, PciDriver};

// ============================================================================
// HID Constants
// ============================================================================

pub const USB_CLASS_HID: u8 = 0x03;
pub const HID_SUBCLASS_BOOT: u8 = 0x01;
pub const HID_PROTOCOL_KEYBOARD: u8 = 0x01;
pub const HID_PROTOCOL_MOUSE: u8 = 0x02;

// HID Report Types
pub const HID_REPORT_TYPE_INPUT: u8 = 0x01;
pub const HID_REPORT_TYPE_OUTPUT: u8 = 0x02;
pub const HID_REPORT_TYPE_FEATURE: u8 = 0x03;

// HID Keyboard Codes (US Layout)
pub const HID_KEY_NONE: u8 = 0x00;
pub const HID_KEY_ERROR_ROLLOVER: u8 = 0x01;
pub const HID_KEY_POST_FAIL: u8 = 0x02;
pub const HID_KEY_ERROR_UNDEFINED: u8 = 0x03;
pub const HID_KEY_A: u8 = 0x04;
pub const HID_KEY_B: u8 = 0x05;
pub const HID_KEY_ENTER: u8 = 0x28;
pub const HID_KEY_ESCAPE: u8 = 0x29;
pub const HID_KEY_BACKSPACE: u8 = 0x2A;
pub const HID_KEY_TAB: u8 = 0x2B;
pub const HID_KEY_SPACEBAR: u8 = 0x2C;

// HID Modifiers
pub const HID_MOD_LCTRL: u8 = 0x01;
pub const HID_MOD_LSHIFT: u8 = 0x02;
pub const HID_MOD_LALT: u8 = 0x04;
pub const HID_MOD_LMETA: u8 = 0x08;
pub const HID_MOD_RCTRL: u8 = 0x10;
pub const HID_MOD_RSHIFT: u8 = 0x20;
pub const HID_MOD_RALT: u8 = 0x40;
pub const HID_MOD_RMETA: u8 = 0x80;

// ============================================================================
// HID Device Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidDeviceType {
    Keyboard,
    Mouse,
    Touchpad,
    Joystick,
    GameController,
    MediaControl,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct HidKeyboardReport {
    pub modifier: u8,
    pub reserved: u8,
    pub keycodes: [u8; 6],
}

impl HidKeyboardReport {
    pub fn new() -> Self {
        HidKeyboardReport {
            modifier: 0,
            reserved: 0,
            keycodes: [0; 6],
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keycodes.iter().any(|&k| k == key)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HidMouseReport {
    pub buttons: u8,      // Buttons bitmap: bit 0=left, bit 1=right, bit 2=middle
    pub x: i8,            // X movement (-127 to +127)
    pub y: i8,            // Y movement (-127 to +127)
    pub wheel: i8,        // Wheel (-127 to +127)
}

impl HidMouseReport {
    pub fn new() -> Self {
        HidMouseReport {
            buttons: 0,
            x: 0,
            y: 0,
            wheel: 0,
        }
    }

    pub fn left_button_pressed(&self) -> bool {
        (self.buttons & 0x01) != 0
    }

    pub fn right_button_pressed(&self) -> bool {
        (self.buttons & 0x02) != 0
    }

    pub fn middle_button_pressed(&self) -> bool {
        (self.buttons & 0x04) != 0
    }
}

// ============================================================================
// HID Device
// ============================================================================

#[derive(Debug, Clone)]
pub struct HidDevice {
    pub device_id: u16,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_type: HidDeviceType,
    pub is_connected: bool,
    pub interrupt_endpoint: u8,
    pub max_packet_size: u16,
    pub poll_interval: u8,
    pub manufacturer: String,
    pub product_name: String,
    pub serial_number: String,
}

impl HidDevice {
    pub fn new(vendor_id: u16, product_id: u16, device_type: HidDeviceType) -> Self {
        HidDevice {
            device_id: 0,
            vendor_id,
            product_id,
            device_type,
            is_connected: false,
            interrupt_endpoint: 0x81,
            max_packet_size: 8,
            poll_interval: 10,
            manufacturer: String::new(),
            product_name: String::new(),
            serial_number: String::new(),
        }
    }
}

// ============================================================================
// HID Report Buffer
// ============================================================================

pub struct HidReportBuffer {
    data: Vec<u8>,
    report_id: u8,
    report_type: u8,
    report_length: usize,
}

impl HidReportBuffer {
    pub fn new(capacity: usize) -> Self {
        HidReportBuffer {
            data: Vec::with_capacity(capacity),
            report_id: 0,
            report_type: 0,
            report_length: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.data.len() >= self.data.capacity() {
            return Err("Report buffer full");
        }
        self.data.push(byte);
        Ok(())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

// ============================================================================
// HID Input Device Driver
// ============================================================================

pub struct HidInputDeviceDriver {
    devices: Vec<HidDevice>,
    keyboard_buffer: Vec<HidKeyboardReport>,
    mouse_buffer: Vec<HidMouseReport>,
    device_count: AtomicU32,
    report_count: AtomicU32,
}

impl HidInputDeviceDriver {
    pub fn new() -> Self {
        HidInputDeviceDriver {
            devices: Vec::new(),
            keyboard_buffer: Vec::with_capacity(16),
            mouse_buffer: Vec::with_capacity(16),
            device_count: AtomicU32::new(0),
            report_count: AtomicU32::new(0),
        }
    }

    pub fn register_device(&mut self, device: HidDevice) -> Result<u16, &'static str> {
        if self.devices.len() >= 16 {
            return Err("Too many devices registered");
        }

        let device_id = self.devices.len() as u16;
        self.devices.push(device);
        self.device_count.fetch_add(1, Ordering::SeqCst);

        Ok(device_id)
    }

    pub fn probe_keyboard(&mut self, vendor_id: u16, product_id: u16) -> Result<u16, &'static str> {
        let mut device = HidDevice::new(vendor_id, product_id, HidDeviceType::Keyboard);
        device.manufacturer = "Generic".to_string();
        device.product_name = "Keyboard".to_string();
        device.is_connected = true;
        device.max_packet_size = 8;

        self.register_device(device)
    }

    pub fn probe_mouse(&mut self, vendor_id: u16, product_id: u16) -> Result<u16, &'static str> {
        let mut device = HidDevice::new(vendor_id, product_id, HidDeviceType::Mouse);
        device.manufacturer = "Generic".to_string();
        device.product_name = "Mouse".to_string();
        device.is_connected = true;
        device.max_packet_size = 4;

        self.register_device(device)
    }

    pub fn submit_keyboard_report(&mut self, report: HidKeyboardReport) -> Result<(), &'static str> {
        if self.keyboard_buffer.len() >= 16 {
            return Err("Keyboard buffer full");
        }
        self.keyboard_buffer.push(report);
        self.report_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn submit_mouse_report(&mut self, report: HidMouseReport) -> Result<(), &'static str> {
        if self.mouse_buffer.len() >= 16 {
            return Err("Mouse buffer full");
        }
        self.mouse_buffer.push(report);
        self.report_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn get_keyboard_report(&mut self) -> Option<HidKeyboardReport> {
        if self.keyboard_buffer.is_empty() {
            None
        } else {
            Some(self.keyboard_buffer.remove(0))
        }
    }

    pub fn get_mouse_report(&mut self) -> Option<HidMouseReport> {
        if self.mouse_buffer.is_empty() {
            None
        } else {
            Some(self.mouse_buffer.remove(0))
        }
    }

    pub fn get_device(&self, device_id: u16) -> Option<&HidDevice> {
        self.devices.get(device_id as usize)
    }

    pub fn get_devices(&self) -> &[HidDevice] {
        &self.devices
    }

    pub fn get_device_count(&self) -> u32 {
        self.device_count.load(Ordering::SeqCst)
    }

    pub fn get_report_count(&self) -> u32 {
        self.report_count.load(Ordering::SeqCst)
    }

    pub fn disconnect_device(&mut self, device_id: u16) -> Result<(), &'static str> {
        if (device_id as usize) >= self.devices.len() {
            return Err("Invalid device ID");
        }

        self.devices[device_id as usize].is_connected = false;
        self.device_count.fetch_sub(1, Ordering::SeqCst);

        Ok(())
    }

    pub fn set_led(&self, device_id: u16, _led_mask: u8) -> Result<(), &'static str> {
        if (device_id as usize) >= self.devices.len() {
            return Err("Invalid device ID");
        }

        let device = &self.devices[device_id as usize];
        if device.device_type != HidDeviceType::Keyboard {
            return Err("Device is not a keyboard");
        }

        // In real implementation, would send SET_REPORT to device
        // Bit 0: NumLock, Bit 1: CapsLock, Bit 2: ScrollLock, Bit 3: Compose, Bit 4: Kana

        Ok(())
    }

    pub fn get_report_descriptor(&self, device_id: u16) -> Result<Vec<u8>, &'static str> {
        if (device_id as usize) >= self.devices.len() {
            return Err("Invalid device ID");
        }

        let device = &self.devices[device_id as usize];

        // Simplified report descriptors
        let descriptor = match device.device_type {
            HidDeviceType::Keyboard => vec![
                0x05, 0x01, // Usage Page (Generic Desktop)
                0x09, 0x06, // Usage (Keyboard)
                0xA1, 0x01, // Collection (Application)
                0x75, 0x01, // Report Size (1)
                0x95, 0x08, // Report Count (8)
                0x05, 0x07, // Usage Page (Keyboard/Keypad)
                0x19, 0xE0, // Usage Minimum (Left Control)
                0x29, 0xE7, // Usage Maximum (Right GUI)
                0x15, 0x00, // Logical Minimum (0)
                0x25, 0x01, // Logical Maximum (1)
                0x81, 0x02, // Input (Data, Variable, Absolute)
                0xC0,       // End Collection
            ],
            HidDeviceType::Mouse => vec![
                0x05, 0x01, // Usage Page (Generic Desktop)
                0x09, 0x02, // Usage (Mouse)
                0xA1, 0x01, // Collection (Application)
                0x09, 0x01, // Usage (Pointer)
                0xA1, 0x00, // Collection (Physical)
                0x05, 0x09, // Usage Page (Button)
                0x19, 0x01, // Usage Minimum (Button 1)
                0x29, 0x03, // Usage Maximum (Button 3)
                0x15, 0x00, // Logical Minimum (0)
                0x25, 0x01, // Logical Maximum (1)
                0x75, 0x01, // Report Size (1)
                0x95, 0x03, // Report Count (3)
                0x81, 0x02, // Input (Data, Variable, Absolute)
                0xC0,       // End Collection
                0xC0,       // End Collection
            ],
            _ => return Err("Device type not supported"),
        };

        Ok(descriptor)
    }

    pub fn parse_keyboard_report(&self, data: &[u8]) -> Result<HidKeyboardReport, &'static str> {
        if data.len() < 8 {
            return Err("Report too short");
        }

        Ok(HidKeyboardReport {
            modifier: data[0],
            reserved: data[1],
            keycodes: [
                data[2], data[3], data[4], data[5], data[6], data[7],
            ],
        })
    }

    pub fn parse_mouse_report(&self, data: &[u8]) -> Result<HidMouseReport, &'static str> {
        if data.len() < 4 {
            return Err("Report too short");
        }

        Ok(HidMouseReport {
            buttons: data[0],
            x: data[1] as i8,
            y: data[2] as i8,
            wheel: if data.len() > 3 { data[3] as i8 } else { 0 },
        })
    }
}

impl Default for HidInputDeviceDriver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PciDriver Implementation
// ============================================================================

pub struct HidPciDriver {
    driver: Box<HidInputDeviceDriver>,
}

impl HidPciDriver {
    pub fn new() -> Self {
        HidPciDriver {
            driver: Box::new(HidInputDeviceDriver::new()),
        }
    }

    pub fn get_driver(&self) -> &HidInputDeviceDriver {
        &self.driver
    }

    pub fn get_driver_mut(&mut self) -> &mut HidInputDeviceDriver {
        &mut self.driver
    }
}

impl PciDriver for HidPciDriver {
    fn probe(&mut self, device: &PciDeviceInfo) -> Result<bool, &'static str> {
        // HID devices are typically on USB, but check device class
        if device.class_code != USB_CLASS_HID {
            return Ok(false);
        }

        // Determine device type from subclass/protocol
        let device_type = match device.subclass_code {
            HID_PROTOCOL_KEYBOARD => HidDeviceType::Keyboard,
            HID_PROTOCOL_MOUSE => HidDeviceType::Mouse,
            _ => HidDeviceType::Unknown,
        };

        let hid_device = HidDevice::new(device.vendor_id, device.device_id, device_type);
        self.driver.register_device(hid_device)?;

        Ok(true)
    }

    fn remove(&mut self, device: &PciDeviceInfo) -> Result<(), &'static str> {
        // Find and disconnect device
        for (idx, dev) in self.driver.devices.iter().enumerate() {
            if dev.vendor_id == device.vendor_id && dev.product_id == device.device_id {
                self.driver.disconnect_device(idx as u16)?;
                break;
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "hid_input"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_hid_device_creation() {
        let device = HidDevice::new(0x046D, 0xC31C, HidDeviceType::Mouse);
        assert_eq!(device.vendor_id, 0x046D);
        assert_eq!(device.device_type, HidDeviceType::Mouse);
        assert!(!device.is_connected);
    }

    #[test]
    fn test_keyboard_report() {
        let report = HidKeyboardReport::new();
        assert_eq!(report.modifier, 0);
        assert!(!report.is_key_pressed(HID_KEY_A));
    }

    #[test]
    fn test_mouse_report() {
        let report = HidMouseReport::new();
        assert!(!report.left_button_pressed());
        assert_eq!(report.x, 0);
        assert_eq!(report.y, 0);
    }

    #[test]
    fn test_hid_driver_creation() {
        let driver = HidInputDeviceDriver::new();
        assert_eq!(driver.get_device_count(), 0);
    }

    #[test]
    fn test_register_keyboard() {
        let mut driver = HidInputDeviceDriver::new();
        assert!(driver.probe_keyboard(0x046D, 0xC31C).is_ok());
        assert_eq!(driver.get_device_count(), 1);
    }

    #[test]
    fn test_register_mouse() {
        let mut driver = HidInputDeviceDriver::new();
        assert!(driver.probe_mouse(0x046D, 0xC31C).is_ok());
        assert_eq!(driver.get_device_count(), 1);
    }

    #[test]
    fn test_keyboard_report_submission() {
        let mut driver = HidInputDeviceDriver::new();
        driver.probe_keyboard(0x046D, 0xC31C).unwrap();

        let report = HidKeyboardReport::new();
        assert!(driver.submit_keyboard_report(report).is_ok());
        assert_eq!(driver.get_report_count(), 1);
    }

    #[test]
    fn test_mouse_report_submission() {
        let mut driver = HidInputDeviceDriver::new();
        driver.probe_mouse(0x046D, 0xC31C).unwrap();

        let report = HidMouseReport::new();
        assert!(driver.submit_mouse_report(report).is_ok());
        assert_eq!(driver.get_report_count(), 1);
    }

    #[test]
    fn test_keyboard_parsing() {
        let driver = HidInputDeviceDriver::new();
        let data = vec![0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
        let report = driver.parse_keyboard_report(&data).unwrap();
        assert_eq!(report.modifier, 0x00);
        assert_eq!(report.keycodes[0], 0x04);
    }

    #[test]
    fn test_mouse_parsing() {
        let driver = HidInputDeviceDriver::new();
        let data = vec![0x01, 0x10, 0x20, 0x00];
        let report = driver.parse_mouse_report(&data).unwrap();
        assert!(report.left_button_pressed());
        assert_eq!(report.x, 16);
        assert_eq!(report.y, 32);
    }

    #[test]
    fn test_hid_pci_driver() {
        let driver = HidPciDriver::new();
        assert_eq!(driver.name(), "hid_input");
    }

    #[test]
    fn test_report_descriptor_generation() {
        let mut driver = HidInputDeviceDriver::new();
        let dev_id = driver.register_device(HidDevice::new(0x046D, 0xC31C, HidDeviceType::Keyboard)).unwrap();
        let descriptor = driver.get_report_descriptor(dev_id).unwrap();
        assert!(!descriptor.is_empty());
    }
}
