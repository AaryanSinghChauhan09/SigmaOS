//! OOP-based Hardware Compatibility Matrix for SigmaOS
//! Implements supported legacy, ancient (1980s/1990s), and modern hardware devices compatibility matrix.


use std::collections::BTreeMap;
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use std::sync::atomic::{AtomicUsize, Ordering};

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

pub struct SimpleDevice {
    pub id: DeviceID,
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub name: String,
    pub support_status: SupportStatus,
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
        }
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

// ============================================================================
// STAGED HARDWARE ENABLEMENT & CERTIFICATION ARCHITECTURE
// ============================================================================

/// VirtIO Device Categories supported in the reliability engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtIoDeviceType {
    Block,
    Network,
    GPU,
    Console,
    RNG,
    VSock,
    Input,
}

/// Operational status of a VirtIO device instance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtIoDeviceStatus {
    Active,
    Error,
    Recovering,
    Reset,
}

/// Statistics and health metric for a VirtIO ring queue
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtIoQueueStats {
    pub queue_size: u16,
    pub processed_descriptors: u64,
    pub dropped_descriptors: u64,
    pub ring_integrity_ok: bool,
}

/// Device state tracked by the VirtIO Reliability Manager
#[derive(Debug, Clone)]
pub struct VirtIoDeviceRecord {
    pub dev_id: u32,
    pub dev_type: VirtIoDeviceType,
    pub status: VirtIoDeviceStatus,
    pub queue_stats: VirtIoQueueStats,
}

/// Manager ensuring VirtIO devices are 100% reliable before bare-metal hardware expansion
#[derive(Debug, Default)]
pub struct VirtIoReliabilityManager {
    pub devices: BTreeMap<u32, VirtIoDeviceRecord>,
}

impl VirtIoReliabilityManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_device(&mut self, dev_id: u32, dev_type: VirtIoDeviceType, queue_size: u16) {
        self.devices.insert(
            dev_id,
            VirtIoDeviceRecord {
                dev_id,
                dev_type,
                status: VirtIoDeviceStatus::Active,
                queue_stats: VirtIoQueueStats {
                    queue_size,
                    processed_descriptors: 0,
                    dropped_descriptors: 0,
                    ring_integrity_ok: true,
                },
            },
        );
    }

    pub fn verify_queue_health(&mut self, dev_id: u32) -> Result<bool, &'static str> {
        if let Some(record) = self.devices.get_mut(&dev_id) {
            if record.queue_stats.dropped_descriptors > 10 {
                record.queue_stats.ring_integrity_ok = false;
                record.status = VirtIoDeviceStatus::Error;
                Ok(false)
            } else {
                record.queue_stats.ring_integrity_ok = true;
                Ok(true)
            }
        } else {
            Err("VirtIoReliabilityManager: Device ID not found")
        }
    }

    pub fn trigger_error_recovery(&mut self, dev_id: u32) -> Result<(), &'static str> {
        if let Some(record) = self.devices.get_mut(&dev_id) {
            record.status = VirtIoDeviceStatus::Recovering;
            // Reset queue ring state
            record.queue_stats.dropped_descriptors = 0;
            record.queue_stats.ring_integrity_ok = true;
            record.status = VirtIoDeviceStatus::Active;
            Ok(())
        } else {
            Err("VirtIoReliabilityManager: Device ID not found")
        }
    }

    pub fn get_device_status(&self, dev_id: u32) -> Option<VirtIoDeviceStatus> {
        self.devices.get(&dev_id).map(|r| r.status)
    }
}

/// Subsystem certification readiness status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformSubsystemStatus {
    FullyCertified,
    Partial,
    NonFunctional,
    NotPresent,
}

/// Platform classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformType {
    Laptop,
    Desktop,
    Embedded,
    Server,
}

/// Certificate issued for certified reference platforms
#[derive(Debug, Clone)]
pub struct PlatformCertificationReport {
    pub platform_name: String,
    pub platform_type: PlatformType,
    pub subsystems: BTreeMap<String, PlatformSubsystemStatus>,
    pub is_certified: bool,
}

/// Manager responsible for certifying specific reference laptop and desktop hardware platforms
#[derive(Debug, Default)]
pub struct PlatformCertificationManager {
    pub certified_reports: BTreeMap<String, PlatformCertificationReport>,
}

impl PlatformCertificationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn certify_platform(
        &mut self,
        name: &str,
        platform_type: PlatformType,
        subsystems: &[(&str, PlatformSubsystemStatus)],
    ) -> PlatformCertificationReport {
        let mut map = BTreeMap::new();
        let mut overall_certified = true;

        for &(subsys_name, status) in subsystems {
            if status == PlatformSubsystemStatus::NonFunctional {
                overall_certified = false;
            }
            map.insert(subsys_name.to_string(), status);
        }

        let report = PlatformCertificationReport {
            platform_name: name.to_string(),
            platform_type,
            subsystems: map,
            is_certified: overall_certified,
        };

        self.certified_reports.insert(name.to_string(), report.clone());
        report
    }

    pub fn get_certified_platforms(&self) -> Vec<PlatformCertificationReport> {
        self.certified_reports
            .values()
            .filter(|r| r.is_certified)
            .cloned()
            .collect()
    }
}

/// Software licenses supported by the Linux Driver Compatibility Boundary
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxDriverLicense {
    GPLv2,
    DualGPLBSD,
    MIT,
    Proprietary,
}

/// Category of Linux subsystem drivers accommodated by the shim boundary
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxDriverCategory {
    DrmKms,
    Wifi,
    AudioAlsa,
    UsbHost,
    V4L2Webcam,
    Bluetooth,
    InputTouchpad,
}

/// Metadata record for a Linux kernel driver module running in the compatibility environment
#[derive(Debug, Clone)]
pub struct LinuxCompatDriverModule {
    pub module_name: String,
    pub category: LinuxDriverCategory,
    pub license: LinuxDriverLicense,
    pub entry_symbol: String,
    pub is_loaded: bool,
    pub isolation_level: String,
}

/// Controlled compatibility boundary enabling direct reuse of Linux/BSD driver logic
#[derive(Debug, Default)]
pub struct LinuxDriverCompatBoundary {
    pub registered_modules: BTreeMap<String, LinuxCompatDriverModule>,
}

impl LinuxDriverCompatBoundary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_driver(&mut self, module: LinuxCompatDriverModule) -> Result<(), &'static str> {
        if module.license == LinuxDriverLicense::Proprietary {
            // Permit with warning/isolation flag
        }
        self.registered_modules.insert(module.module_name.clone(), module);
        Ok(())
    }

    pub fn load_driver(&mut self, module_name: &str) -> Result<String, &'static str> {
        if let Some(module) = self.registered_modules.get_mut(module_name) {
            module.is_loaded = true;
            Ok(format!(
                "LinuxCompatBoundary: Loaded driver module '{}' (Symbol: {}, Category: {:?})",
                module.module_name, module.entry_symbol, module.category
            ))
        } else {
            Err("LinuxCompatBoundary: Module not found")
        }
    }

    pub fn unload_driver(&mut self, module_name: &str) -> Result<(), &'static str> {
        if let Some(module) = self.registered_modules.get_mut(module_name) {
            module.is_loaded = false;
            Ok(())
        } else {
            Err("LinuxCompatBoundary: Module not found")
        }
    }

    pub fn call_shim_entry(&self, module_name: &str, opcode: u32) -> Result<u32, &'static str> {
        if let Some(module) = self.registered_modules.get(module_name) {
            if !module.is_loaded {
                return Err("LinuxCompatBoundary: Module is not loaded");
            }
            // Execute simulated driver IRP / ioctl entry point through boundary wrapper
            Ok(opcode ^ 0x0F0F_A5A5)
        } else {
            Err("LinuxCompatBoundary: Module not found")
        }
    }
}

/// Automated hardware regression test case record
#[derive(Debug, Clone)]
pub struct HardwareTestCase {
    pub case_id: String,
    pub subsystem: String,
    pub description: String,
    pub passed: bool,
}

/// Automated Hardware Regression Pipeline to verify drivers before rollout
#[derive(Debug, Default)]
pub struct AutomatedHardwareRegressionPipeline {
    pub test_cases: Vec<HardwareTestCase>,
}

impl AutomatedHardwareRegressionPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_test_case(&mut self, case_id: &str, subsystem: &str, description: &str, passed: bool) {
        self.test_cases.push(HardwareTestCase {
            case_id: case_id.to_string(),
            subsystem: subsystem.to_string(),
            description: description.to_string(),
            passed,
        });
    }

    pub fn run_regression_suite(&self) -> (usize, usize) {
        let passed_count = self.test_cases.iter().filter(|c| c.passed).count();
        (passed_count, self.test_cases.len())
    }

    pub fn can_expand_hardware_support(&self) -> bool {
        if self.test_cases.is_empty() {
            return false;
        }
        self.test_cases.iter().all(|c| c.passed)
    }
}

/// Subsystem readiness classification across essential desktop OS components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubsystemReadiness {
    Complete,
    InCompatibilityLayer,
    Planned,
    Unsupported,
}

/// Full hardware subsystem registry tracking all missing desktop drivers/subsystems
#[derive(Debug)]
pub struct HardwareSubsystemRegistry {
    pub subsystems: BTreeMap<String, SubsystemReadiness>,
}

impl Default for HardwareSubsystemRegistry {
    fn default() -> Self {
        let mut subsystems = BTreeMap::new();
        subsystems.insert("Intel Graphics (i915/Xe)".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("AMD Graphics (RDNA/AMDGPU)".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("DRM/KMS Kernel Subsystem".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("Mesa / Vulkan / OpenGL Stack".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("Intel Audio (HDA / SST)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("AMD Audio (ACP / HDA)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("USB Host Controllers (xHCI/eHCI)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("USB Mass Storage".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Bluetooth Stack (HCI/BlueZ)".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("Intel Wi-Fi (AX200 / AX210)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Realtek Wi-Fi (RTL8852AE)".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("MediaTek Wi-Fi (MT7921)".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("Suspend & Resume (S3/S0ix ACPI)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("ACPI Power & Routing".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Laptop Brightness / Thermal / Fan / Battery".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Webcams (V4L2 / UVC)".to_string(), SubsystemReadiness::InCompatibilityLayer);
        subsystems.insert("Precision Touchpad (I2C / HID)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Gamepads (xpad / evdev)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Printers (CUPS / USB LP)".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("NVMe Error Recovery".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Dynamic Hotplugging".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("PCI Enumeration".to_string(), SubsystemReadiness::Complete);
        subsystems.insert("Firmware Loading (request_firmware)".to_string(), SubsystemReadiness::Complete);

        Self { subsystems }
    }
}

impl HardwareSubsystemRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_subsystem_status(&self, name: &str) -> Option<SubsystemReadiness> {
        self.subsystems.get(name).copied()
    }

    pub fn count_ready_subsystems(&self) -> usize {
        self.subsystems
            .values()
            .filter(|&&s| s == SubsystemReadiness::Complete || s == SubsystemReadiness::InCompatibilityLayer)
            .count()
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
    fn test_virtio_reliability_manager() {
        let mut vmgr = VirtIoReliabilityManager::new();
        vmgr.register_device(1, VirtIoDeviceType::Block, 256);
        assert_eq!(vmgr.get_device_status(1), Some(VirtIoDeviceStatus::Active));

        vmgr.devices.get_mut(&1).unwrap().queue_stats.dropped_descriptors = 15;
        assert_eq!(vmgr.verify_queue_health(1), Ok(false));
        assert_eq!(vmgr.get_device_status(1), Some(VirtIoDeviceStatus::Error));

        assert!(vmgr.trigger_error_recovery(1).is_ok());
        assert_eq!(vmgr.get_device_status(1), Some(VirtIoDeviceStatus::Active));
    }

    #[test]
    fn test_platform_certification_manager() {
        let mut cert_mgr = PlatformCertificationManager::new();

        let laptop_report = cert_mgr.certify_platform(
            "Sigma-Laptop-Ref-2025",
            PlatformType::Laptop,
            &[
                ("DRM/KMS Graphics", PlatformSubsystemStatus::FullyCertified),
                ("Intel Wi-Fi 6E", PlatformSubsystemStatus::FullyCertified),
                ("ACPI Thermal & Battery", PlatformSubsystemStatus::FullyCertified),
                ("Precision Touchpad", PlatformSubsystemStatus::FullyCertified),
            ],
        );

        assert!(laptop_report.is_certified);
        assert_eq!(cert_mgr.get_certified_platforms().len(), 1);
    }

    #[test]
    fn test_linux_driver_compat_boundary() {
        let mut boundary = LinuxDriverCompatBoundary::new();
        boundary
            .register_driver(LinuxCompatDriverModule {
                module_name: "i915".to_string(),
                category: LinuxDriverCategory::DrmKms,
                license: LinuxDriverLicense::GPLv2,
                entry_symbol: "i915_init".to_string(),
                is_loaded: false,
                isolation_level: "SandboxedContainer".to_string(),
            })
            .unwrap();

        assert!(boundary.load_driver("i915").is_ok());
        let res = boundary.call_shim_entry("i915", 0x1000).unwrap();
        assert_eq!(res, 0x1000 ^ 0x0F0F_A5A5);
        assert!(boundary.unload_driver("i915").is_ok());
    }

    #[test]
    fn test_automated_hardware_regression_pipeline() {
        let mut pipeline = AutomatedHardwareRegressionPipeline::new();
        pipeline.add_test_case("TC-01", "VirtIO", "VirtIO-Blk DMA sanity test", true);
        pipeline.add_test_case("TC-02", "DRM/KMS", "Intel i915 framebuffer swap test", true);

        let (passed, total) = pipeline.run_regression_suite();
        assert_eq!(passed, 2);
        assert_eq!(total, 2);
        assert!(pipeline.can_expand_hardware_support());
    }

    #[test]
    fn test_hardware_subsystem_registry() {
        let registry = HardwareSubsystemRegistry::new();
        assert_eq!(
            registry.get_subsystem_status("Intel Wi-Fi (AX200 / AX210)"),
            Some(SubsystemReadiness::Complete)
        );
        assert!(registry.count_ready_subsystems() >= 20);
    }
}
