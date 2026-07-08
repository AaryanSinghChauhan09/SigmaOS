// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/usb/usb_device_base.rs — Base Device Trait for USB Drivers
//
// Defines the OOP base class for all USB devices using Rust traits.
// This provides a common interface for USB operations across different controllers.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── USB Error Codes ─────────────────────────────────────────────────────

pub const USB_OK: I32 = 0;
pub const USB_ERR_NO_DEVICE: I32 = -1;
pub const USB_ERR_INIT_FAILED: I32 = -2;
pub const USB_ERR_OUT_OF_MEM: I32 = -3;
pub const USB_ERR_NOT_SUPPORTED: I32 = -4;
pub const USB_ERR_TRANSFER_FAILED: I32 = -5;
pub const USB_ERR_TIMEOUT: I32 = -6;
pub const USB_ERR_STALL: I32 = -7;

// ─── USB Speed ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UsbSpeed {
    Low,       // 1.5 Mbps
    Full,      // 12 Mbps
    High,      // 480 Mbps
    Super,     // 5 Gbps
    SuperPlus, // 10 Gbps
}

// ─── USB Device Class ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UsbDeviceClass {
    PerInterface,
    Audio,
    Comm,
    HID,
    Physical,
    Image,
    Printer,
    MassStorage,
    Hub,
    CDCData,
    SmartCard,
    ContentSecurity,
    Video,
    PersonalHealthcare,
    AudioVideo,
    Billboard,
    USBTypeCBridge,
    Wireless,
    Misc,
    ApplicationSpecific,
    VendorSpecific,
}

// ─── USB Endpoint Type ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UsbEndpointType {
    Control,
    Isochronous,
    Bulk,
    Interrupt,
}

// ─── USB Endpoint Direction ───────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UsbDirection {
    Out,
    In,
}

// ─── USB Endpoint Descriptor ─────────────────────────────────────

#[repr(C)]
pub struct UsbEndpointDescriptor {
    pub length: U8,
    pub descriptor_type: U8,
    pub endpoint_address: U8,
    pub attributes: U8,
    pub max_packet_size: U16,
    pub interval: U8,
}

impl UsbEndpointDescriptor {
    pub const fn new() -> Self {
        UsbEndpointDescriptor {
            length: 0,
            descriptor_type: 0,
            endpoint_address: 0,
            attributes: 0,
            max_packet_size: 0,
            interval: 0,
        }
    }

    pub fn get_endpoint_number(&self) -> U8 {
        self.endpoint_address & 0x0F
    }

    pub fn get_direction(&self) -> UsbDirection {
        if self.endpoint_address & 0x80 != 0 {
            UsbDirection::In
        } else {
            UsbDirection::Out
        }
    }

    pub fn get_type(&self) -> UsbEndpointType {
        match self.attributes & 0x03 {
            0 => UsbEndpointType::Control,
            1 => UsbEndpointType::Isochronous,
            2 => UsbEndpointType::Bulk,
            3 => UsbEndpointType::Interrupt,
            _ => UsbEndpointType::Control,
        }
    }
}

// ─── USB Interface Descriptor ───────────────────────────────────

#[repr(C)]
pub struct UsbInterfaceDescriptor {
    pub length: U8,
    pub descriptor_type: U8,
    pub interface_number: U8,
    pub alternate_setting: U8,
    pub endpoint_count: U8,
    pub interface_class: U8,
    pub interface_subclass: U8,
    pub interface_protocol: U8,
    pub interface_string: U8,
}

impl UsbInterfaceDescriptor {
    pub const fn new() -> Self {
        UsbInterfaceDescriptor {
            length: 0,
            descriptor_type: 0,
            interface_number: 0,
            alternate_setting: 0,
            endpoint_count: 0,
            interface_class: 0,
            interface_subclass: 0,
            interface_protocol: 0,
            interface_string: 0,
        }
    }

    pub fn get_class(&self) -> UsbDeviceClass {
        match self.interface_class {
            0 => UsbDeviceClass::PerInterface,
            1 => UsbDeviceClass::Audio,
            2 => UsbDeviceClass::Comm,
            3 => UsbDeviceClass::HID,
            5 => UsbDeviceClass::Physical,
            6 => UsbDeviceClass::Image,
            7 => UsbDeviceClass::Printer,
            8 => UsbDeviceClass::MassStorage,
            9 => UsbDeviceClass::Hub,
            10 => UsbDeviceClass::CDCData,
            11 => UsbDeviceClass::SmartCard,
            13 => UsbDeviceClass::ContentSecurity,
            14 => UsbDeviceClass::Video,
            15 => UsbDeviceClass::PersonalHealthcare,
            16 => UsbDeviceClass::AudioVideo,
            17 => UsbDeviceClass::Billboard,
            18 => UsbDeviceClass::USBTypeCBridge,
            224 => UsbDeviceClass::Wireless,
            239 => UsbDeviceClass::Misc,
            254 => UsbDeviceClass::ApplicationSpecific,
            255 => UsbDeviceClass::VendorSpecific,
            _ => UsbDeviceClass::VendorSpecific,
        }
    }
}

// ─── USB Device Descriptor ─────────────────────────────────────

#[repr(C)]
pub struct UsbDeviceDescriptor {
    pub length: U8,
    pub descriptor_type: U8,
    pub usb_version: U16,
    pub device_class: U8,
    pub device_subclass: U8,
    pub device_protocol: U8,
    pub max_packet_size0: U8,
    pub vendor_id: U16,
    pub product_id: U16,
    pub device_version: U16,
    pub manufacturer_string: U8,
    pub product_string: U8,
    pub serial_number_string: U8,
    pub num_configurations: U8,
}

impl UsbDeviceDescriptor {
    pub const fn new() -> Self {
        UsbDeviceDescriptor {
            length: 0,
            descriptor_type: 0,
            usb_version: 0,
            device_class: 0,
            device_subclass: 0,
            device_protocol: 0,
            max_packet_size0: 0,
            vendor_id: 0,
            product_id: 0,
            device_version: 0,
            manufacturer_string: 0,
            product_string: 0,
            serial_number_string: 0,
            num_configurations: 0,
        }
    }
}

// ─── USB Transfer Request ───────────────────────────────────────

#[repr(C)]
pub struct UsbTransfer {
    pub endpoint: U8,
    pub direction: UsbDirection,
    pub buffer: *mut U8,
    pub length: U32,
    pub actual_length: U32,
    pub status: I32,
    pub timeout_ms: U32,
    pub complete: bool,
}

impl UsbTransfer {
    pub const fn new() -> Self {
        UsbTransfer {
            endpoint: 0,
            direction: UsbDirection::Out,
            buffer: 0 as *mut U8,
            length: 0,
            actual_length: 0,
            status: USB_OK,
            timeout_ms: 1000,
            complete: false,
        }
    }
}

// ─── USB Device Trait ─────────────────────────────────────────

/// Trait for USB controller operations
pub trait UsbController {
    /// Initialize the USB controller
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if controller is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get controller name
    fn get_controller_name(&self) -> &'static str;
    
    /// Get USB speed
    fn get_speed(&self) -> UsbSpeed;
    
    /// Reset the controller
    fn reset(&mut self) -> I32;
    
    /// Shutdown the controller
    fn shutdown(&mut self) -> I32;
    
    /// Get device descriptor
    fn get_device_descriptor(&self, descriptor: *mut UsbDeviceDescriptor) -> I32;
    
    /// Get configuration descriptor
    fn get_configuration_descriptor(&self, config_index: U8, buffer: *mut U8, length: *mut U16) -> I32;
    
    /// Control transfer
    fn control_transfer(&mut self, request_type: U8, request: U8, value: U16, index: U16, buffer: *mut U8, length: U16) -> I32;
    
    /// Bulk transfer
    fn bulk_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32;
    
    /// Interrupt transfer
    fn interrupt_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32;
    
    /// Isochronous transfer
    fn isochronous_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32;
    
    /// Submit transfer request
    fn submit_transfer(&mut self, transfer: *mut UsbTransfer) -> I32;
    
    /// Cancel transfer
    fn cancel_transfer(&mut self, transfer: *mut UsbTransfer) -> I32;
    
    /// Allocate USB device address
    fn allocate_address(&mut self) -> U8;
    
    /// Set device address
    fn set_address(&mut self, address: U8) -> I32;
    
    /// Set device configuration
    fn set_configuration(&mut self, config_value: U8) -> I32;
    
    /// Get port status
    fn get_port_status(&self, port: U8) -> U32;
    
    /// Set port feature
    fn set_port_feature(&mut self, port: U8, feature: U16) -> I32;
    
    /// Clear port feature
    fn clear_port_feature(&mut self, port: U8, feature: U16) -> I32;
}

// ─── USB Hub Trait ─────────────────────────────────────────────

/// Trait for USB hub operations
pub trait UsbHub {
    /// Get number of ports
    fn get_port_count(&self) -> U8;
    
    /// Get hub descriptor
    fn get_hub_descriptor(&self, descriptor: *mut U8, length: *mut U16) -> I32;
    
    /// Power on port
    fn power_on_port(&mut self, port: U8) -> I32;
    
    /// Power off port
    fn power_off_port(&mut self, port: U8) -> I32;
    
    /// Reset port
    fn reset_port(&mut self, port: U8) -> I32;
    
    /// Enable port
    fn enable_port(&mut self, port: U8) -> I32;
    
    /// Disable port
    fn disable_port(&mut self, port: U8) -> I32;
}
