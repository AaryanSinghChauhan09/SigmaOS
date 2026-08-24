// SigmaOS PCI / PCIe Bus Driver & Bus Manager Implementation
// Clean-room implementation taking inspiration from Linux (pci-sysfs, ECAM, BAR allocation, MSI/MSI-X, PCIe AER, PCIe ASPM)
// and BSD distributions (FreeBSD devctl/pci ioctls, OpenBSD autoconf PCI matching).

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use core::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// 1. Core Constants & Architectural Enums
// ============================================================================

pub const PCI_CONFIG_ADDRESS_PORT: u16 = 0x0CF8;
pub const PCI_CONFIG_DATA_PORT: u16 = 0x0CFC;

pub const PCI_MAX_BUSES: usize = 256;
pub const PCI_MAX_DEVICES_PER_BUS: u8 = 32;
pub const PCI_MAX_FUNCTIONS_PER_DEVICE: u8 = 8;

pub const PCIE_ECAM_BUS_SIZE: u64 = 1024 * 1024; // 1 MB per bus

/// PCI Bus/Device/Function address triple (Domain:Bus:Device:Function)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciAddress {
    pub domain: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

impl PciAddress {
    pub fn new(domain: u16, bus: u8, device: u8, function: u8) -> Self {
        Self {
            domain,
            bus,
            device,
            function,
        }
    }

    /// Formats as standard Linux/BSD sysfs PCI address format `0000:00:1f.2`
    pub fn sysfs_format(&self) -> String {
        format!(
            "{:04x}:{:02x}:{:02x}.{:x}",
            self.domain, self.bus, self.device, self.function
        )
    }

    /// Calculates legacy 32-bit `0xCF8` IO-port configuration space register address
    pub fn legacy_io_address(&self, register_offset: u8) -> u32 {
        0x8000_0000
            | ((self.bus as u32) << 16)
            | ((self.device as u32) << 11)
            | ((self.function as u32) << 8)
            | ((register_offset as u32) & 0xFC)
    }

    /// Calculates PCIe ECAM memory-mapped configuration space physical offset
    pub fn ecam_offset(&self, base_phys_addr: u64, register_offset: u16) -> u64 {
        base_phys_addr
            + ((self.bus as u64) << 20)
            + ((self.device as u64) << 15)
            + ((self.function as u64) << 12)
            + ((register_offset as u64) & 0xFFF)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciHeaderType {
    StandardDevice,
    PciToPciBridge,
    CardBusBridge,
    Unknown(u8),
}

impl PciHeaderType {
    pub fn raw_code(&self) -> u8 {
        match self {
            PciHeaderType::StandardDevice => 0x00,
            PciHeaderType::PciToPciBridge => 0x01,
            PciHeaderType::CardBusBridge => 0x02,
            PciHeaderType::Unknown(code) => *code,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciBarType {
    Memory32 { prefetchable: bool },
    Memory64 { prefetchable: bool },
    IoSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciBarInfo {
    pub index: u8,
    pub bar_type: PciBarType,
    pub base_address: u64,
    pub size: u64,
    pub is_assigned: bool,
}

/// PCIe Active State Power Management (ASPM) Link States (Linux sysfs ASPM parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcieAspmState {
    Disabled,
    L0s,
    L1,
    L0sAndL1,
    L1Substates,
}

/// PCIe Advanced Error Reporting (AER) Severity (Linux PCIe AER sub-driver parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcieAerSeverity {
    Correctable,
    UncorrectableNonFatal,
    UncorrectableFatal,
}

#[derive(Debug, Clone)]
pub struct PcieAerLog {
    pub timestamp_ms: u64,
    pub address: PciAddress,
    pub severity: PcieAerSeverity,
    pub error_status_register: u32,
    pub header_log: [u32; 4],
}

/// MSI/MSI-X Interrupt Mode Capability Allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciInterruptMode {
    LegacyPin { irq_line: u8, pin: u8 },
    Msi { vector_base: u16, count: u8 },
    MsiX { vector_base: u16, table_size: u16, table_bar: u8 },
}

// ============================================================================
// 2. PCI Device Node & Driver Matching
// ============================================================================

#[derive(Debug, Clone)]
pub struct PciDeviceNode {
    pub address: PciAddress,
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub class_code: u8,
    pub subclass_code: u8,
    pub programming_interface: u8,
    pub revision_id: u8,
    pub header_type: PciHeaderType,
    pub is_multi_function: bool,
    pub bars: Vec<PciBarInfo>,
    pub interrupt_mode: PciInterruptMode,
    pub aspm_state: PcieAspmState,
    pub bound_driver: Option<String>,
    pub command_register: u16,
    pub status_register: u16,
}

impl PciDeviceNode {
    pub fn new(address: PciAddress, vendor_id: u16, device_id: u16) -> Self {
        Self {
            address,
            vendor_id,
            device_id,
            subsystem_vendor_id: 0,
            subsystem_device_id: 0,
            class_code: 0,
            subclass_code: 0,
            programming_interface: 0,
            revision_id: 0,
            header_type: PciHeaderType::StandardDevice,
            is_multi_function: false,
            bars: Vec::new(),
            interrupt_mode: PciInterruptMode::LegacyPin { irq_line: 0, pin: 0 },
            aspm_state: PcieAspmState::Disabled,
            bound_driver: None,
            command_register: 0x0006, // Bus Master + Memory Space enabled default
            status_register: 0x0010,  // Capabilities list supported
        }
    }

    /// Enables PCI Bus Mastering (DMA privilege) in Command Register
    pub fn enable_bus_mastering(&mut self) {
        self.command_register |= 0x0004;
    }

    /// Enables Memory Space decoding in Command Register
    pub fn enable_memory_space(&mut self) {
        self.command_register |= 0x0002;
    }

    /// Enables IO Space decoding in Command Register
    pub fn enable_io_space(&mut self) {
        self.command_register |= 0x0001;
    }
}

/// OpenBSD / Linux inspired Autoconf PCI Driver Matching Rule
#[derive(Debug, Clone)]
pub struct PciDriverMatchRule {
    pub driver_name: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_mask: Option<(u8, u8)>, // (class, subclass)
    pub priority: u32,
}

impl PciDriverMatchRule {
    pub fn matches(&self, dev: &PciDeviceNode) -> bool {
        if self.vendor_id != 0xFFFF && self.vendor_id != dev.vendor_id {
            return false;
        }
        if self.device_id != 0xFFFF && self.device_id != dev.device_id {
            return false;
        }
        if let Some((req_class, req_subclass)) = self.class_mask {
            if dev.class_code != req_class || dev.subclass_code != req_subclass {
                return false;
            }
        }
        true
    }
}

// ============================================================================
// 3. Hardware Access Backend (Mockable simulated hardware registers + I/O)
// ============================================================================

pub trait PciHardwareAccess: Send + Sync {
    fn read_config_u32(&self, addr: PciAddress, reg: u16) -> u32;
    fn write_config_u32(&self, addr: PciAddress, reg: u16, value: u32);
    fn read_config_u16(&self, addr: PciAddress, reg: u16) -> u16 {
        let val32 = self.read_config_u32(addr, reg & !3);
        let shift = (reg & 2) * 8;
        ((val32 >> shift) & 0xFFFF) as u16
    }
    fn read_config_u8(&self, addr: PciAddress, reg: u16) -> u8 {
        let val32 = self.read_config_u32(addr, reg & !3);
        let shift = (reg & 3) * 8;
        ((val32 >> shift) & 0xFF) as u8
    }
}

/// Software simulated PCI hardware configuration space backplane
pub struct SimulatedPciHardwareAccess {
    // Stores fake config space dwords indexed by key (address, reg)
    pub simulated_devices: Vec<(PciAddress, Vec<u32>)>,
}

impl SimulatedPciHardwareAccess {
    pub fn new() -> Self {
        Self {
            simulated_devices: Vec::new(),
        }
    }

    pub fn add_simulated_device(&mut self, addr: PciAddress, config_dwords: &[u32]) {
        self.simulated_devices.push((addr, config_dwords.to_vec()));
    }
}

impl Default for SimulatedPciHardwareAccess {
    fn default() -> Self {
        Self::new()
    }
}

impl PciHardwareAccess for SimulatedPciHardwareAccess {
    fn read_config_u32(&self, addr: PciAddress, reg: u16) -> u32 {
        let dword_idx = (reg >> 2) as usize;
        for (dev_addr, dwords) in &self.simulated_devices {
            if *dev_addr == addr {
                if dword_idx < dwords.len() {
                    return dwords[dword_idx];
                }
            }
        }
        0xFFFF_FFFF // Non-existent PCI device returns all 1s
    }

    fn write_config_u32(&self, addr: PciAddress, reg: u16, value: u32) {
        let _ = (addr, reg, value);
    }
}

// ============================================================================
// 4. Main PCI / PCIe Bus Manager Engine
// ============================================================================

pub struct PciBusManager {
    pub devices: Vec<PciDeviceNode>,
    pub driver_rules: Vec<PciDriverMatchRule>,
    pub aer_logs: Vec<PcieAerLog>,
    pub ecam_base_address: Option<u64>,
    pub next_mem32_alloc: u32,
    pub next_mem64_alloc: u64,
    pub next_io_alloc: u16,
    pub msi_vector_allocator: AtomicU32,
}

impl PciBusManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            driver_rules: Vec::new(),
            aer_logs: Vec::new(),
            ecam_base_address: None,
            next_mem32_alloc: 0xE000_0000,     // 3.5GB base MMIO space for 32-bit BARs
            next_mem64_alloc: 0x10_0000_0000, // 64GB base MMIO space for 64-bit BARs
            next_io_alloc: 0x2000,             // I/O port alloc base
            msi_vector_allocator: AtomicU32::new(32), // Legacy IRQs 0-31 reserved
        }
    }

    /// Sets PCIe ECAM (Enhanced Configuration Access Mechanism) MMIO base physical address
    pub fn set_ecam_base(&mut self, base_address: u64) {
        self.ecam_base_address = Some(base_address);
    }

    /// Registers a driver autoconf matching rule (OpenBSD/Linux probe match parity)
    pub fn register_driver_rule(
        &mut self,
        driver_name: &str,
        vendor_id: u16,
        device_id: u16,
        class_mask: Option<(u8, u8)>,
        priority: u32,
    ) {
        self.driver_rules.push(PciDriverMatchRule {
            driver_name: String::from(driver_name),
            vendor_id,
            device_id,
            class_mask,
            priority,
        });
    }

    /// Scans hardware backplane across PCI buses, slots, and functions (Recursive multi-function probe)
    pub fn scan_bus(&mut self, hw: &dyn PciHardwareAccess, domain: u16, max_bus: u8) -> usize {
        let mut count = 0;
        for bus in 0..=max_bus {
            for slot in 0..32 {
                for func in 0..8 {
                    let addr = PciAddress::new(domain, bus, slot, func);
                    let vendor_dev = hw.read_config_u32(addr, 0x00);
                    let vendor_id = (vendor_dev & 0xFFFF) as u16;
                    let device_id = ((vendor_dev >> 16) & 0xFFFF) as u16;

                    if vendor_id == 0xFFFF || vendor_id == 0x0000 {
                        if func == 0 {
                            break; // Skip remaining functions if function 0 is absent
                        }
                        continue;
                    }

                    let mut node = PciDeviceNode::new(addr, vendor_id, device_id);

                    // Read Header Type & Multi-function flag
                    let header_type_reg = hw.read_config_u8(addr, 0x0E);
                    node.is_multi_function = (header_type_reg & 0x80) != 0;
                    node.header_type = match header_type_reg & 0x7F {
                        0x00 => PciHeaderType::StandardDevice,
                        0x01 => PciHeaderType::PciToPciBridge,
                        0x02 => PciHeaderType::CardBusBridge,
                        other => PciHeaderType::Unknown(other),
                    };

                    // Read Class & Subclass
                    let class_rev = hw.read_config_u32(addr, 0x08);
                    node.revision_id = (class_rev & 0xFF) as u8;
                    node.programming_interface = ((class_rev >> 8) & 0xFF) as u8;
                    node.subclass_code = ((class_rev >> 16) & 0xFF) as u8;
                    node.class_code = ((class_rev >> 24) & 0xFF) as u8;

                    // Read Subsystem IDs (for standard devices at reg 0x2C)
                    if node.header_type == PciHeaderType::StandardDevice {
                        let sub_ids = hw.read_config_u32(addr, 0x2C);
                        node.subsystem_vendor_id = (sub_ids & 0xFFFF) as u16;
                        node.subsystem_device_id = ((sub_ids >> 16) & 0xFFFF) as u16;

                        // Decode BARs (0x10..0x24)
                        self.probe_device_bars(hw, &mut node);
                    }

                    // Autoprobe and bind driver rule
                    self.autoprobe_driver(&mut node);

                    self.devices.push(node);
                    count += 1;

                    if func == 0 && (header_type_reg & 0x80) == 0 {
                        // Single-function device: stop checking remaining functions for this device
                        break;
                    }
                }
            }
        }
        count
    }

    /// Probes and decodes BAR (Base Address Registers) for standard PCI devices
    fn probe_device_bars(&self, hw: &dyn PciHardwareAccess, node: &mut PciDeviceNode) {
        let mut bar_idx = 0u8;
        while bar_idx < 6 {
            let reg_offset = 0x10 + (bar_idx * 4) as u16;
            let raw_bar0 = hw.read_config_u32(node.address, reg_offset);
            if raw_bar0 == 0 || raw_bar0 == 0xFFFF_FFFF {
                bar_idx += 1;
                continue;
            }

            let is_io = (raw_bar0 & 0x01) != 0;
            if is_io {
                let base_addr = (raw_bar0 & !0x03) as u64;
                node.bars.push(PciBarInfo {
                    index: bar_idx,
                    bar_type: PciBarType::IoSpace,
                    base_address: base_addr,
                    size: 0x100, // Standard IO size default
                    is_assigned: base_addr != 0,
                });
                bar_idx += 1;
            } else {
                let is_64bit = ((raw_bar0 >> 1) & 0x03) == 0x02;
                let prefetchable = (raw_bar0 & 0x08) != 0;

                let base_addr = if is_64bit && bar_idx < 5 {
                    let raw_bar1 = hw.read_config_u32(node.address, reg_offset + 4);
                    ((raw_bar1 as u64) << 32) | ((raw_bar0 & !0x0F) as u64)
                } else {
                    (raw_bar0 & !0x0F) as u64
                };

                let bar_type = if is_64bit {
                    PciBarType::Memory64 { prefetchable }
                } else {
                    PciBarType::Memory32 { prefetchable }
                };

                node.bars.push(PciBarInfo {
                    index: bar_idx,
                    bar_type,
                    base_address: base_addr,
                    size: 0x10_0000, // Standard 1MB default size simulation
                    is_assigned: base_addr != 0,
                });

                if is_64bit {
                    bar_idx += 2; // 64-bit BAR consumes two 32-bit registers
                } else {
                    bar_idx += 1;
                }
            }
        }
    }

    /// Dynamically allocates and assigns physical BAR memory ranges (Linux resource assignment parity)
    pub fn allocate_bar_resources(&mut self, address: PciAddress) -> Result<(), &'static str> {
        let mem32 = &mut self.next_mem32_alloc;
        let mem64 = &mut self.next_mem64_alloc;
        let io_alloc = &mut self.next_io_alloc;

        if let Some(dev) = self.devices.iter_mut().find(|d| d.address == address) {
            for bar in dev.bars.iter_mut() {
                if !bar.is_assigned || bar.base_address == 0 {
                    match bar.bar_type {
                        PciBarType::IoSpace => {
                            bar.base_address = *io_alloc as u64;
                            *io_alloc = io_alloc.wrapping_add(0x100);
                        }
                        PciBarType::Memory32 { .. } => {
                            bar.base_address = *mem32 as u64;
                            *mem32 = mem32.wrapping_add(bar.size as u32);
                        }
                        PciBarType::Memory64 { .. } => {
                            bar.base_address = *mem64;
                            *mem64 = mem64.wrapping_add(bar.size);
                        }
                    }
                    bar.is_assigned = true;
                }
            }
            Ok(())
        } else {
            Err("PCI Device not found")
        }
    }

    /// Allocates MSI / MSI-X interrupt vectors for a given PCI device
    pub fn request_msi_vectors(
        &mut self,
        address: PciAddress,
        count: u8,
    ) -> Result<PciInterruptMode, &'static str> {
        if count == 0 {
            return Err("Vector count must be greater than zero");
        }

        let base_vector = self.msi_vector_allocator.fetch_add(count as u32, Ordering::SeqCst) as u16;

        if let Some(dev) = self.devices.iter_mut().find(|d| d.address == address) {
            let mode = PciInterruptMode::Msi {
                vector_base: base_vector,
                count,
            };
            dev.interrupt_mode = mode;
            Ok(mode)
        } else {
            Err("PCI Device not found")
        }
    }

    /// Configures PCIe ASPM (Active State Power Management) L0s/L1 power savings state
    pub fn set_pcie_aspm(
        &mut self,
        address: PciAddress,
        state: PcieAspmState,
    ) -> Result<(), &'static str> {
        if let Some(dev) = self.devices.iter_mut().find(|d| d.address == address) {
            dev.aspm_state = state;
            Ok(())
        } else {
            Err("PCI Device not found")
        }
    }

    /// Logs a PCIe Advanced Error Reporting (AER) event
    pub fn log_aer_error(
        &mut self,
        address: PciAddress,
        severity: PcieAerSeverity,
        status_reg: u32,
        header_log: [u32; 4],
        timestamp_ms: u64,
    ) {
        self.aer_logs.push(PcieAerLog {
            timestamp_ms,
            address,
            severity,
            error_status_register: status_reg,
            header_log,
        });
    }

    /// Probes registered driver autoconf rules and binds the best match to the device
    fn autoprobe_driver(&self, dev: &mut PciDeviceNode) {
        let mut best_match: Option<(&str, u32)> = None;

        for rule in &self.driver_rules {
            if rule.matches(dev) {
                if let Some((_, best_pri)) = best_match {
                    if rule.priority > best_pri {
                        best_match = Some((rule.driver_name.as_str(), rule.priority));
                    }
                } else {
                    best_match = Some((rule.driver_name.as_str(), rule.priority));
                }
            }
        }

        if let Some((driver_name, _)) = best_match {
            dev.bound_driver = Some(String::from(driver_name));
        }
    }

    /// FreeBSD `devctl`/`pci` ioctl parity: Exports device listing formatted as devctl inspect strings
    pub fn export_freebsd_devctl_listing(&self) -> Vec<String> {
        let mut listing = Vec::new();
        for dev in &self.devices {
            let driver_str = dev.bound_driver.as_deref().unwrap_or("none");
            let line = format!(
                "pci{}@pci{:04x}:{:02x}:{:02x}:{:02x}: class=0x{:02x}{:02x}{:02x} card=0x{:04x}{:04x} chip=0x{:04x}{:04x} rev=0x{:02x} hdr=0x{:02x}",
                dev.address.device,
                dev.address.domain,
                dev.address.bus,
                dev.address.device,
                dev.address.function,
                dev.class_code,
                dev.subclass_code,
                dev.programming_interface,
                dev.subsystem_device_id,
                dev.subsystem_vendor_id,
                dev.device_id,
                dev.vendor_id,
                dev.revision_id,
                dev.header_type.raw_code(),
            );
            listing.push(format!("{} driver={}", line, driver_str));
        }
        listing
    }
}

impl Default for PciBusManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_address_formatting() {
        let addr = PciAddress::new(0, 1, 31, 2);
        assert_eq!(addr.sysfs_format(), "0000:01:1f.2");

        let legacy_io = addr.legacy_io_address(0x10);
        assert_eq!(legacy_io, 0x8001_FA10);

        let ecam_off = addr.ecam_offset(0xE000_0000, 0x100);
        // 0xE000_0000 + (1 << 20) + (31 << 15) + (2 << 12) + 0x100
        // = 0xE000_0000 + 0x100000 + 0xF8000 + 0x2000 + 0x100 = 0xE01FA100
        assert_eq!(ecam_off, 0xE01F_A100);
    }

    #[test]
    fn test_pci_bus_scan_and_driver_autobind() {
        let mut hw = SimulatedPciHardwareAccess::new();

        // Intel Network Controller (Vendor 0x8086, Device 0x100E) at 0000:00:03.0
        // Class 0x02 (Network), Subclass 0x00 (Ethernet), BAR0 Mem32 at 0xFE000000
        let e1000_config = vec![
            0x100E_8086, // 0x00: Vendor & Device
            0x0010_0006, // 0x04: Command & Status
            0x0200_0001, // 0x08: Class (0x02), Subclass (0x00), PI (0x00), Rev (0x01)
            0x0000_0000, // 0x0C: Header (0x00 standard)
            0xFE00_0000, // 0x10: BAR0 32-bit Memory
            0x0000_0000, // 0x14: BAR1
            0x0000_0000, // 0x18: BAR2
            0x0000_0000, // 0x1C: BAR3
            0x0000_0000, // 0x20: BAR4
            0x0000_0000, // 0x24: BAR5
            0x0000_0000, // 0x28: Cardbus CIS
            0x0001_8086, // 0x2C: Subsystem Vendor & Device
        ];

        let addr = PciAddress::new(0, 0, 3, 0);
        hw.add_simulated_device(addr, &e1000_config);

        let mut mgr = PciBusManager::new();
        mgr.register_driver_rule("e1000e", 0x8086, 0x100E, None, 100);

        let count = mgr.scan_bus(&hw, 0, 0);
        assert_eq!(count, 1);
        assert_eq!(mgr.devices.len(), 1);

        let dev = &mgr.devices[0];
        assert_eq!(dev.vendor_id, 0x8086);
        assert_eq!(dev.device_id, 0x100E);
        assert_eq!(dev.bound_driver.as_deref(), Some("e1000e"));
        assert_eq!(dev.bars.len(), 1);
        assert_eq!(dev.bars[0].base_address, 0xFE00_0000);
    }

    #[test]
    fn test_pci_bar_allocation_and_msi() {
        let mut mgr = PciBusManager::new();
        let addr = PciAddress::new(0, 1, 0, 0);
        let mut dev = PciDeviceNode::new(addr, 0x10DE, 0x2204); // Nvidia GPU

        // Unassigned BAR
        dev.bars.push(PciBarInfo {
            index: 0,
            bar_type: PciBarType::Memory64 { prefetchable: true },
            base_address: 0,
            size: 16 * 1024 * 1024, // 16MB BAR
            is_assigned: false,
        });

        mgr.devices.push(dev);

        // Allocate BAR resources
        assert!(mgr.allocate_bar_resources(addr).is_ok());

        let dev_after = &mgr.devices[0];
        assert!(dev_after.bars[0].is_assigned);
        assert_eq!(dev_after.bars[0].base_address, 0x10_0000_0000); // Allocated from 64GB range

        // Request MSI vectors
        let msi_mode = mgr.request_msi_vectors(addr, 4).unwrap();
        match msi_mode {
            PciInterruptMode::Msi { vector_base, count } => {
                assert_eq!(vector_base, 32);
                assert_eq!(count, 4);
            }
            _ => panic!("Expected MSI mode"),
        }
    }

    #[test]
    fn test_pcie_aspm_and_aer_logging() {
        let mut mgr = PciBusManager::new();
        let addr = PciAddress::new(0, 2, 0, 0);
        let dev = PciDeviceNode::new(addr, 0x14E4, 0x43A0); // Broadcom Wi-Fi
        mgr.devices.push(dev);

        // Set ASPM state
        assert!(mgr.set_pcie_aspm(addr, PcieAspmState::L1).is_ok());
        assert_eq!(mgr.devices[0].aspm_state, PcieAspmState::L1);

        // Log AER error
        mgr.log_aer_error(
            addr,
            PcieAerSeverity::Correctable,
            0x0000_0001,
            [0x1, 0x2, 0x3, 0x4],
            10000,
        );
        assert_eq!(mgr.aer_logs.len(), 1);
        assert_eq!(mgr.aer_logs[0].severity, PcieAerSeverity::Correctable);
    }

    #[test]
    fn test_freebsd_devctl_export() {
        let mut mgr = PciBusManager::new();
        let addr = PciAddress::new(0, 0, 2, 0);
        let mut dev = PciDeviceNode::new(addr, 0x8086, 0x5917);
        dev.bound_driver = Some("i915kms".to_string());
        mgr.devices.push(dev);

        let devctl_out = mgr.export_freebsd_devctl_listing();
        assert_eq!(devctl_out.len(), 1);
        assert!(devctl_out[0].contains("pci2@pci0000:00:02:00"));
        assert!(devctl_out[0].contains("driver=i915kms"));
    }
}
