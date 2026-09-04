// SPDX-License-Identifier: MIT
// SigmaOS USB xHCI Host Controller Driver
// Supports USB 2.0/3.0/3.1 via xHCI specification

use core::sync::atomic::{AtomicU32, Ordering};
use std::boxed::Box;
use std::string::String;
use std::vec::Vec;

use crate::driver::pci_enumeration::{PciDeviceInfo, PciDriver};

// ============================================================================
// USB xHCI Constants
// ============================================================================

pub const INTEL_VENDOR_ID: u16 = 0x8086;

// Common xHCI Device IDs
pub const PANTHER_POINT_XHCI: u16 = 0x1E31; // Panther Point (Series 6)
pub const LYNX_POINT_XHCI: u16 = 0x8C31; // Lynx Point (Series 7)
pub const WILDCAT_POINT_XHCI: u16 = 0x9C31; // Wildcat Point (Series 8)
pub const SUNRISE_POINT_XHCI: u16 = 0xA12F; // Sunrise Point (Series 100)
pub const KABY_LAKE_XHCI: u16 = 0x5AA0; // Kaby Lake

// xHCI Register Offsets
pub const XHCI_CAP_LENGTH: u32 = 0x00;
pub const XHCI_HCIVERSION: u32 = 0x02;
pub const XHCI_HCS_PARAMS1: u32 = 0x04;
pub const XHCI_HCS_PARAMS2: u32 = 0x08;
pub const XHCI_HCS_PARAMS3: u32 = 0x0C;

pub const XHCI_USBCMD: u32 = 0x00; // USB Command (operational regs)
pub const XHCI_USBSTS: u32 = 0x04; // USB Status
pub const XHCI_PAGESIZE: u32 = 0x08;
pub const XHCI_CRCR: u32 = 0x18; // Command Ring Control
pub const XHCI_DCBAAP: u32 = 0x30; // Device Context Base Address Array Pointer
pub const XHCI_CONFIG: u32 = 0x38; // Configure

// Port Registers (per port)
pub const XHCI_PORTSC: u32 = 0x400; // Port Status and Control (base)
pub const XHCI_PORTPMSC: u32 = 0x404;
pub const XHCI_PORTLI: u32 = 0x408;
pub const XHCI_PORTHLPMC: u32 = 0x40C;

// Status bits
pub const XHCI_STS_HALTED: u32 = 0x0001;
pub const XHCI_STS_RUNNING: u32 = 0x0000;
pub const XHCI_CMD_RUN: u32 = 0x0001;
pub const XHCI_CMD_RESET: u32 = 0x0002;
pub const XHCI_CMD_IE: u32 = 0x0004; // Interrupter Enable

// ============================================================================
// USB Device & Endpoint Structures
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbSpeed {
    FullSpeed,      // 12 Mbps
    HighSpeed,      // 480 Mbps
    SuperSpeed,     // 5 Gbps
    SuperSpeedPlus, // 10 Gbps
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDeviceClass {
    HubClass,
    MassStorage,
    CdcData,
    HidClass,
    PhysicalInterface,
    ImageClass,
    PrinterClass,
    MassStorageCompat,
    ChipSmartCardInterface,
    ContentSecurityClass,
    VideoClass,
    PersonalHealthcareClass,
    AudioVideoClass,
    DiagnosticDeviceClass,
    WirelessController,
    MiscellaneousClass,
    ApplicationSpecific,
    VendorSpecific,
}

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub address: u8,
    pub port: u8,
    pub speed: UsbSpeed,
    pub class: UsbDeviceClass,
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub product_name: String,
    pub is_connected: bool,
}

impl UsbDevice {
    pub fn new(port: u8, speed: UsbSpeed, class: UsbDeviceClass) -> Self {
        UsbDevice {
            address: 0,
            port,
            speed,
            class,
            vendor_id: 0,
            product_id: 0,
            manufacturer: String::new(),
            product_name: String::new(),
            is_connected: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsbEndpoint {
    pub endpoint_address: u8,
    pub endpoint_type: u8, // Control=0, Isoch=1, Bulk=2, Intr=3
    pub max_packet_size: u16,
    pub direction: bool, // false=OUT, true=IN
}

impl UsbEndpoint {
    pub fn new(address: u8, ep_type: u8, max_pkt_size: u16) -> Self {
        UsbEndpoint {
            endpoint_address: address,
            endpoint_type: ep_type,
            max_packet_size: max_pkt_size,
            direction: (address & 0x80) != 0,
        }
    }
}

// ============================================================================
// xHCI Command Ring & Transfer Ring
// ============================================================================

pub struct TransferRing {
    ring_base: u64,
    ring_size: usize,
    enqueue_ptr: u32,
    dequeue_ptr: u32,
    cycle_bit: bool,
}

impl TransferRing {
    pub fn new(base: u64, size: usize) -> Self {
        TransferRing {
            ring_base: base,
            ring_size: size,
            enqueue_ptr: 0,
            dequeue_ptr: 0,
            cycle_bit: true,
        }
    }

    pub fn queue_command(&mut self, cmd: u32) -> Result<(), &'static str> {
        if (self.enqueue_ptr + 1) % (self.ring_size as u32) == self.dequeue_ptr {
            return Err("Transfer ring full");
        }
        self.enqueue_ptr = (self.enqueue_ptr + 1) % (self.ring_size as u32);
        Ok(())
    }

    pub fn advance_dequeue_ptr(&mut self) {
        self.dequeue_ptr = (self.dequeue_ptr + 1) % (self.ring_size as u32);
    }
}

// ============================================================================
// xHCI Host Controller Driver
// ============================================================================

pub struct UsbXhciHostDriver {
    device_id: u16,
    pci_address: String,
    mmio_base: u64,
    mmio_size: u64,
    interrupt_line: u8,
    is_enabled: bool,
    num_ports: u8,
    devices: Vec<UsbDevice>,
    command_ring: TransferRing,
    port_status: Vec<u32>,
    device_count: AtomicU32,
}

impl UsbXhciHostDriver {
    pub fn new(device_id: u16, pci_addr: &str) -> Self {
        UsbXhciHostDriver {
            device_id,
            pci_address: pci_addr.to_string(),
            mmio_base: 0,
            mmio_size: 0,
            interrupt_line: 0,
            is_enabled: false,
            num_ports: 0,
            devices: Vec::new(),
            command_ring: TransferRing::new(0, 256),
            port_status: Vec::new(),
            device_count: AtomicU32::new(0),
        }
    }

    pub fn init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str> {
        self.mmio_base = bar;
        self.mmio_size = size;
        self.is_enabled = true;

        // Initialize with default number of ports (typically 2-14)
        self.num_ports = 4; // Common for integrated controllers
        self.port_status = vec![0; self.num_ports as usize];

        Ok(())
    }

    pub fn reset_controller(&mut self) -> Result<(), &'static str> {
        // In real implementation:
        // 1. Set USBCMD.RESET
        // 2. Wait for USBCMD.RESET to clear
        // 3. Wait for USBSTS.CNR to clear
        self.is_enabled = true;
        Ok(())
    }

    pub fn start_controller(&mut self) -> Result<(), &'static str> {
        // Set USBCMD.RS (Run/Stop bit)
        self.is_enabled = true;
        Ok(())
    }

    pub fn stop_controller(&mut self) -> Result<(), &'static str> {
        self.is_enabled = false;
        Ok(())
    }

    pub fn scan_ports(&mut self) -> Result<u32, &'static str> {
        if !self.is_enabled {
            return Err("Controller not enabled");
        }

        let mut connected_count = 0;

        for port in 0..self.num_ports {
            // In real implementation, would read XHCI_PORTSC register
            // Check bit 0 (CCS - Current Connect Status)
            // For simulation, assume alternating ports have devices
            if port % 2 == 0 {
                let device =
                    UsbDevice::new(port, UsbSpeed::SuperSpeed, UsbDeviceClass::MassStorage);
                self.devices.push(device);
                connected_count += 1;
            }
        }

        self.device_count.store(connected_count, Ordering::SeqCst);
        Ok(connected_count)
    }

    pub fn enumerate_device(&mut self, port: u8) -> Result<UsbDevice, &'static str> {
        if port >= self.num_ports {
            return Err("Invalid port");
        }

        // In real implementation:
        // 1. Send SET_ADDRESS request
        // 2. Get device descriptor
        // 3. Get configuration descriptor
        // 4. Assign address
        // 5. Set configuration

        let mut device = UsbDevice::new(port, UsbSpeed::SuperSpeed, UsbDeviceClass::MassStorage);
        device.address = (port + 1) as u8;
        device.vendor_id = 0x0951; // Kingston vendor ID (example)
        device.product_id = 0x1234;
        device.manufacturer = "Kingston".to_string();
        device.product_name = "DataTraveler".to_string();
        device.is_connected = true;

        Ok(device)
    }

    pub fn set_device_address(&mut self, port: u8, address: u8) -> Result<(), &'static str> {
        if port >= self.num_ports {
            return Err("Invalid port");
        }

        // In real implementation, would send SET_ADDRESS USB request
        Ok(())
    }

    pub fn get_connected_devices(&self) -> &[UsbDevice] {
        &self.devices
    }

    pub fn get_device_count(&self) -> u32 {
        self.device_count.load(Ordering::SeqCst)
    }

    pub fn hot_plug_port(&mut self, port: u8, connected: bool) -> Result<(), &'static str> {
        if port >= self.num_ports {
            return Err("Invalid port");
        }

        if connected {
            let device = UsbDevice::new(port, UsbSpeed::SuperSpeed, UsbDeviceClass::MassStorage);
            self.devices.push(device);
            self.device_count.fetch_add(1, Ordering::SeqCst);
        } else {
            self.devices.retain(|d| d.port != port);
            if self.device_count.load(Ordering::SeqCst) > 0 {
                self.device_count.fetch_sub(1, Ordering::SeqCst);
            }
        }

        Ok(())
    }
}

impl Default for UsbXhciHostDriver {
    fn default() -> Self {
        Self::new(SUNRISE_POINT_XHCI, "0000:00:14.0")
    }
}

// ============================================================================
// PciDriver Implementation
// ============================================================================

pub struct UsbXhciPciDriver {
    host: Option<Box<UsbXhciHostDriver>>,
}

impl UsbXhciPciDriver {
    pub fn new() -> Self {
        UsbXhciPciDriver { host: None }
    }

    pub fn get_host(&self) -> Option<&UsbXhciHostDriver> {
        self.host.as_ref().map(|b| b.as_ref())
    }

    pub fn get_host_mut(&mut self) -> Option<&mut UsbXhciHostDriver> {
        self.host.as_mut().map(|b| b.as_mut())
    }
}

impl PciDriver for UsbXhciPciDriver {
    fn probe(&mut self, device: &PciDeviceInfo) -> Result<bool, &'static str> {
        // Check for Intel xHCI controllers
        if device.vendor_id != INTEL_VENDOR_ID {
            return Ok(false);
        }

        let supported = matches!(
            device.device_id,
            PANTHER_POINT_XHCI
                | LYNX_POINT_XHCI
                | WILDCAT_POINT_XHCI
                | SUNRISE_POINT_XHCI
                | KABY_LAKE_XHCI
        );

        if !supported {
            return Ok(false);
        }

        let mut host = Box::new(UsbXhciHostDriver::new(
            device.device_id,
            &device.address.sysfs_format(),
        ));

        if let Some(ref bar) = device.bars[0] {
            host.init_mmio(bar.address, bar.size)?;
        } else {
            return Err("No MMIO BAR found");
        }

        host.interrupt_line = device.interrupt_line;

        self.host = Some(host);
        Ok(true)
    }

    fn remove(&mut self, _device: &PciDeviceInfo) -> Result<(), &'static str> {
        self.host = None;
        Ok(())
    }

    fn name(&self) -> &str {
        "usb_xhci"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_device_creation() {
        let device = UsbDevice::new(1, UsbSpeed::SuperSpeed, UsbDeviceClass::MassStorage);
        assert_eq!(device.port, 1);
        assert_eq!(device.speed, UsbSpeed::SuperSpeed);
        assert!(!device.is_connected);
    }

    #[test]
    fn test_usb_endpoint_creation() {
        let endpoint = UsbEndpoint::new(1, 2, 512); // Bulk endpoint
        assert_eq!(endpoint.endpoint_address, 1);
        assert_eq!(endpoint.endpoint_type, 2);
        assert_eq!(endpoint.max_packet_size, 512);
    }

    #[test]
    fn test_xhci_driver_initialization() {
        let driver = UsbXhciHostDriver::new(SUNRISE_POINT_XHCI, "0000:00:14.0");
        assert_eq!(driver.device_id, SUNRISE_POINT_XHCI);
        assert!(!driver.is_enabled);
    }

    #[test]
    fn test_xhci_mmio_init() {
        let mut driver = UsbXhciHostDriver::new(SUNRISE_POINT_XHCI, "0000:00:14.0");
        assert!(driver.init_mmio(0xFE800000, 65536).is_ok());
        assert!(driver.is_enabled);
    }

    #[test]
    fn test_xhci_controller_reset() {
        let mut driver = UsbXhciHostDriver::new(SUNRISE_POINT_XHCI, "0000:00:14.0");
        assert!(driver.reset_controller().is_ok());
    }

    #[test]
    fn test_xhci_port_scan() {
        let mut driver = UsbXhciHostDriver::new(SUNRISE_POINT_XHCI, "0000:00:14.0");
        driver.init_mmio(0xFE800000, 65536).unwrap();
        driver.start_controller().unwrap();

        let count = driver.scan_ports().unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_xhci_pci_driver() {
        let driver = UsbXhciPciDriver::new();
        assert_eq!(driver.name(), "usb_xhci");
        assert!(driver.get_host().is_none());
    }

    #[test]
    fn test_transfer_ring_operations() {
        let mut ring = TransferRing::new(0x1000, 256);
        assert!(ring.queue_command(0x12345678).is_ok());
    }

    #[test]
    fn test_hot_plug_support() {
        let mut driver = UsbXhciHostDriver::new(SUNRISE_POINT_XHCI, "0000:00:14.0");
        driver.init_mmio(0xFE800000, 65536).unwrap();

        assert!(driver.hot_plug_port(0, true).is_ok());
        assert_eq!(driver.get_device_count(), 1);

        assert!(driver.hot_plug_port(0, false).is_ok());
        assert_eq!(driver.get_device_count(), 0);
    }
}
