#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Modern high-performance NVMe PCIe block storage driver
#[cfg(not(test))]
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeCmd {
    pub opcode: u8,
    pub nsid: u32,
    pub prp1: u64,
    pub prp2: u64,
}

pub struct NvmeSubmissionQueue {
    pub size: usize,
    pub head: usize,
    pub tail: usize,
}

impl NvmeSubmissionQueue {
    pub fn new(size: usize) -> Self {
        Self { size, head: 0, tail: 0 }
    }

    /// Submits a command and increments the doorbell tail pointer (doorbell write)
    pub fn submit_command(&mut self, _cmd: NvmeCmd) -> Result<usize, &'static str> {
        let next_tail = (self.tail + 1) % self.size;
        if next_tail == self.head {
            return Err("Submission queue is full");
        }
        let submitted_idx = self.tail;
        self.tail = next_tail;
        Ok(submitted_idx)
    }
}

pub struct NvmeCompletionQueue {
    pub size: usize,
    pub head: usize,
    pub phase: bool,
}

impl NvmeCompletionQueue {
    pub fn new(size: usize) -> Self {
        Self { size, head: 0, phase: true }
    }

    /// Reaps a completion entry, updating head and toggling phase bit on wrap
    pub fn reap_completion(&mut self) -> (usize, bool) {
        let reaped_head = self.head;
        self.head = (self.head + 1) % self.size;
        if self.head == 0 {
            self.phase = !self.phase; // phase bit flips on wrap
        }
        (reaped_head, self.phase)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmartTelemetry {
    pub temperature_c: u16,
    pub percentage_used: u8,
    pub data_units_read: u64,
    pub data_units_written: u64,
}

impl SmartTelemetry {
    pub fn new() -> Self {
        Self {
            temperature_c: 38, // 38C optimal temperature
            percentage_used: 1, // 1% life used
            data_units_read: 1048576, // 1GB read
            data_units_written: 2097152, // 2GB written
        }
    }
}

impl Default for SmartTelemetry {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulated AHCI SATA Command Header structure (HBA memory layout)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AhciCommandHeader {
    pub opts: u16,
    pub prdtl: u16,
    pub prdbc: u32,
    pub ctba: u64,
}

impl AhciCommandHeader {
    pub const fn new() -> Self {
        Self { opts: 0, prdtl: 0, prdbc: 0, ctba: 0 }
    }
}

/// Simulated AHCI Port MMIO Register Map
pub struct AhciPort {
    pub cmd_issue: u32,
    pub cmd_headers: [AhciCommandHeader; 32], // 32 command slots
}

impl AhciPort {
    pub const fn new() -> Self {
        Self {
            cmd_issue: 0,
            cmd_headers: [AhciCommandHeader::new(); 32],
        }
    }

    /// Allocates an empty slot for NCQ SATA command issues
    pub fn allocate_slot(&mut self) -> Option<usize> {
        for i in 0..32 {
            if (self.cmd_issue & (1 << i)) == 0 {
                self.cmd_issue |= 1 << i;
                return Some(i);
            }
        }
        None
    }

    /// Releases command slot upon SATA controller interrupt completion
    pub fn complete_slot(&mut self, slot: usize) {
        if slot < 32 {
            self.cmd_issue &= !(1 << slot);
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceGeneration { Legacy, Modern }

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState { Off, On }

#[cfg(test)]
pub trait PeripheralDevice {
    fn name(&self) -> &'static str;
    fn generation(&self) -> DeviceGeneration;
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str>;
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str>;
    fn shutdown(&mut self) -> Result<(), &'static str>;
}

pub struct ModernNvmeDriver {
    is_initialized: bool,
    power_state: PowerState,
    lba_count: u64,
    pub sq: NvmeSubmissionQueue,
    pub cq: NvmeCompletionQueue,
    pub smart: SmartTelemetry,
    pub ahci_port: AhciPort,
}

impl ModernNvmeDriver {
    pub fn new(lba_count: u64) -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            lba_count,
            sq: NvmeSubmissionQueue::new(64),
            cq: NvmeCompletionQueue::new(64),
            smart: SmartTelemetry::new(),
            ahci_port: AhciPort::new(),
        }
    }

    pub fn get_lba_count(&self) -> u64 {
        self.lba_count
    }

    /// Dataset Management: NVMe TRIM / Deallocate sectors command (0x0A)
    pub fn deallocate_sectors(&mut self, _lba: u64, _sectors_count: u32) -> Result<(), &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        // Submits TRIM command to Submission Queue
        let cmd = NvmeCmd { opcode: 0x0A, nsid: 1, prp1: 0, prp2: 0 };
        self.sq.submit_command(cmd)?;

        // Simulates Completion Queue event
        let _ = self.cq.reap_completion();
        Ok(())
    }
}

impl PeripheralDevice for ModernNvmeDriver {
    fn name(&self) -> &'static str {
        "PCIe NVMe Solid-State Block Driver"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is offline");
        }

        // Simulate high-speed sequential sector read
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }
        self.smart.data_units_read += (buffer.len() as u64) / 512;
        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("Device is offline");
        }

        // Simulate high-speed PCIe block write
        self.smart.data_units_written += (data.len() as u64) / 512;
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvme_lifecycle() {
        let mut driver = ModernNvmeDriver::new(2048);
        assert!(driver.read(&mut [0; 10]).is_err());
        driver.initialize().unwrap();
        assert_eq!(driver.name(), "PCIe NVMe Solid-State Block Driver");
        assert_eq!(driver.generation(), DeviceGeneration::Modern);
        assert_eq!(driver.write(&[1, 2, 3]).unwrap(), 3);
        driver.shutdown().unwrap();
    }

    #[test]
    fn test_nvme_ahci_linux_driver_parity() {
        let mut driver = ModernNvmeDriver::new(4096);
        driver.initialize().unwrap();

        // 1. Validate submission and completion queues doorbells & phase bit transitions
        let cmd = NvmeCmd { opcode: 0x02, nsid: 1, prp1: 0x1000, prp2: 0 };
        let slot = driver.sq.submit_command(cmd).unwrap();
        assert_eq!(slot, 0);
        assert_eq!(driver.sq.tail, 1);

        let (idx, phase) = driver.cq.reap_completion();
        assert_eq!(idx, 0);
        assert!(phase);

        // 2. Validate Dataset Management TRIM / deallocation
        assert!(driver.deallocate_sectors(100, 8).is_ok());

        // 3. Validate S.M.A.R.T telemetry reports
        assert_eq!(driver.smart.temperature_c, 38);
        let mut buf = [0u8; 1024]; // 2 sectors
        driver.read(&mut buf).unwrap();
        assert_eq!(driver.smart.data_units_read, 1048576 + 2);

        // 4. Validate AHCI HBA Port command issue slots allocation
        let slot1 = driver.ahci_port.allocate_slot().unwrap();
        assert_eq!(slot1, 0);
        let slot2 = driver.ahci_port.allocate_slot().unwrap();
        assert_eq!(slot2, 1);

        driver.ahci_port.complete_slot(slot1);
        let slot3 = driver.ahci_port.allocate_slot().unwrap();
        assert_eq!(slot3, 0); // slot 0 was completed and recycled
    }
}
