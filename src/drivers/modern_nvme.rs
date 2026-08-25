// Modern high-performance NVMe PCIe block storage & AHCI SATA Controller Driver
// Conforms to SigmaOS Unified Peripheral Architecture

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


/// AHCI Command Header Structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AhciCommandHeader {
    pub opts: u16,
    pub prdtl: u16,
    pub prdbc: u32,
    pub ctba: u64,
    pub reserved: [u32; 4],
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
            cmd_headers: [AhciCommandHeader { opts: 0, prdtl: 0, prdbc: 0, ctba: 0, reserved: [0; 4] }; 32],
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
pub enum DeviceGeneration {
    Legacy,
    Modern,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Off,
    On,
}

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

/// AHCI SATA Physical Region Descriptor Table (PRDT) Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AhciPrdtEntry {
    pub dba: u64, // Data Base Address
    pub reserved: u32,
    pub dbc: u32, // Data Byte Count & Interrupt-on-Completion
}

/// AHCI Serial ATA Storage Controller Driver
pub struct AhciStorageDriver {
    pub is_initialized: bool,
    pub power_state: PowerState,
    pub ports_active_mask: u32,
    pub command_headers: [AhciCommandHeader; 32],
}

impl Default for AhciStorageDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl AhciStorageDriver {
    pub fn new() -> Self {
        AhciStorageDriver {
            is_initialized: false,
            power_state: PowerState::Off,
            ports_active_mask: 0x00000001, // Port 0 active
            command_headers: [AhciCommandHeader {
                opts: 0,
                prdtl: 1,
                prdbc: 0,
                ctba: 0x100000,

            }; 32],
        }
    }

    /// Issue SATA ATA Read DMA command over AHCI Port
    pub fn execute_sata_read_dma(&mut self, port: usize, lba: u64, sectors: u32, buf: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("AHCI driver not initialized");
        }
        if port >= 32 || (self.ports_active_mask & (1 << port)) == 0 {
            return Err("AHCI Port inactive");
        }

        // Simulate AHCI DMA PRDT transfer
        let bytes_expected = (sectors as usize) * 512;
        let read_len = buf.len().min(bytes_expected);
        for (i, byte) in buf[..read_len].iter_mut().enumerate() {
            *byte = ((lba as usize + i) % 256) as u8;
        }

        Ok(read_len)
    }

    /// Issue SATA ATA Write DMA command over AHCI Port
    pub fn execute_sata_write_dma(&mut self, port: usize, _lba: u64, _sectors: u32, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("AHCI driver not initialized");
        }
        if port >= 32 || (self.ports_active_mask & (1 << port)) == 0 {
            return Err("AHCI Port inactive");
        }

        Ok(data.len())
    }
}

impl PeripheralDevice for AhciStorageDriver {
    fn name(&self) -> &'static str {
        "AHCI Serial ATA Storage Driver"
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
        self.execute_sata_read_dma(0, 0, (buffer.len() / 512).max(1) as u32, buffer)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        self.execute_sata_write_dma(0, 0, (data.len() / 512).max(1) as u32, data)
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

/// PCIe NVMe Solid-State Block Driver
pub struct ModernNvmeDriver {
    pub is_initialized: bool,
    pub power_state: PowerState,
    pub lba_count: u64,
    pub submission_doorbell: u32,
    pub completion_doorbell: u32,
}

impl ModernNvmeDriver {
    pub fn new(lba_count: u64) -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            lba_count,
            submission_doorbell: 0,
            completion_doorbell: 0,
        }
    }

    pub fn get_lba_count(&self) -> u64 {
        self.lba_count
    }

    /// Ring NVMe Submission Queue Doorbell
    pub fn ring_submission_doorbell(&mut self, tail_ptr: u32) {
        self.submission_doorbell = tail_ptr;
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
    fn test_ahci_sata_driver() {
        let mut ahci = AhciStorageDriver::new();
        ahci.initialize().unwrap();
        let mut buf = [0u8; 512];
        let bytes = ahci.read(&mut buf).unwrap();
        assert_eq!(bytes, 512);
        assert_eq!(ahci.name(), "AHCI Serial ATA Storage Driver");
    }
}
