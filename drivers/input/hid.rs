// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/hid.rs — HID (Human Interface Device) Driver
//
// Implements the USB HID driver for keyboards, mice, and other input devices.
// Supports USB HID protocol 1.11.
// Based on Linux kernel hid driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── HID Error Codes ─────────────────────────────────────────────

pub const HID_OK: I32 = 0;
pub const HID_ERR_NO_DEVICE: I32 = -1;
pub const HID_ERR_INIT_FAILED: I32 = -2;
pub const HID_ERR_OUT_OF_MEM: I32 = -3;
pub const HID_ERR_NOT_SUPPORTED: I32 = -4;
pub const HID_ERR_INVALID_PARAM: I32 = -5;

// ─── HID Report Types ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HidReportType {
    Input,
    Output,
    Feature,
}

// ─── HID Usage Page ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HidUsagePage {
    GenericDesktop = 0x01,
    Simulation = 0x02,
    VR = 0x03,
    GenericDevice = 0x04,
    Keyboard = 0x07,
    LED = 0x08,
    Button = 0x09,
    Ordinal = 0x0A,
    Telephony = 0x0B,
    Consumer = 0x0C,
    Digitizer = 0x0D,
    PhysicalInputDevice = 0x0F,
    Unicode = 0x10,
    AlphanumericDisplay = 0x14,
    MedicalInstrument = 0x40,
    MonitorPage = 0x80,
    MonitorEnumeratedValues = 0x81,
    MonitorVirtualControls = 0x82,
    PowerDevice = 0x84,
    BatterySystem = 0x85,
    BarcodeScanner = 0x8C,
    Scale = 0x8D,
    MagneticStripeReader = 0x8E,
    PointOfSale = 0x8F,
    CameraControl = 0x90,
    Arcade = 0x91,
}

// ─── HID Descriptor ───────────────────────────────────────────

#[repr(C)]
pub struct HidDescriptor {
    pub length: U8,
    pub descriptor_type: U8,
    bcd_hid: U16,
    b_country_code: U8,
    b_num_descriptors: U8,
    pub report_descriptor_type: U8,
    pub report_descriptor_length: U16,
}

impl HidDescriptor {
    pub const fn new() -> Self {
        HidDescriptor {
            length: 0,
            descriptor_type: 0,
            bcd_hid: 0,
            b_country_code: 0,
            b_num_descriptors: 0,
            report_descriptor_type: 0,
            report_descriptor_length: 0,
        }
    }
}

// ─── HID Report Descriptor Item ─────────────────────────────

#[repr(C)]
pub struct HidReportItem {
    pub item_type: U8,
    pub tag: U8,
    pub size: U8,
    pub data: U32,
}

impl HidReportItem {
    pub const fn new() -> Self {
        HidReportItem {
            item_type: 0,
            tag: 0,
            size: 0,
            data: 0,
        }
    }
}

// ─── HID Input Report ───────────────────────────────────────

#[repr(C)]
pub struct HidInputReport {
    pub report_id: U8,
    pub data: [U8; 64],
    pub length: U8,
}

impl HidInputReport {
    pub const fn new() -> Self {
        HidInputReport {
            report_id: 0,
            data: [0; 64],
            length: 0,
        }
    }
}

// ─── HID Output Report ──────────────────────────────────────

#[repr(C)]
pub struct HidOutputReport {
    pub report_id: U8,
    pub data: [U8; 64],
    pub length: U8,
}

impl HidOutputReport {
    pub const fn new() -> Self {
        HidOutputReport {
            report_id: 0,
            data: [0; 64],
            length: 0,
        }
    }
}

// ─── HID Device Structure ───────────────────────────────────

pub struct HidDevice {
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub enabled: bool,
    pub hid_descriptor: HidDescriptor,
    pub report_descriptor: [U8; 256],
    pub report_descriptor_length: U16,
    pub usage_page: HidUsagePage,
    pub usage: U16,
    pub input_report: HidInputReport,
    pub output_report: HidOutputReport,
    pub max_input_report_length: U8,
    pub max_output_report_length: U8,
    pub num_reports: U8,
}

impl HidDevice {
    pub const fn new() -> Self {
        HidDevice {
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            enabled: false,
            hid_descriptor: HidDescriptor::new(),
            report_descriptor: [0; 256],
            report_descriptor_length: 0,
            usage_page: HidUsagePage::GenericDesktop,
            usage: 0,
            input_report: HidInputReport::new(),
            output_report: HidOutputReport::new(),
            max_input_report_length: 64,
            max_output_report_length: 64,
            num_reports: 0,
        }
    }

    /// Initialize HID device
    fn init_hid(&mut self, device_id: U16, vendor_id: U16) -> I32 {
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        // In a real implementation, this would:
        // 1. Get HID descriptor from USB device
        // 2. Get report descriptor from USB device
        // 3. Parse report descriptor
        // 4. Set up input/output reports

        // Stub: set default values
        self.hid_descriptor.length = 9;
        self.hid_descriptor.descriptor_type = 0x21;
        self.hid_descriptor.bcd_hid = 0x0111;
        self.hid_descriptor.b_country_code = 0;
        self.hid_descriptor.b_num_descriptors = 1;
        self.hid_descriptor.report_descriptor_type = 0x22;
        self.hid_descriptor.report_descriptor_length = 64;

        self.report_descriptor_length = 64;

        self.initialized = true;
        self.enabled = true;

        HID_OK
    }

    /// Parse report descriptor
    fn parse_report_descriptor(&mut self) -> I32 {
        // In a real implementation, this would parse the HID report descriptor
        // and extract usage page, usage, report sizes, etc.

        // Stub: set default values
        self.usage_page = HidUsagePage::GenericDesktop;
        self.usage = 0x06; // Keyboard

        HID_OK
    }

    /// Get input report
    fn get_input_report(&mut self, report_id: U8) -> I32 {
        if !self.initialized || !self.enabled {
            return HID_ERR_INIT_FAILED;
        }

        // In a real implementation, this would read from the USB device
        self.input_report.report_id = report_id;
        self.input_report.length = 8;

        HID_OK
    }

    /// Set output report
    fn set_output_report(&mut self, report_id: U8, data: &[U8]) -> I32 {
        if !self.initialized || !self.enabled {
            return HID_ERR_INIT_FAILED;
        }

        if data.len() > 64 {
            return HID_ERR_INVALID_PARAM;
        }

        // In a real implementation, this would write to the USB device
        self.output_report.report_id = report_id;
        self.output_report.length = data.len() as U8;
        for i in 0..data.len() {
            self.output_report.data[i] = data[i];
        }

        HID_OK
    }
}

// ─── HID Device Trait ───────────────────────────────────────

pub trait HidDriver {
    /// Initialize the HID device
    fn init(&mut self, device_id: U16, vendor_id: U16) -> I32;

    /// Check if device is initialized
    fn is_initialized(&self) -> bool;

    /// Get device name
    fn get_device_name(&self) -> &'static str;

    /// Get usage page
    fn get_usage_page(&self) -> HidUsagePage;

    /// Get usage
    fn get_usage(&self) -> U16;

    /// Get input report
    fn get_input_report(&mut self, report_id: U8) -> I32;

    /// Set output report
    fn set_output_report(&mut self, report_id: U8, data: &[U8]) -> I32;

    /// Get feature report
    fn get_feature_report(&mut self, report_id: U8, buffer: &mut [U8]) -> I32;

    /// Set feature report
    fn set_feature_report(&mut self, report_id: U8, data: &[U8]) -> I32;

    /// Enable device
    fn enable(&mut self) -> I32;

    /// Disable device
    fn disable(&mut self) -> I32;

    /// Reset device
    fn reset(&mut self) -> I32;

    /// Shutdown device
    fn shutdown(&mut self) -> I32;
}

// ─── Implement HidDriver Trait ─────────────────────────────

impl HidDriver for HidDevice {
    fn init(&mut self, device_id: U16, vendor_id: U16) -> I32 {
        self.init_hid(device_id, vendor_id);
        self.parse_report_descriptor()
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "USB HID Device"
    }

    fn get_usage_page(&self) -> HidUsagePage {
        self.usage_page
    }

    fn get_usage(&self) -> U16 {
        self.usage
    }

    fn get_input_report(&mut self, report_id: U8) -> I32 {
        self.get_input_report(report_id)
    }

    fn set_output_report(&mut self, report_id: U8, data: &[U8]) -> I32 {
        self.set_output_report(report_id, data)
    }

    fn get_feature_report(&mut self, report_id: U8, buffer: &mut [U8]) -> I32 {
        if !self.initialized || !self.enabled {
            return HID_ERR_INIT_FAILED;
        }

        // In a real implementation, this would read feature report from USB device
        HID_OK
    }

    fn set_feature_report(&mut self, report_id: U8, data: &[U8]) -> I32 {
        if !self.initialized || !self.enabled {
            return HID_ERR_INIT_FAILED;
        }

        // In a real implementation, this would write feature report to USB device
        HID_OK
    }

    fn enable(&mut self) -> I32 {
        if !self.initialized {
            return HID_ERR_INIT_FAILED;
        }

        self.enabled = true;
        HID_OK
    }

    fn disable(&mut self) -> I32 {
        if !self.initialized {
            return HID_ERR_INIT_FAILED;
        }

        self.enabled = false;
        HID_OK
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return HID_ERR_INIT_FAILED;
        }

        HID_OK
    }

    fn shutdown(&mut self) -> I32 {
        if !self.initialized {
            return HID_ERR_INIT_FAILED;
        }

        self.enabled = false;
        self.initialized = false;
        HID_OK
    }
}

// ─── Global HID Device ─────────────────────────────────────

static mut G_HID: HidDevice = HidDevice::new();

// ─── C-ABI Exports ─────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn hid_init(device_id: U16, vendor_id: U16) -> I32 {
    G_HID.init(device_id, vendor_id)
}

#[no_mangle]
pub unsafe extern "C" fn hid_is_initialized() -> I32 {
    if G_HID.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn hid_shutdown() -> I32 {
    G_HID.shutdown()
}

#[no_mangle]
pub unsafe extern "C" fn hid_get_input_report(report_id: U8) -> I32 {
    G_HID.get_input_report(report_id)
}

#[no_mangle]
pub unsafe extern "C" fn hid_set_output_report(report_id: U8, data: *const U8, length: U8) -> I32 {
    if data.is_null() {
        return HID_ERR_INVALID_PARAM;
    }

    let slice = core::slice::from_raw_parts(data, length as usize);
    G_HID.set_output_report(report_id, slice)
}
