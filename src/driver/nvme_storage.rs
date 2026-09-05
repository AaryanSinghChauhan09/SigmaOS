// SPDX-License-Identifier: MIT
// SigmaOS NVMe Storage Driver
// Supports NVMe 1.0+ SSDs with queue pair and completion queue management

use std::boxed::Box;
use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicU32, AtomicU16, Ordering};

use crate::driver::pci_enumeration::{PciDeviceInfo, PciDriver};

// ============================================================================
// NVMe Constants
// ============================================================================

pub const NVME_VENDOR_ID: u16 = 0x8086; // Intel NVMe devices commonly used

// Common NVMe Device Classes
pub const NVME_CLASS_MASS_STORAGE: u8 = 0x01;
pub const NVME_SUBCLASS_NVM: u8 = 0x08;

// PCI Configuration Space Offsets
pub const PCI_CAP_OFFSET: u32 = 0x34;
pub const PCI_MSIX_CAP_ID: u8 = 0x11;

// NVMe Register Space
pub const NVME_CAP: u32 = 0x00; // Capabilities
pub const NVME_VS: u32 = 0x08; // Version
pub const NVME_INTMS: u32 = 0x0C; // Interrupt Mask Set
pub const NVME_INTMC: u32 = 0x10; // Interrupt Mask Clear
pub const NVME_CC: u32 = 0x14; // Controller Configuration
pub const NVME_CSTS: u32 = 0x1C; // Controller Status
pub const NVME_NSSR: u32 = 0x20; // NVM Subsystem Reset
pub const NVME_AQA: u32 = 0x24; // Admin Queue Attributes
pub const NVME_ASQ: u32 = 0x28; // Admin Submission Queue Base Address
pub const NVME_ACQ: u32 = 0x30; // Admin Completion Queue Base Address
pub const NVME_CMBLOC: u32 = 0x38; // Controller Memory Buffer Location
pub const NVME_CMBSZ: u32 = 0x3C; // Controller Memory Buffer Size

// Queue Stride
pub const NVME_SQ_BASE: u32 = 0x1000; // Submission Queue Base
pub const NVME_CQ_BASE: u32 = 0x2000; // Completion Queue Base
pub const NVME_QUEUE_STRIDE: u32 = 0x1000; // Queue memory stride

// Queue Entry Sizes
pub const NVME_SQE_SIZE: u32 = 64;
pub const NVME_CQE_SIZE: u32 = 16;

// Default Queue Depths
pub const DEFAULT_QUEUE_DEPTH: u32 = 256;
pub const ADMIN_QUEUE_DEPTH: u32 = 64;

// ============================================================================
// NVMe Command Structures
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct NvmeCommandHeader {
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub namespace_id: u32,
    pub reserved: u64,
}

impl NvmeCommandHeader {
    pub fn new(opcode: u8, cmd_id: u16) -> Self {
        NvmeCommandHeader {
            opcode,
            flags: 0,
            command_id: cmd_id,
            namespace_id: 0,
            reserved: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NvmeCompletionEntry {
    pub command_specific: u32,
    pub reserved: u32,
    pub submission_queue_head_pointer: u16,
    pub submission_queue_id: u16,
    pub command_id: u16,
    pub status: u16,
}

impl NvmeCompletionEntry {
    pub fn status_code(&self) -> u16 {
        self.status >> 1 & 0xFF
    }

    pub fn is_success(&self) -> bool {
        self.status_code() == 0
    }
}

// ============================================================================
// NVMe Queue Pair
// ============================================================================

pub struct SubmissionQueue {
    base_address: u64,
    queue_depth: u32,
    tail_pointer: u16,
    entries: Vec<u64>,
}

impl SubmissionQueue {
    pub fn new(base: u64, depth: u32) -> Self {
        SubmissionQueue {
            base_address: base,
            queue_depth: depth,
            tail_pointer: 0,
            entries: Vec::with_capacity(depth as usize),
        }
    }

    pub fn submit_command(&mut self, cmd: u64) -> Result<u16, &'static str> {
        if self.entries.len() >= self.queue_depth as usize {
            return Err("Submission queue full");
        }

        self.entries.push(cmd);
        let id = self.tail_pointer;

        self.tail_pointer = (self.tail_pointer + 1) % (self.queue_depth as u16);
        Ok(id)
    }

    pub fn get_tail_pointer(&self) -> u16 {
        self.tail_pointer
    }

    pub fn entries_pending(&self) -> usize {
        self.entries.len()
    }
}

pub struct CompletionQueue {
    base_address: u64,
    queue_depth: u32,
    head_pointer: u16,
    phase_tag: bool,
    entries: Vec<NvmeCompletionEntry>,
}

impl CompletionQueue {
    pub fn new(base: u64, depth: u32) -> Self {
        CompletionQueue {
            base_address: base,
            queue_depth: depth,
            head_pointer: 0,
            phase_tag: false,
            entries: Vec::with_capacity(depth as usize),
        }
    }

    pub fn get_completion(&mut self) -> Option<NvmeCompletionEntry> {
        if self.entries.is_empty() {
            return None;
        }

        let entry = self.entries.remove(0);
        self.head_pointer = (self.head_pointer + 1) % (self.queue_depth as u16);

        Some(entry)
    }

    pub fn add_completion(&mut self, entry: NvmeCompletionEntry) {
        self.entries.push(entry);
    }

    pub fn get_head_pointer(&self) -> u16 {
        self.head_pointer
    }

    pub fn has_completions(&self) -> bool {
        !self.entries.is_empty()
    }
}

pub struct QueuePair {
    queue_id: u16,
    submission_queue: SubmissionQueue,
    completion_queue: CompletionQueue,
    next_command_id: AtomicU16,
}

impl QueuePair {
    pub fn new(
        id: u16,
        sq_base: u64,
        cq_base: u64,
        depth: u32,
    ) -> Self {
        QueuePair {
            queue_id: id,
            submission_queue: SubmissionQueue::new(sq_base, depth),
            completion_queue: CompletionQueue::new(cq_base, depth),
            next_command_id: AtomicU16::new(0),
        }
    }

    pub fn allocate_command_id(&self) -> u16 {
        self.next_command_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn submit_command(&mut self, cmd: u64) -> Result<u16, &'static str> {
        self.submission_queue.submit_command(cmd)
    }

    pub fn poll_completion(&mut self) -> Option<NvmeCompletionEntry> {
        self.completion_queue.get_completion()
    }

    pub fn has_completions(&self) -> bool {
        self.completion_queue.has_completions()
    }
}

// ============================================================================
// NVMe Namespace Information
// ============================================================================

#[derive(Debug, Clone)]
pub struct NvmeNamespace {
    pub namespace_id: u32,
    pub size_sectors: u64,
    pub sector_size: u32,
    pub features: u8,
}

impl NvmeNamespace {
    pub fn new(nsid: u32, size: u64, sector_size: u32) -> Self {
        NvmeNamespace {
            namespace_id: nsid,
            size_sectors: size,
            sector_size,
            features: 0,
        }
    }

    pub fn total_size_bytes(&self) -> u64 {
        self.size_sectors * (self.sector_size as u64)
    }
}

// ============================================================================
// NVMe Controller Driver
// ============================================================================

pub struct NvmeController {
    device_id: u16,
    pci_address: String,
    mmio_base: u64,
    mmio_size: u64,
    interrupt_line: u8,
    is_enabled: bool,
    admin_queue: QueuePair,
    io_queues: Vec<QueuePair>,
    namespaces: Vec<NvmeNamespace>,
    controller_memory_buffer_size: u32,
    max_queue_depth: u32,
    io_command_set_supported: bool,
    read_commands: AtomicU32,
    write_commands: AtomicU32,
}

impl NvmeController {
    pub fn new(device_id: u16, pci_addr: &str) -> Self {
        // Create admin queue pair
        let admin_queue = QueuePair::new(0, NVME_SQ_BASE as u64, NVME_CQ_BASE as u64, ADMIN_QUEUE_DEPTH);

        NvmeController {
            device_id,
            pci_address: pci_addr.to_string(),
            mmio_base: 0,
            mmio_size: 0,
            interrupt_line: 0,
            is_enabled: false,
            admin_queue,
            io_queues: Vec::new(),
            namespaces: Vec::new(),
            controller_memory_buffer_size: 0,
            max_queue_depth: DEFAULT_QUEUE_DEPTH,
            io_command_set_supported: true,
            read_commands: AtomicU32::new(0),
            write_commands: AtomicU32::new(0),
        }
    }

    pub fn init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str> {
        self.mmio_base = bar;
        self.mmio_size = size;

        // In real implementation, would:
        // 1. Read controller capabilities
        // 2. Configure controller
        // 3. Enable controller
        // 4. Initialize admin queue

        self.is_enabled = true;
        Ok(())
    }

    pub fn identify_controller(&mut self) -> Result<(), &'static str> {
        if !self.is_enabled {
            return Err("Controller not enabled");
        }

        // In real implementation:
        // 1. Allocate buffer for identify data
        // 2. Submit identify command to admin queue
        // 3. Wait for completion
        // 4. Parse controller properties

        Ok(())
    }

    pub fn identify_namespace(&mut self, namespace_id: u32) -> Result<NvmeNamespace, &'static str> {
        if !self.is_enabled {
            return Err("Controller not enabled");
        }

        // In real implementation:
        // 1. Submit identify namespace command
        // 2. Parse namespace properties
        // 3. Return namespace info

        // For now, create a dummy namespace
        Ok(NvmeNamespace::new(namespace_id, 1_000_000, 4096))
    }

    pub fn create_io_queue_pair(&mut self, queue_id: u16) -> Result<(), &'static str> {
        if self.io_queues.len() >= 32 {
            return Err("Too many I/O queues");
        }

        let sq_base = (NVME_SQ_BASE + (queue_id as u32) * NVME_QUEUE_STRIDE) as u64;
        let cq_base = (NVME_CQ_BASE + (queue_id as u32) * NVME_QUEUE_STRIDE) as u64;

        let queue = QueuePair::new(queue_id, sq_base, cq_base, DEFAULT_QUEUE_DEPTH);
        self.io_queues.push(queue);

        Ok(())
    }

    pub fn read_sectors(
        &mut self,
        _namespace_id: u32,
        _start_lba: u64,
        _num_sectors: u32,
    ) -> Result<u16, &'static str> {
        if !self.is_enabled {
            return Err("Controller not enabled");
        }

        if self.io_queues.is_empty() {
            return Err("No I/O queues");
        }

        self.read_commands.fetch_add(1, Ordering::SeqCst);

        // In real implementation, would submit READ command to I/O queue
        let cmd_id = self.io_queues[0].allocate_command_id();
        Ok(cmd_id)
    }

    pub fn write_sectors(
        &mut self,
        _namespace_id: u32,
        _start_lba: u64,
        _num_sectors: u32,
    ) -> Result<u16, &'static str> {
        if !self.is_enabled {
            return Err("Controller not enabled");
        }

        if self.io_queues.is_empty() {
            return Err("No I/O queues");
        }

        self.write_commands.fetch_add(1, Ordering::SeqCst);

        // In real implementation, would submit WRITE command to I/O queue
        let cmd_id = self.io_queues[0].allocate_command_id();
        Ok(cmd_id)
    }

    pub fn poll_completions(&mut self) -> Result<u32, &'static str> {
        let mut count = 0;

        if let Some(completion) = self.admin_queue.poll_completion() {
            if completion.is_success() {
                count += 1;
            }
        }

        for queue in &mut self.io_queues {
            while let Some(completion) = queue.poll_completion() {
                if completion.is_success() {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    pub fn get_stats(&self) -> (u32, u32) {
        (
            self.read_commands.load(Ordering::SeqCst),
            self.write_commands.load(Ordering::SeqCst),
        )
    }

    pub fn add_namespace(&mut self, namespace: NvmeNamespace) {
        self.namespaces.push(namespace);
    }

    pub fn get_namespaces(&self) -> &[NvmeNamespace] {
        &self.namespaces
    }
}

impl Default for NvmeController {
    fn default() -> Self {
        Self::new(0x0001, "0000:00:1f.0")
    }
}

// ============================================================================
// PciDriver Implementation
// ============================================================================

pub struct NvmePciDriver {
    controller: Option<Box<NvmeController>>,
}

impl NvmePciDriver {
    pub fn new() -> Self {
        NvmePciDriver { controller: None }
    }

    pub fn get_controller(&self) -> Option<&NvmeController> {
        self.controller.as_ref().map(|b| b.as_ref())
    }

    pub fn get_controller_mut(&mut self) -> Option<&mut NvmeController> {
        self.controller.as_mut().map(|b| b.as_mut())
    }
}

impl PciDriver for NvmePciDriver {
    fn probe(&mut self, device: &PciDeviceInfo) -> Result<bool, &'static str> {
        // Check if this is an NVMe device (class 0x01, subclass 0x08)
        if device.class_code != NVME_CLASS_MASS_STORAGE || device.subclass_code != NVME_SUBCLASS_NVM {
            return Ok(false);
        }

        // Device is NVMe, initialize driver
        let mut controller =
            Box::new(NvmeController::new(device.device_id, &device.address.sysfs_format()));

        // Extract MMIO BAR (typically BAR0)
        if let Some(ref bar) = device.bars[0] {
            controller.init_mmio(bar.address, bar.size)?;
        } else {
            return Err("No MMIO BAR found");
        }

        controller.interrupt_line = device.interrupt_line;

        self.controller = Some(controller);
        Ok(true)
    }

    fn remove(&mut self, _device: &PciDeviceInfo) -> Result<(), &'static str> {
        self.controller = None;
        Ok(())
    }

    fn name(&self) -> &str {
        "nvme"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvme_controller_creation() {
        let controller = NvmeController::new(0x0001, "0000:00:1f.0");
        assert_eq!(controller.device_id, 0x0001);
        assert!(!controller.is_enabled);
    }

    #[test]
    fn test_nvme_namespace_creation() {
        let ns = NvmeNamespace::new(1, 1_000_000, 4096);
        assert_eq!(ns.namespace_id, 1);
        assert_eq!(ns.total_size_bytes(), 1_000_000 * 4096);
    }

    #[test]
    fn test_submission_queue_operations() {
        let mut sq = SubmissionQueue::new(0x1000, 256);
        assert!(sq.submit_command(0x0001).is_ok());
        assert_eq!(sq.entries_pending(), 1);
    }

    #[test]
    fn test_completion_queue_operations() {
        let mut cq = CompletionQueue::new(0x2000, 256);
        let entry = NvmeCompletionEntry {
            command_specific: 0,
            reserved: 0,
            submission_queue_head_pointer: 0,
            submission_queue_id: 0,
            command_id: 0,
            status: 0, // Success
        };

        cq.add_completion(entry);
        assert!(cq.has_completions());

        let retrieved = cq.get_completion();
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().is_success());
    }

    #[test]
    fn test_queue_pair_creation() {
        let qp = QueuePair::new(0, 0x1000, 0x2000, 256);
        let cmd_id1 = qp.allocate_command_id();
        let cmd_id2 = qp.allocate_command_id();

        assert_ne!(cmd_id1, cmd_id2);
    }

    #[test]
    fn test_nvme_pci_driver() {
        let driver = NvmePciDriver::new();
        assert_eq!(driver.name(), "nvme");
        assert!(driver.get_controller().is_none());
    }
}
