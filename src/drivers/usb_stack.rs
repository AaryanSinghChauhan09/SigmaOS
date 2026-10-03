//! # USB Stack Implementation
//!
//! Universal Serial Bus subsystem supporting USB 1.1/2.0/3.x.
//! Inspired by Linux drivers/usb/ and FreeBSD sys/dev/usb/.

#![no_std]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

/// USB speeds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbSpeed {
    Low = 0,         // 1.5 Mbps (USB 1.0)
    Full = 1,        // 12 Mbps (USB 1.1)
    High = 2,        // 480 Mbps (USB 2.0)
    Super = 3,       // 5 Gbps (USB 3.0)
    SuperPlus = 4,   // 10 Gbps (USB 3.1)
    SuperPlus20 = 5, // 20 Gbps (USB 3.2)
}

/// USB device states (USB 2.0 spec Chapter 9)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbDeviceState {
    Attached = 0,
    Powered = 1,
    Default = 2,
    Address = 3,
    Configured = 4,
    Suspended = 5,
}

/// USB request types
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UsbRequestType {
    Standard = 0,
    Class = 1,
    Vendor = 2,
}

/// USB recipients
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UsbRecipient {
    Device = 0,
    Interface = 1,
    Endpoint = 2,
    Other = 3,
}

/// Standard USB requests (Chapter 9)
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UsbStandardRequest {
    GetStatus = 0,
    ClearFeature = 1,
    SetFeature = 3,
    SetAddress = 5,
    GetDescriptor = 6,
    SetDescriptor = 7,
    GetConfiguration = 8,
    SetConfiguration = 9,
    GetInterface = 10,
    SetInterface = 11,
    SynchFrame = 12,
}

/// USB descriptor types
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UsbDescriptorType {
    Device = 1,
    Configuration = 2,
    String = 3,
    Interface = 4,
    Endpoint = 5,
    DeviceQualifier = 6,
    OtherSpeedConfiguration = 7,
    InterfacePower = 8,
    OTG = 9,
    Debug = 10,
    InterfaceAssociation = 11,
    BOS = 15,
    DeviceCapability = 16,
    SuperSpeedEndpointCompanion = 48,
}

/// USB device descriptor
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UsbDeviceDescriptor {
    pub b_length: u8,             // 18
    pub b_descriptor_type: u8,    // DEVICE (1)
    pub bcd_usb: u16,             // USB spec release (0x0200 = USB 2.0)
    pub b_device_class: u8,       // Device class code
    pub b_device_sub_class: u8,   // Device subclass code
    pub b_device_protocol: u8,    // Device protocol code
    pub b_max_packet_size0: u8,   // Max packet size for endpoint 0
    pub id_vendor: u16,           // Vendor ID
    pub id_product: u16,          // Product ID
    pub bcd_device: u16,          // Device release number
    pub i_manufacturer: u8,       // Manufacturer string index
    pub i_product: u8,            // Product string index
    pub i_serial_number: u8,      // Serial number string index
    pub b_num_configurations: u8, // Number of configurations
}

/// USB configuration descriptor
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UsbConfigDescriptor {
    pub b_length: u8,              // 9
    pub b_descriptor_type: u8,     // CONFIGURATION (2)
    pub w_total_length: u16,       // Total length including interfaces/endpoints
    pub b_num_interfaces: u8,      // Number of interfaces
    pub b_configuration_value: u8, // Configuration value for SetConfiguration
    pub i_configuration: u8,       // Configuration string index
    pub bm_attributes: u8,         // Attributes (self-powered, remote wakeup)
    pub b_max_power: u8,           // Max power in 2mA units
}

/// USB interface descriptor
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UsbInterfaceDescriptor {
    pub b_length: u8,              // 9
    pub b_descriptor_type: u8,     // INTERFACE (4)
    pub b_interface_number: u8,    // Interface number
    pub b_alternate_setting: u8,   // Alternate setting
    pub b_num_endpoints: u8,       // Number of endpoints (excluding EP0)
    pub b_interface_class: u8,     // Interface class code
    pub b_interface_sub_class: u8, // Interface subclass code
    pub b_interface_protocol: u8,  // Interface protocol code
    pub i_interface: u8,           // Interface string index
}

/// USB endpoint descriptor
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UsbEndpointDescriptor {
    pub b_length: u8,           // 7
    pub b_descriptor_type: u8,  // ENDPOINT (5)
    pub b_endpoint_address: u8, // Endpoint address (bit 7: direction)
    pub bm_attributes: u8,      // Transfer type and sync type
    pub w_max_packet_size: u16, // Maximum packet size
    pub b_interval: u8,         // Polling interval for interrupt/isochronous
}

/// USB endpoint direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDirection {
    Out = 0, // Host to device
    In = 1,  // Device to host
}

/// USB transfer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbTransferType {
    Control = 0,
    Isochronous = 1,
    Bulk = 2,
    Interrupt = 3,
}

/// USB endpoint information
#[derive(Debug, Clone)]
pub struct UsbEndpoint {
    pub address: u8,
    pub direction: UsbDirection,
    pub transfer_type: UsbTransferType,
    pub max_packet_size: u16,
    pub interval: u8,
}

impl UsbEndpoint {
    pub fn from_descriptor(desc: &UsbEndpointDescriptor) -> Self {
        Self {
            address: desc.b_endpoint_address & 0x0F,
            direction: if desc.b_endpoint_address & 0x80 != 0 {
                UsbDirection::In
            } else {
                UsbDirection::Out
            },
            transfer_type: unsafe { core::mem::transmute(desc.bm_attributes & 0x03) },
            max_packet_size: desc.w_max_packet_size & 0x7FF,
            interval: desc.b_interval,
        }
    }
}

/// USB device
pub struct UsbDevice {
    pub address: AtomicU8,
    pub speed: UsbSpeed,
    pub state: AtomicU8,
    pub descriptor: UsbDeviceDescriptor,
    pub configuration: u8,
    pub endpoints: Vec<UsbEndpoint>,
    pub port: u8,
    pub hub_address: u8,
}

impl UsbDevice {
    pub fn new(speed: UsbSpeed, port: u8) -> Self {
        Self {
            address: AtomicU8::new(0),
            speed,
            state: AtomicU8::new(UsbDeviceState::Attached as u8),
            descriptor: unsafe { core::mem::zeroed() },
            configuration: 0,
            endpoints: Vec::new(),
            port,
            hub_address: 0,
        }
    }

    pub fn set_address(&self, addr: u8) {
        self.address.store(addr, Ordering::Release);
        self.state
            .store(UsbDeviceState::Address as u8, Ordering::Release);
    }

    pub fn get_address(&self) -> u8 {
        self.address.load(Ordering::Acquire)
    }

    pub fn get_state(&self) -> UsbDeviceState {
        let state_val = self.state.load(Ordering::Acquire);
        unsafe { core::mem::transmute(state_val) }
    }
}

/// USB host controller interface
pub trait UsbHostController {
    fn reset_port(&mut self, port: u8) -> Result<(), UsbError>;
    fn get_port_speed(&self, port: u8) -> Result<UsbSpeed, UsbError>;
    fn control_transfer(
        &mut self,
        device: &UsbDevice,
        setup: &UsbSetupPacket,
        data: Option<&mut [u8]>,
    ) -> Result<usize, UsbError>;
    fn bulk_transfer(
        &mut self,
        device: &UsbDevice,
        endpoint: u8,
        data: &mut [u8],
        direction: UsbDirection,
    ) -> Result<usize, UsbError>;
    fn interrupt_transfer(
        &mut self,
        device: &UsbDevice,
        endpoint: u8,
        data: &mut [u8],
        direction: UsbDirection,
    ) -> Result<usize, UsbError>;
}

/// USB setup packet (control transfers)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UsbSetupPacket {
    pub bm_request_type: u8, // Request type (D7: direction, D6-5: type, D4-0: recipient)
    pub b_request: u8,       // Request code
    pub w_value: u16,        // Value (varies by request)
    pub w_index: u16,        // Index (interface, endpoint, etc.)
    pub w_length: u16,       // Data transfer length
}

impl UsbSetupPacket {
    pub fn get_descriptor(
        desc_type: UsbDescriptorType,
        desc_index: u8,
        lang_id: u16,
        length: u16,
    ) -> Self {
        Self {
            bm_request_type: 0x80, // Device to host, standard, device
            b_request: UsbStandardRequest::GetDescriptor as u8,
            w_value: ((desc_type as u16) << 8) | (desc_index as u16),
            w_index: lang_id,
            w_length: length,
        }
    }

    pub fn set_address(address: u8) -> Self {
        Self {
            bm_request_type: 0x00, // Host to device, standard, device
            b_request: UsbStandardRequest::SetAddress as u8,
            w_value: address as u16,
            w_index: 0,
            w_length: 0,
        }
    }

    pub fn set_configuration(config: u8) -> Self {
        Self {
            bm_request_type: 0x00,
            b_request: UsbStandardRequest::SetConfiguration as u8,
            w_value: config as u16,
            w_index: 0,
            w_length: 0,
        }
    }
}

/// USB enumeration engine
pub struct UsbEnumerator;

impl UsbEnumerator {
    /// Enumerate new USB device
    pub fn enumerate_device<HC: UsbHostController>(
        hc: &mut HC,
        port: u8,
        next_address: u8,
    ) -> Result<UsbDevice, UsbError> {
        // Reset port
        hc.reset_port(port)?;

        // Determine speed
        let speed = hc.get_port_speed(port)?;

        // Create device at default address 0
        let mut device = UsbDevice::new(speed, port);

        // Get initial 8 bytes of device descriptor to determine max packet size
        let mut desc_buf = [0u8; 18];
        let setup = UsbSetupPacket::get_descriptor(UsbDescriptorType::Device, 0, 0, 8);

        hc.control_transfer(&device, &setup, Some(&mut desc_buf[..8]))?;

        // Set new address
        let setup_addr = UsbSetupPacket::set_address(next_address);
        hc.control_transfer(&device, &setup_addr, None)?;
        device.set_address(next_address);

        // Get full device descriptor
        let setup_full = UsbSetupPacket::get_descriptor(UsbDescriptorType::Device, 0, 0, 18);

        hc.control_transfer(&device, &setup_full, Some(&mut desc_buf))?;

        device.descriptor =
            unsafe { core::ptr::read_unaligned(desc_buf.as_ptr() as *const UsbDeviceDescriptor) };

        // Set configuration 1
        let setup_config = UsbSetupPacket::set_configuration(1);
        hc.control_transfer(&device, &setup_config, None)?;
        device.configuration = 1;
        device
            .state
            .store(UsbDeviceState::Configured as u8, Ordering::Release);

        Ok(device)
    }
}

/// USB class codes
pub mod class_codes {
    pub const AUDIO: u8 = 0x01;
    pub const CDC: u8 = 0x02;
    pub const HID: u8 = 0x03;
    pub const PHYSICAL: u8 = 0x05;
    pub const IMAGE: u8 = 0x06;
    pub const PRINTER: u8 = 0x07;
    pub const MASS_STORAGE: u8 = 0x08;
    pub const HUB: u8 = 0x09;
    pub const CDC_DATA: u8 = 0x0A;
    pub const SMART_CARD: u8 = 0x0B;
    pub const CONTENT_SECURITY: u8 = 0x0D;
    pub const VIDEO: u8 = 0x0E;
    pub const PERSONAL_HEALTHCARE: u8 = 0x0F;
    pub const AUDIO_VIDEO: u8 = 0x10;
    pub const BILLBOARD: u8 = 0x11;
    pub const USB_TYPE_C_BRIDGE: u8 = 0x12;
    pub const DIAGNOSTIC: u8 = 0xDC;
    pub const WIRELESS_CONTROLLER: u8 = 0xE0;
    pub const MISCELLANEOUS: u8 = 0xEF;
    pub const APPLICATION_SPECIFIC: u8 = 0xFE;
    pub const VENDOR_SPECIFIC: u8 = 0xFF;
}

/// USB errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbError {
    Timeout,
    Stalled,
    Babble,
    DataBufferError,
    CrcError,
    NoResponse,
    InvalidDescriptor,
    InvalidDevice,
    NotSupported,
    IoError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn test_usb_descriptor_sizes() {
        assert_eq!(size_of::<UsbDeviceDescriptor>(), 18);
        assert_eq!(size_of::<UsbConfigDescriptor>(), 9);
        assert_eq!(size_of::<UsbInterfaceDescriptor>(), 9);
        assert_eq!(size_of::<UsbEndpointDescriptor>(), 7);
        assert_eq!(size_of::<UsbSetupPacket>(), 8);
    }

    #[test]
    fn test_usb_device_creation() {
        let device = UsbDevice::new(UsbSpeed::High, 1);
        assert_eq!(device.get_address(), 0);
        assert_eq!(device.speed, UsbSpeed::High);
        assert_eq!(device.port, 1);
    }

    #[test]
    fn test_setup_packet_construction() {
        let setup = UsbSetupPacket::set_address(5);
        assert_eq!(setup.bm_request_type, 0x00);
        assert_eq!(setup.b_request, 5);
        assert_eq!(setup.w_value, 5);
    }

    #[test]
    fn test_endpoint_parsing() {
        let desc = UsbEndpointDescriptor {
            b_length: 7,
            b_descriptor_type: 5,
            b_endpoint_address: 0x81, // EP1 IN
            bm_attributes: 0x02,      // Bulk
            w_max_packet_size: 512,
            b_interval: 0,
        };

        let ep = UsbEndpoint::from_descriptor(&desc);
        assert_eq!(ep.address, 1);
        assert_eq!(ep.direction, UsbDirection::In);
        assert_eq!(ep.transfer_type, UsbTransferType::Bulk);
        assert_eq!(ep.max_packet_size, 512);
    }
}
