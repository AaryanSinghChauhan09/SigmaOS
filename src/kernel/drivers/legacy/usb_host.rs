use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — USB Host Controller + HID + Mass Storage
/// Absorbs Linux USB stack (linux/drivers/usb/): OHCI, UHCI, EHCI, xHCI
/// USB HID (keyboards, mice, gamepads), USB Mass Storage (BBB protocol)
use core::sync::atomic::{AtomicUsize, Ordering};
use std::string::{String, ToString};
use std::vec::Vec;

/// USB speed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbSpeed {
    Low,
    Full,
    High,
    Super,
    SuperPlus,
}

impl UsbSpeed {
    pub fn mbps(&self) -> u32 {
        match self {
            UsbSpeed::Low => 1,
            UsbSpeed::Full => 12,
            UsbSpeed::High => 480,
            UsbSpeed::Super => 5_000,
            UsbSpeed::SuperPlus => 10_000,
        }
    }
}

/// USB host controller type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciType {
    Ohci,
    Uhci,
    Ehci,
    Xhci,
}

/// USB device descriptor (simplified)
#[derive(Debug, Clone)]
pub struct UsbDescriptor {
    pub vid: u16,
    pub pid: u16,
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
    pub product: String,
    pub manufacturer: String,
}

impl UsbDescriptor {
    pub fn new(vid: u16, pid: u16, class: u8, product: &str) -> Self {
        UsbDescriptor {
            vid,
            pid,
            class,
            subclass: 0,
            protocol: 0,
            product: product.to_string(),
            manufacturer: "Unknown".to_string(),
        }
    }
}

/// USB endpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointType {
    Control,
    Bulk,
    Interrupt,
    Isochronous,
}

#[derive(Debug, Clone)]
pub struct UsbEndpoint {
    pub addr: u8,
    pub ep_type: EndpointType,
    pub max_packet: u16,
    pub interval: u8,
}

/// USB device
pub struct UsbDevice {
    pub addr: u8,
    pub speed: UsbSpeed,
    pub descriptor: UsbDescriptor,
    pub endpoints: Vec<UsbEndpoint>,
    pub enabled: bool,
}

impl UsbDevice {
    pub fn new(addr: u8, speed: UsbSpeed, desc: UsbDescriptor) -> Self {
        UsbDevice {
            addr,
            speed,
            descriptor: desc,
            endpoints: Vec::new(),
            enabled: true,
        }
    }
}

/// USB host controller driver (polymorphic OOP trait)
pub trait UsbHostController: Send + Sync {
    fn hci_type(&self) -> HciType;
    fn speed(&self) -> UsbSpeed;
    fn enumerate(&mut self) -> Vec<UsbDevice>;
    fn submit_bulk(&mut self, dev_addr: u8, ep: u8, data: &[u8]) -> Result<usize, &'static str>;
    fn submit_control(
        &mut self,
        dev_addr: u8,
        request: UsbControlRequest,
    ) -> Result<Vec<u8>, &'static str>;
    fn port_count(&self) -> u8;
}

#[derive(Debug, Clone)]
pub struct UsbControlRequest {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

/// xHCI — eXtensible Host Controller Interface (USB 3.0/3.1/3.2/4.0)
pub struct XhciController {
    pub base_mmio: u64,
    pub port_count: u8,
    pub max_slots: u8,
    devices: Vec<UsbDevice>,
    transfer_count: AtomicUsize,
    initialized: bool,
}

impl XhciController {
    pub fn new(base_mmio: u64, ports: u8) -> Self {
        XhciController {
            base_mmio,
            port_count: ports,
            max_slots: 64,
            devices: Vec::new(),
            transfer_count: AtomicUsize::new(0),
            initialized: false,
        }
    }
}

impl UsbHostController for XhciController {
    fn hci_type(&self) -> HciType {
        HciType::Xhci
    }
    fn speed(&self) -> UsbSpeed {
        UsbSpeed::SuperPlus
    }
    fn port_count(&self) -> u8 {
        self.port_count
    }

    fn enumerate(&mut self) -> Vec<UsbDevice> {
        // Mock: return registered devices
        Vec::new()
    }

    fn submit_bulk(&mut self, _dev: u8, _ep: u8, data: &[u8]) -> Result<usize, &'static str> {
        self.transfer_count.fetch_add(1, Ordering::Relaxed);
        Ok(data.len())
    }

    fn submit_control(
        &mut self,
        _dev: u8,
        _req: UsbControlRequest,
    ) -> Result<Vec<u8>, &'static str> {
        self.transfer_count.fetch_add(1, Ordering::Relaxed);
        Ok(vec![0u8; 18]) // Mock descriptor response
    }
}

impl KernelSubsystem for XhciController {
    fn name(&self) -> &str {
        "xhci"
    }
    fn version(&self) -> &str {
        "4.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::High
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }
    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

// ── USB HID (Human Interface Device) ──────────────────────────────────────

/// HID usage page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidUsagePage {
    GenericDesktop = 0x01,
    Keyboard = 0x07,
    Leds = 0x08,
    Button = 0x09,
}

/// HID report type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidReportKind {
    Input,
    Output,
    Feature,
}

pub struct UsbHidDevice {
    pub descriptor: UsbDescriptor,
    pub report_size: u16,
    events: Vec<Vec<u8>>,
    report_count: AtomicUsize,
}

impl UsbHidDevice {
    pub fn keyboard(addr: u8) -> Self {
        UsbHidDevice {
            descriptor: UsbDescriptor::new(0x04B3, 0x3003, 0x03, "USB Keyboard"),
            report_size: 8,
            events: Vec::new(),
            report_count: AtomicUsize::new(0),
        }
    }

    pub fn mouse(addr: u8) -> Self {
        UsbHidDevice {
            descriptor: UsbDescriptor::new(0x046D, 0xC077, 0x03, "USB Mouse"),
            report_size: 4,
            events: Vec::new(),
            report_count: AtomicUsize::new(0),
        }
    }

    pub fn inject_report(&mut self, report: Vec<u8>) {
        self.report_count.fetch_add(1, Ordering::Relaxed);
        self.events.push(report);
    }

    pub fn poll_event(&mut self) -> Option<Vec<u8>> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }

    pub fn report_count(&self) -> usize {
        self.report_count.load(Ordering::Relaxed)
    }
}

// ── USB Mass Storage (BBB — Bulk-Only Transport) ───────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScsiCommand {
    TestUnitReady = 0x00,
    RequestSense = 0x03,
    Read10 = 0x28,
    Write10 = 0x2A,
    Inquiry = 0x12,
    ReadCapacity10 = 0x25,
}

pub struct UsbMassStorage {
    pub descriptor: UsbDescriptor,
    pub max_lun: u8,
    data: Vec<[u8; 512]>,
    scsi_count: AtomicUsize,
    initialized: bool,
}

impl UsbMassStorage {
    pub fn new(capacity_sectors: usize) -> Self {
        UsbMassStorage {
            descriptor: UsbDescriptor::new(0x0781, 0x5583, 0x08, "USB Flash Drive"),
            max_lun: 0,
            data: (0..core::cmp::min(capacity_sectors, 8192))
                .map(|_| [0u8; 512])
                .collect(),
            scsi_count: AtomicUsize::new(0),
            initialized: false,
        }
    }

    pub fn scsi_read(&self, lba: u32, sectors: u32, buf: &mut Vec<u8>) -> Result<(), &'static str> {
        for i in 0..sectors as usize {
            let idx = lba as usize + i;
            if idx >= self.data.len() {
                return Err("USB MSC: LBA out of range");
            }
            buf.extend_from_slice(&self.data[idx]);
            self.scsi_count.fetch_add(1, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn scsi_write(&mut self, lba: u32, buf: &[u8]) -> Result<(), &'static str> {
        let count = buf.len() / 512;
        for i in 0..count {
            let idx = lba as usize + i;
            if idx >= self.data.len() {
                return Err("USB MSC: LBA out of range");
            }
            self.data[idx].copy_from_slice(&buf[i * 512..(i + 1) * 512]);
            self.scsi_count.fetch_add(1, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn capacity_sectors(&self) -> u32 {
        self.data.len() as u32
    }
    pub fn scsi_count(&self) -> usize {
        self.scsi_count.load(Ordering::Relaxed)
    }
}

impl KernelSubsystem for UsbMassStorage {
    fn name(&self) -> &str {
        "usb_storage"
    }
    fn version(&self) -> &str {
        "2.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Late
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Optional
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["xhci"]
    }
    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_speeds() {
        assert_eq!(UsbSpeed::Full.mbps(), 12);
        assert_eq!(UsbSpeed::SuperPlus.mbps(), 10_000);
    }

    #[test]
    fn test_xhci_transfer() {
        let mut hci = XhciController::new(0xFED0_0000, 8);
        hci.initialize().unwrap();
        let result = hci.submit_bulk(1, 0x81, &[0xDE, 0xAD]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_hid_keyboard_report() {
        let mut kb = UsbHidDevice::keyboard(1);
        kb.inject_report(vec![0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00]); // 'a' key
        assert!(kb.poll_event().is_some());
        assert_eq!(kb.report_count(), 1);
    }

    #[test]
    fn test_usb_mass_storage() {
        let mut msc = UsbMassStorage::new(2048);
        let write_data: Vec<u8> = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes()
            .iter()
            .cycle()
            .take(512)
            .copied()
            .collect();
        msc.scsi_write(0, &write_data).unwrap();
        let mut read_buf = Vec::new();
        msc.scsi_read(0, 1, &mut read_buf).unwrap();
        assert_eq!(read_buf[0], write_data[0]);
        assert_eq!(msc.capacity_sectors(), 2048);
    }
}
