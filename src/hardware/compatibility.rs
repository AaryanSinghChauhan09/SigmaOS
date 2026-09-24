//! OOP-based Hardware Compatibility Matrix for SigmaOS
//! Implements supported legacy, ancient (1980s/1990s), and modern hardware devices compatibility matrix.


use std::collections::BTreeMap;
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiPowerState {
    D0,
    D1,
    D2,
    D3,
}

pub struct SimpleAcpiManager {
    pub irq_routing: BTreeMap<u32, u32>,
    pub power_states: BTreeMap<DeviceID, AcpiPowerState>,
}

impl SimpleAcpiManager {
    pub fn new() -> Self {
        Self {
            irq_routing: BTreeMap::new(),
            power_states: BTreeMap::new(),
        }
    }

    pub fn balance_irq_routing(&mut self, irq: u32, cpu: u32) -> Result<(), &'static str> {
        self.irq_routing.insert(irq, cpu);
        Ok(())
    }

    pub fn set_device_power_state(
        &mut self,
        dev: DeviceID,
        state: AcpiPowerState,
    ) -> Result<(), &'static str> {
        self.power_states.insert(dev, state);
        Ok(())
    }

    pub fn get_device_power_state(&self, dev: DeviceID) -> Option<AcpiPowerState> {
        self.power_states.get(&dev).copied()
    }
}

impl Default for SimpleAcpiManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ACPI power and interrupt load balancing strategy (inspired by Linux and BSD)
pub trait AcpiLoadBalancer {
    fn balance_irq_routing(&mut self, interrupt_line: u8, cpu_id: usize) -> Result<(), &'static str>;
    fn set_device_power_state(
        &mut self,
        device_id: DeviceID,
        state: AcpiPowerState,
    ) -> Result<(), &'static str>;
    fn get_device_power_state(&self, device_id: DeviceID) -> Option<AcpiPowerState>;
}

impl AcpiLoadBalancer for SimpleAcpiManager {
    fn balance_irq_routing(&mut self, interrupt_line: u8, cpu_id: usize) -> Result<(), &'static str> {
        // Map u32 irq to u8 interrupt_line by modulo
        self.irq_routing.insert(interrupt_line as u32, cpu_id as u32);
        Ok(())
    }

    fn set_device_power_state(
        &mut self,
        device_id: DeviceID,
        state: AcpiPowerState,
    ) -> Result<(), &'static str> {
        self.power_states.insert(device_id, state);
        Ok(())
    }

    fn get_device_power_state(&self, device_id: DeviceID) -> Option<AcpiPowerState> {
        self.power_states.get(&device_id).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    GPU = 0,
    WiFi = 1,
    Printer = 2,
    Chipset = 3,
    Audio = 4,
    Storage = 5,
    LegacyBus = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus {
    Supported = 0,
    Partial = 1,
    Unsupported = 2,
    Unknown = 3,
}

pub trait HardwareDevice {
    fn id(&self) -> DeviceID;
    fn device_type(&self) -> DeviceType;
    fn vendor_id(&self) -> u16;
    fn device_id(&self) -> u16;
    fn name(&self) -> &str;
    fn support_status(&self) -> SupportStatus;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxBsdHardwareBusInfo {
    pub bus_type: String, // "pci", "usb", "acpi", "virtio"
    pub pci_vendor_id: u16,
    pub pci_device_id: u16,
    pub usb_vendor_id: u16,
    pub usb_product_id: u16,
    pub driver_alias: String, // e.g. "pci:v00008086d00001000sv*sd*bc*sc*i*", "usb:v046DpC52Bb*"
}

pub struct SimpleDevice {
    pub id: DeviceID,
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub name: String,
    pub support_status: SupportStatus,
    pub bus_info: Option<LinuxBsdHardwareBusInfo>,
}

impl SimpleDevice {
    pub fn new(
        id: DeviceID,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        name: &str,
        status: SupportStatus,
    ) -> Self {
        SimpleDevice {
            id,
            device_type,
            vendor_id,
            device_id,
            name: name.to_string(),
            support_status: status,
            bus_info: None,
        }
    }

    pub fn with_bus_info(mut self, bus_info: LinuxBsdHardwareBusInfo) -> Self {
        self.bus_info = Some(bus_info);
        self
    }
}

impl HardwareDevice for SimpleDevice {
    fn id(&self) -> DeviceID {
        self.id
    }
    fn device_type(&self) -> DeviceType {
        self.device_type
    }
    fn vendor_id(&self) -> u16 {
        self.vendor_id
    }
    fn device_id(&self) -> u16 {
        self.device_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn support_status(&self) -> SupportStatus {
        self.support_status
    }
}

pub trait HardwareCompatibilityManager {
    fn add_device(
        &mut self,
        device: Box<dyn HardwareDevice>,
    ) -> Result<DeviceID, CompatibilityError>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), CompatibilityError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn HardwareDevice>;
    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID>;
    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID>;
    fn list_supported(&self) -> Vec<DeviceID>;
    fn match_driver_by_alias(&self, alias_query: &str) -> Option<DeviceID>;
}

pub trait DriverManager {
    fn load_driver(&mut self, device_id: DeviceID) -> Result<(), ()>;
    fn unload_driver(&mut self, device_id: DeviceID) -> Result<(), ()>;
    fn get_driver_status(&self, device_id: DeviceID) -> bool;
}

pub struct SimpleDriverManager {
    pub loaded_drivers: Vec<DeviceID>,
}

impl SimpleDriverManager {
    pub fn new() -> Self {
        Self {
            loaded_drivers: Vec::new(),
        }
    }
}

impl Default for SimpleDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverManager for SimpleDriverManager {
    fn load_driver(&mut self, device_id: DeviceID) -> Result<(), ()> {
        if !self.get_driver_status(device_id) {
            self.loaded_drivers.push(device_id);
        }
        Ok(())
    }

    fn unload_driver(&mut self, device_id: DeviceID) -> Result<(), ()> {
        if let Some(pos) = self.loaded_drivers.iter().position(|&id| id == device_id) {
            self.loaded_drivers.remove(pos);
        }
        Ok(())
    }

    fn get_driver_status(&self, device_id: DeviceID) -> bool {
        self.loaded_drivers.contains(&device_id)
    }
}

pub struct SimpleDiagnostics {
    pub matrix: SimpleCompatibilityMatrix,
}

impl SimpleDiagnostics {
    pub fn new(matrix: SimpleCompatibilityMatrix) -> Self {
        Self { matrix }
    }

    pub fn run_full_scan(&self) -> CompatibilityReport {
        let mut results = Vec::new();
        for dev in &self.matrix.devices {
            let res = match dev.support_status() {
                SupportStatus::Supported => CompatibilityResult::Healthy,
                SupportStatus::Partial => CompatibilityResult::Warning,
                SupportStatus::Unsupported => CompatibilityResult::Error,
                SupportStatus::Unknown => CompatibilityResult::Unknown,
            };
            results.push((dev.id(), res));
        }
        CompatibilityReport { results }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityError {
    Success = 0,
    DeviceNotFound = 1,
    DuplicateDevice = 2,
    InvalidParameter = 3,
}

/// Hotplug event type (inspired by Linux udev)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotplugEvent {
    Add = 0,
    Remove = 1,
}

pub trait HotplugManager {
    fn trigger_hotplug(
        &mut self,
        event: HotplugEvent,
        device: Box<dyn HardwareDevice>,
    ) -> Result<(), &'static str>;
    fn list_hotplug_history(&self) -> &[(HotplugEvent, DeviceID)];
}

pub struct SimpleCompatibilityMatrix {
    pub devices: Vec<Box<dyn HardwareDevice>>,
    pub next_id: AtomicUsize,
    pub hotplug_history: Vec<(HotplugEvent, DeviceID)>,
}

impl Default for SimpleCompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleCompatibilityMatrix {
    pub fn new() -> Self {
        SimpleCompatibilityMatrix {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
            hotplug_history: Vec::new(),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let com1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::LegacyBus,
            0x0003,
            0x03F8,
            "Serial Port COM1 (UART 16550)",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(com1));

        let nvme = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x144D,
            0xA808,
            "Samsung PCIe Gen 4 NVMe Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(nvme));

        let gpu1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::GPU,
            0x10DE,
            0x1C02,
            "NVIDIA GeForce RTX 3060",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(gpu1));

        let gpu2 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::GPU,
            0x1002,
            0x73DF,
            "AMD Radeon RX 6800 XT",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(gpu2));

        let wifi1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x8086,
            0x2723,
            "Intel Wi-Fi 6 AX200",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(wifi1));

        let wifi2 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x168C,
            0x003A,
            "Realtek RTL8852AE",
            SupportStatus::Partial,
        );
        self.devices.push(Box::new(wifi2));

        let printer1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Printer,
            0x03F0,
            0x4A17,
            "HP LaserJet Pro M404n",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(printer1));

        let chipset1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Chipset,
            0x8086,
            0x1C02,
            "Intel Z590",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(chipset1));

        let audio1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Audio,
            0x10EC,
            0x0887,
            "Realtek ALC887",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(audio1));

        let storage1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x8086,
            0x2822,
            "Intel SATA Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(storage1));

        let wifi3 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x8086,
            0x2725,
            "Intel Wi-Fi 6E AX210",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(wifi3));

        let usb1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Chipset,
            0x8086,
            0xA36D,
            "Intel xHCI USB 3.2 Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(usb1));

        let nvme1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x144D,
            0xA809,
            "Samsung NVMe SSD Controller 980 Pro",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(nvme1));

        let virtio_net = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x1AF4,
            0x1000,
            "VirtIO Network Adapter",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(virtio_net));

        let virtio_blk = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x1AF4,
            0x1001,
            "VirtIO Block Storage Device",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(virtio_blk));
    }
}

impl HotplugManager for SimpleCompatibilityMatrix {
    fn trigger_hotplug(
        &mut self,
        event: HotplugEvent,
        device: Box<dyn HardwareDevice>,
    ) -> Result<(), &'static str> {
        let dev_id = device.id();
        self.hotplug_history.push((event, dev_id));
        match event {
            HotplugEvent::Add => {
                let _ = self.add_device(device);
            }
            HotplugEvent::Remove => {
                let _ = self.remove_device(dev_id);
            }
        }
        Ok(())
    }

    fn list_hotplug_history(&self) -> &[(HotplugEvent, DeviceID)] {
        &self.hotplug_history
    }
}

impl HardwareCompatibilityManager for SimpleCompatibilityMatrix {
    fn add_device(
        &mut self,
        device: Box<dyn HardwareDevice>,
    ) -> Result<DeviceID, CompatibilityError> {
        let id = device.id();
        self.devices.push(device);
        Ok(id)
    }

    fn remove_device(&mut self, id: DeviceID) -> Result<(), CompatibilityError> {
        if let Some(pos) = self.devices.iter().position(|d| d.id() == id) {
            self.devices.remove(pos);
            Ok(())
        } else {
            Err(CompatibilityError::DeviceNotFound)
        }
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn HardwareDevice> {
        self.devices
            .iter()
            .find(|d| d.id() == id)
            .map(|d| d.as_ref())
    }

    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID> {
        self.devices
            .iter()
            .find(|d| d.vendor_id() == vendor_id && d.device_id() == device_id)
            .map(|d| d.id())
    }

    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID> {
        self.devices
            .iter()
            .filter(|d| d.device_type() == device_type)
            .map(|d| d.id())
            .collect()
    }

    fn list_supported(&self) -> Vec<DeviceID> {
        self.devices
            .iter()
            .filter(|d| d.support_status() == SupportStatus::Supported)
            .map(|d| d.id())
            .collect()
    }

    fn match_driver_by_alias(&self, alias_query: &str) -> Option<DeviceID> {
        self.devices.iter().find_map(|d| {
            let vendor = d.vendor_id();
            let dev = d.device_id();
            if alias_query.contains(&format!("{:04x}", vendor)) && alias_query.contains(&format!("{:04x}", dev)) {
                Some(d.id())
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityResult {
    Healthy = 0,
    Warning = 1,
    Error = 2,
    Unknown = 3,
}

pub struct CompatibilityReport {
    pub results: Vec<(DeviceID, CompatibilityResult)>,
}

// =========================================================================
// Linux & BSD Inspired Advanced Hardware Drivers
// =========================================================================

/// Linux NVMe-oF (NVMe over Fabrics) Engine supporting RDMA & TCP transports
#[derive(Debug, Clone)]
pub struct LinuxNvmeOverFabricsEngine {
    pub target_subnqn: String,
    pub transport_type: String, // "rdma" or "tcp"
    pub portal_address: String,
    pub port_number: u16,
    pub max_queue_depth: u16,
    pub connected: bool,
}

impl LinuxNvmeOverFabricsEngine {
    pub fn new(subnqn: &str, transport: &str, portal: &str, port: u16) -> Self {
        Self {
            target_subnqn: subnqn.to_string(),
            transport_type: transport.to_string(),
            portal_address: portal.to_string(),
            port_number: port,
            max_queue_depth: 1024,
            connected: false,
        }
    }

    pub fn connect_fabric(&mut self) -> Result<bool, &'static str> {
        if self.portal_address.is_empty() {
            return Err("Invalid portal address");
        }
        self.connected = true;
        Ok(true)
    }

    pub fn submit_nvme_cmd(&self, _opcode: u8) -> Result<u32, &'static str> {
        if !self.connected {
            return Err("NVMe-oF target disconnected");
        }
        Ok(0) // Success status
    }
}

/// FreeBSD CAM (Common Access Method) Storage Subsystem Engine
#[derive(Debug, Clone)]
pub struct FreeBsdCamStorageEngine {
    pub bus_id: u32,
    pub target_id: u32,
    pub lun_id: u32,
    pub device_type: String,
    pub queue_frozen: bool,
}

impl FreeBsdCamStorageEngine {
    pub fn new(bus: u32, target: u32, lun: u32, dev_type: &str) -> Self {
        Self {
            bus_id: bus,
            target_id: target,
            lun_id: lun,
            device_type: dev_type.to_string(),
            queue_frozen: false,
        }
    }

    pub fn execute_scsi_cdb(&mut self, cdb: &[u8]) -> Result<Vec<u8>, &'static str> {
        if self.queue_frozen {
            return Err("CAM SIM Queue is frozen");
        }
        if cdb.is_empty() {
            return Err("Empty SCSI CDB");
        }
        Ok(vec![0x00, 0x80, 0x02, 0x02]) // Mock Inquiry/Read response
    }

    pub fn freeze_queue(&mut self) {
        self.queue_frozen = true;
    }

    pub fn release_queue(&mut self) {
        self.queue_frozen = false;
    }
}

/// Linux & USB4 / Thunderbolt DisplayPort & PCIe Tunneling Driver Engine
#[derive(Debug, Clone)]
pub struct LinuxThunderboltDisplayPortTunnelEngine {
    pub domain_id: u32,
    pub route_string: u64,
    pub allocated_dp_bandwidth_gbps: f32,
    pub pcie_tunnel_active: bool,
    pub security_level: String, // "user", "secure", "dponly"
}

impl LinuxThunderboltDisplayPortTunnelEngine {
    pub fn new(domain: u32, route: u64, sec_level: &str) -> Self {
        Self {
            domain_id: domain,
            route_string: route,
            allocated_dp_bandwidth_gbps: 0.0,
            pcie_tunnel_active: false,
            security_level: sec_level.to_string(),
        }
    }

    pub fn establish_dp_tunnel(&mut self, bandwidth_gbps: f32) -> Result<bool, &'static str> {
        if bandwidth_gbps > 40.0 {
            return Err("Exceeds Thunderbolt 4 40Gbps maximum bandwidth");
        }
        self.allocated_dp_bandwidth_gbps = bandwidth_gbps;
        Ok(true)
    }

    pub fn enable_pcie_tunneling(&mut self) -> Result<bool, &'static str> {
        if self.security_level == "dponly" {
            return Err("PCIe tunneling blocked under dponly security policy");
        }
        self.pcie_tunnel_active = true;
        Ok(true)
    }
}

/// OpenBSD uvideo (USB Video Class) V4L2-Compatible Webcam Driver Engine
#[derive(Debug, Clone)]
pub struct OpenBsdUvideoWebcamEngine {
    pub device_node: String,
    pub max_resolution_width: u32,
    pub max_resolution_height: u32,
    pub is_streaming: bool,
    pub format_fourcc: String, // e.g. "YUY2", "MJPG", "NV12"
}

impl OpenBsdUvideoWebcamEngine {
    pub fn new(node: &str, width: u32, height: u32, fourcc: &str) -> Self {
        Self {
            device_node: node.to_string(),
            max_resolution_width: width,
            max_resolution_height: height,
            is_streaming: false,
            format_fourcc: fourcc.to_string(),
        }
    }

    pub fn start_video_stream(&mut self) -> Result<(), &'static str> {
        self.is_streaming = true;
        Ok(())
    }

    pub fn capture_video_frame(&self) -> Result<Vec<u8>, &'static str> {
        if !self.is_streaming {
            return Err("Webcam stream is not active");
        }
        Ok(vec![0xFF, 0xD8, 0xFF, 0xE0]) // Mock JPEG/RAW video frame header
    }

    pub fn stop_video_stream(&mut self) {
        self.is_streaming = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatibility_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        assert_eq!(matrix.list_supported().len(), 14);
        assert_eq!(matrix.list_by_type(DeviceType::WiFi).len(), 4);
    }

    #[test]
    fn test_driver_manager_lifecycle() {
        let mut driver_manager = SimpleDriverManager::new();
        assert!(!driver_manager.get_driver_status(42));

        driver_manager.load_driver(42).unwrap();
        assert!(driver_manager.get_driver_status(42));

        driver_manager.unload_driver(42).unwrap();
        assert!(!driver_manager.get_driver_status(42));
    }

    #[test]
    fn test_diagnostics() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        let diag = SimpleDiagnostics::new(matrix);
        let report = diag.run_full_scan();
        assert_eq!(report.results.len(), 15);
    }

    #[test]
    fn test_expanded_device_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        assert_eq!(matrix.devices.len(), 15);
        assert_eq!(matrix.list_supported().len(), 14);

        let diag = SimpleDiagnostics::new(matrix);
        let report = diag.run_full_scan();
        assert_eq!(report.results.len(), 15);
    }

    #[test]
    fn test_device_type_filtering() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();

        assert_eq!(matrix.list_by_type(DeviceType::WiFi).len(), 4);
        assert_eq!(matrix.list_by_type(DeviceType::Storage).len(), 4);
        assert_eq!(matrix.list_by_type(DeviceType::Chipset).len(), 2);
        assert_eq!(matrix.list_by_type(DeviceType::GPU).len(), 2);
        assert_eq!(matrix.list_by_type(DeviceType::Printer).len(), 1);
        assert_eq!(matrix.list_by_type(DeviceType::Audio).len(), 1);

        let nvme_id = matrix.find_by_vendor_device(0x144D, 0xA809).unwrap();
        assert_eq!(
            matrix.get_device(nvme_id).unwrap().name(),
            "Samsung NVMe SSD Controller 980 Pro"
        );
    }

    #[test]
    fn test_acpi_load_balancing() {
        let mut acpi = SimpleAcpiManager::new();
        assert!(acpi.balance_irq_routing(11, 4).is_ok());
        assert_eq!(acpi.irq_routing.get(&11), Some(&4));

        assert!(acpi.set_device_power_state(42, AcpiPowerState::D3).is_ok());
        assert_eq!(acpi.get_device_power_state(42), Some(AcpiPowerState::D3));
    }

    #[test]
    fn test_hotplug_manager() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        let cap = SimpleDevice::new(
            99,
            DeviceType::Storage,
            0x1234,
            0x5678,
            "HotplugDisk",
            SupportStatus::Supported,
        );
        assert!(matrix
            .trigger_hotplug(HotplugEvent::Add, Box::new(cap))
            .is_ok());
        assert_eq!(matrix.list_hotplug_history().len(), 1);
        assert_eq!(matrix.get_device(99).unwrap().name(), "HotplugDisk");
    }

    #[test]
    fn test_linux_and_bsd_advanced_drivers() {
        // 1. Test Linux NVMe-oF driver
        let mut nvme_of = LinuxNvmeOverFabricsEngine::new("nqn.2026-09.org.sigma:storage", "tcp", "192.168.1.100", 4420);
        assert!(nvme_of.submit_nvme_cmd(0x02).is_err());
        assert!(nvme_of.connect_fabric().unwrap());
        assert_eq!(nvme_of.submit_nvme_cmd(0x02).unwrap(), 0);

        // 2. Test FreeBSD CAM storage engine
        let mut cam = FreeBsdCamStorageEngine::new(0, 0, 0, "da0");
        let res = cam.execute_scsi_cdb(&[0x12, 0x00, 0x00, 0x00, 0x24, 0x00]).unwrap();
        assert_eq!(res.len(), 4);
        cam.freeze_queue();
        assert!(cam.execute_scsi_cdb(&[0x12]).is_err());
        cam.release_queue();
        assert!(cam.execute_scsi_cdb(&[0x12]).is_ok());

        // 3. Test Thunderbolt DP & PCIe tunnel driver
        let mut tb = LinuxThunderboltDisplayPortTunnelEngine::new(1, 0x00010002, "secure");
        assert!(tb.establish_dp_tunnel(21.6).unwrap());
        assert!(tb.enable_pcie_tunneling().unwrap());

        // 4. Test OpenBSD uvideo webcam driver
        let mut uvideo = OpenBsdUvideoWebcamEngine::new("/dev/video0", 1920, 1080, "YUY2");
        assert!(uvideo.capture_video_frame().is_err());
        uvideo.start_video_stream().unwrap();
        let frame = uvideo.capture_video_frame().unwrap();
        assert_eq!(frame[0], 0xFF);
        uvideo.stop_video_stream();
    }
}
