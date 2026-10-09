// SPDX-License-Identifier: MIT
// SigmaOS NVMe Storage Driver
// Supports NVMe 1.0+ SSDs with queue pair and completion queue management

use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use std::boxed::Box;
use std::string::String;
use std::vec::Vec;

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

// PRP (Physical Region Page) Constants
pub const NVME_PAGE_SIZE: u64 = 4096;
pub const NVME_MAX_PRP_ENTRIES: usize = 512;
pub const NVME_PRP_LIST_PAGE_SIZE: usize = NVME_MAX_PRP_ENTRIES * 8; // 8 bytes per PRP entry

// Controller Configuration bits
pub const NVME_CC_ENABLE: u32 = 1 << 0;
pub const NVME_CC_CSS_NVM: u32 = 0 << 4;
pub const NVME_CC_MPS_SHIFT: u32 = 7;
pub const NVME_CC_MPS_4K: u32 = 0 << NVME_CC_MPS_SHIFT;
pub const NVME_CC_SHN_SHIFT: u32 = 8;
pub const NVME_CC_SHN_NONE: u32 = 0 << NVME_CC_SHN_SHIFT;
pub const NVME_CC_IOCQES_SHIFT: u32 = 20;
pub const NVME_CC_IOSQES_SHIFT: u32 = 16;

// Controller Status bits
pub const NVME_CSTS_RDY: u32 = 1 << 0;
pub const NVME_CSTS_CFS: u32 = 1 << 1;
pub const NVME_CSTS_SHST_SHIFT: u32 = 2;
pub const NVME_CSTS_SHST_OCCURRING: u32 = 1 << NVME_CSTS_SHST_SHIFT;

// Admin commands
pub const NVME_ADMIN_IDENTIFY: u8 = 0x06;
pub const NVME_ADMIN_GET_LOG_PAGE: u8 = 0x02;
pub const NVME_ADMIN_CREATE_IO_SQ: u8 = 0x01;
pub const NVME_ADMIN_CREATE_IO_CQ: u8 = 0x05;
pub const NVME_ADMIN_DELETE_IO_SQ: u8 = 0x00;
pub const NVME_ADMIN_DELETE_IO_CQ: u8 = 0x04;

// NVM commands
pub const NVME_CMD_READ: u8 = 0x02;
pub const NVME_CMD_WRITE: u8 = 0x01;
pub const NVME_CMD_FLUSH: u8 = 0x00;

// ============================================================================
// NVMe Command Structures
// ============================================================================

/// PRP (Physical Region Page) Entry - points to a physical page
#[derive(Debug, Clone, Copy)]
pub struct PrpEntry {
    pub address: u64,
}

impl PrpEntry {
    pub fn new(addr: u64) -> Self {
        Self { address: addr }
    }

    pub fn is_page_aligned(&self) -> bool {
        self.address % NVME_PAGE_SIZE == 0
    }
}

/// PRP List - used for multi-page transfers
#[derive(Debug, Clone)]
pub struct PrpList {
    entries: Vec<PrpEntry>,
    physical_address: u64,
}

impl PrpList {
    pub fn new(physical_addr: u64) -> Self {
        Self {
            entries: Vec::with_capacity(NVME_MAX_PRP_ENTRIES),
            physical_address: physical_addr,
        }
    }

    pub fn add_entry(&mut self, prp: PrpEntry) -> Result<(), &'static str> {
        if self.entries.len() >= NVME_MAX_PRP_ENTRIES {
            return Err("PRP list full");
        }
        if !prp.is_page_aligned() {
            return Err("PRP not page-aligned");
        }
        self.entries.push(prp);
        Ok(())
    }

    pub fn get_physical_address(&self) -> u64 {
        self.physical_address
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// NVMe Submission Queue Entry (64 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NvmeSubmissionQueueEntry {
    pub dword0: u32,  // CDW0: command-specific
    pub dword1: u32,  // CDW1: namespace ID
    pub dword2: u32,  // CDW2: command-specific
    pub dword3: u32,  // CDW3: command-specific
    pub dword4: u32,  // CDW4: command-specific
    pub dword5: u32,  // CDW5: command-specific
    pub dword6: u32,  // CDW6: command-specific
    pub dword7: u32,  // CDW7: command-specific
    pub dword8: u32,  // CDW8: command-specific
    pub dword9: u32,  // CDW9: command-specific
    pub dword10: u32, // CDW10: command-specific
    pub dword11: u32, // CDW11: command-specific
    pub dword12: u32, // CDW12: command-specific
    pub dword13: u32, // CDW13: command-specific
    pub dword14: u32, // CDW14: command-specific
    pub dword15: u32, // CDW15: command-specific
}

impl NvmeSubmissionQueueEntry {
    pub fn new() -> Self {
        Self {
            dword0: 0,
            dword1: 0,
            dword2: 0,
            dword3: 0,
            dword4: 0,
            dword5: 0,
            dword6: 0,
            dword7: 0,
            dword8: 0,
            dword9: 0,
            dword10: 0,
            dword11: 0,
            dword12: 0,
            dword13: 0,
            dword14: 0,
            dword15: 0,
        }
    }

    pub fn set_prp1(&mut self, addr: u64) {
        self.dword6 = (addr & 0xFFFFFFFF) as u32;
        self.dword7 = ((addr >> 32) & 0xFFFFFFFF) as u32;
    }

    pub fn set_prp2(&mut self, addr: u64) {
        self.dword8 = (addr & 0xFFFFFFFF) as u32;
        self.dword9 = ((addr >> 32) & 0xFFFFFFFF) as u32;
    }

    pub fn set_slba(&mut self, lba: u64) {
        self.dword10 = (lba & 0xFFFFFFFF) as u32;
        self.dword11 = ((lba >> 32) & 0xFFFFFFFF) as u32;
    }

    pub fn set_nlb(&mut self, nlb: u16) {
        // NLB is 0-based, so number of blocks = nlb + 1
        self.dword12 = (nlb as u32) & 0xFFFF;
    }

    pub fn set_command_id(&mut self, cid: u16) {
        self.dword0 = (self.dword0 & 0xFFFF0000) | (cid as u32);
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.dword0 = (self.dword0 & 0xFFFFFF00) | (opcode as u32);
    }
}

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
    pub fn new(id: u16, sq_base: u64, cq_base: u64, depth: u32) -> Self {
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
    prp_lists: Vec<PrpList>,
    next_prp_list_addr: u64,
}

impl NvmeController {
    pub fn new(device_id: u16, pci_addr: &str) -> Self {
        // Create admin queue pair
        let admin_queue = QueuePair::new(
            0,
            NVME_SQ_BASE as u64,
            NVME_CQ_BASE as u64,
            ADMIN_QUEUE_DEPTH,
        );

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
            prp_lists: Vec::new(),
            next_prp_list_addr: 0x10000000, // Start PRP lists at 256MB physical
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

    /// Allocate a PRP list for multi-page transfers
    pub fn allocate_prp_list(&mut self) -> Result<u64, &'static str> {
        let prp_list = PrpList::new(self.next_prp_list_addr);
        let physical_addr = prp_list.get_physical_address();

        self.prp_lists.push(prp_list);
        self.next_prp_list_addr += NVME_PRP_LIST_PAGE_SIZE as u64;

        Ok(physical_addr)
    }

    /// Get a mutable reference to a PRP list by physical address
    pub fn get_prp_list_mut(&mut self, physical_addr: u64) -> Option<&mut PrpList> {
        self.prp_lists
            .iter_mut()
            .find(|pl| pl.get_physical_address() == physical_addr)
    }

    /// Build PRP entries for a data buffer
    pub fn build_prp_entries(
        &mut self,
        data_addr: u64,
        data_size: u64,
    ) -> Result<(u64, u64), &'static str> {
        // For transfers up to 2 pages (8KB), we can use PRP1 and PRP2 directly
        // For larger transfers, we need a PRP list

        let num_pages = (data_size + NVME_PAGE_SIZE - 1) / NVME_PAGE_SIZE;

        if num_pages <= 2 {
            // Use PRP1 and PRP2 directly
            let prp1 = data_addr;
            let prp2 = if num_pages > 1 {
                data_addr + NVME_PAGE_SIZE
            } else {
                0
            };
            Ok((prp1, prp2))
        } else {
            // Need a PRP list
            let prp_list_addr = self.allocate_prp_list()?;
            let prp_list = self
                .get_prp_list_mut(prp_list_addr)
                .ok_or("PRP list not found")?;

            // Clear existing entries
            prp_list.entries.clear();

            // Add PRP entries for each page
            for i in 0..num_pages {
                let page_addr = data_addr + (i * NVME_PAGE_SIZE);
                let prp = PrpEntry::new(page_addr);
                prp_list.add_entry(prp)?;
            }

            // PRP1 points to first page, PRP2 points to PRP list
            let prp1 = data_addr;
            let prp2 = prp_list_addr;
            Ok((prp1, prp2))
        }
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
        namespace_id: u32,
        start_lba: u64,
        num_sectors: u32,
        data_addr: u64,
        data_size: u64,
    ) -> Result<u16, &'static str> {
        if !self.is_enabled {
            return Err("Controller not enabled");
        }

        if self.io_queues.is_empty() {
            return Err("No I/O queues");
        }

        self.read_commands.fetch_add(1, Ordering::SeqCst);

        // Build PRP entries for data buffer
        let (prp1, prp2) = self.build_prp_entries(data_addr, data_size)?;

        // Build proper NVMe submission queue entry
        let cmd_id = self.io_queues[0].allocate_command_id();
        let mut sqe = NvmeSubmissionQueueEntry::new();
        sqe.set_opcode(NVME_CMD_READ);
        sqe.set_command_id(cmd_id);
        sqe.set_slba(start_lba);
        sqe.set_nlb((num_sectors - 1) as u16); // NLB is 0-based
        sqe.set_prp1(prp1);
        sqe.set_prp2(prp2);

        // Convert SQE to u64 array for submission
        let sqe_ptr = &sqe as *const NvmeSubmissionQueueEntry as *const u64;
        let sqe_words = unsafe { core::slice::from_raw_parts(sqe_ptr, 8) };

        // Submit command to I/O queue (submit as individual words)
        for &word in sqe_words {
            self.io_queues[0].submit_command(word)?;
        }

        // Ring submission queue doorbell with proper MMIO
        let sq_tail = self.io_queues[0].submission_queue.get_tail_pointer();
        let doorbell_offset = NVME_SQ_BASE + (0 * 2 * 4); // Queue 0, stride 4 bytes
        unsafe {
            core::ptr::write_volatile(
                (self.mmio_base + doorbell_offset as u64) as *mut u32,
                sq_tail as u32,
            );
        }

        // Poll for completion
        let mut timeout = 10000;
        while timeout > 0 {
            if let Some(completion) = self.io_queues[0].poll_completion() {
                if completion.is_success() {
                    // Ring completion queue doorbell
                    let cq_head = self.io_queues[0].completion_queue.get_head_pointer();
                    let cq_doorbell_offset = NVME_CQ_BASE + (0 * 2 * 4);
                    unsafe {
                        core::ptr::write_volatile(
                            (self.mmio_base + cq_doorbell_offset as u64) as *mut u32,
                            cq_head as u32,
                        );
                    }
                    return Ok(cmd_id);
                } else {
                    return Err("NVMe read command failed");
                }
            }
            core::hint::spin_loop();
            timeout -= 1;
        }

        Err("NVMe read timeout")
    }

    pub fn write_sectors(
        &mut self,
        namespace_id: u32,
        start_lba: u64,
        num_sectors: u32,
        data_addr: u64,
        data_size: u64,
    ) -> Result<u16, &'static str> {
        if !self.is_enabled {
            return Err("Controller not enabled");
        }

        if self.io_queues.is_empty() {
            return Err("No I/O queues");
        }

        self.write_commands.fetch_add(1, Ordering::SeqCst);

        // Build PRP entries for data buffer
        let (prp1, prp2) = self.build_prp_entries(data_addr, data_size)?;

        // Build proper NVMe submission queue entry
        let cmd_id = self.io_queues[0].allocate_command_id();
        let mut sqe = NvmeSubmissionQueueEntry::new();
        sqe.set_opcode(NVME_CMD_WRITE);
        sqe.set_command_id(cmd_id);
        sqe.set_slba(start_lba);
        sqe.set_nlb((num_sectors - 1) as u16); // NLB is 0-based
        sqe.set_prp1(prp1);
        sqe.set_prp2(prp2);

        // Convert SQE to u64 array for submission
        let sqe_ptr = &sqe as *const NvmeSubmissionQueueEntry as *const u64;
        let sqe_words = unsafe { core::slice::from_raw_parts(sqe_ptr, 8) };

        // Submit command to I/O queue (submit as individual words)
        for &word in sqe_words {
            self.io_queues[0].submit_command(word)?;
        }

        // Ring submission queue doorbell with proper MMIO
        let sq_tail = self.io_queues[0].submission_queue.get_tail_pointer();
        let doorbell_offset = NVME_SQ_BASE + (0 * 2 * 4);
        unsafe {
            core::ptr::write_volatile(
                (self.mmio_base + doorbell_offset as u64) as *mut u32,
                sq_tail as u32,
            );
        }

        // Poll for completion
        let mut timeout = 10000;
        while timeout > 0 {
            if let Some(completion) = self.io_queues[0].poll_completion() {
                if completion.is_success() {
                    // Ring completion queue doorbell
                    let cq_head = self.io_queues[0].completion_queue.get_head_pointer();
                    let cq_doorbell_offset = NVME_CQ_BASE + (0 * 2 * 4);
                    unsafe {
                        core::ptr::write_volatile(
                            (self.mmio_base + cq_doorbell_offset as u64) as *mut u32,
                            cq_head as u32,
                        );
                    }
                    return Ok(cmd_id);
                } else {
                    return Err("NVMe write command failed");
                }
            }
            core::hint::spin_loop();
            timeout -= 1;
        }

        Err("NVMe write timeout")
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
        if device.class_code != NVME_CLASS_MASS_STORAGE || device.subclass_code != NVME_SUBCLASS_NVM
        {
            return Ok(false);
        }

        // Device is NVMe, initialize driver
        let mut controller = Box::new(NvmeController::new(
            device.device_id,
            &device.address.sysfs_format(),
        ));

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

    #[test]
    fn test_prp_entry() {
        let prp = PrpEntry::new(0x1000);
        assert_eq!(prp.address, 0x1000);
        assert!(prp.is_page_aligned());

        let prp_unaligned = PrpEntry::new(0x1001);
        assert!(!prp_unaligned.is_page_aligned());
    }

    #[test]
    fn test_prp_list() {
        let mut prp_list = PrpList::new(0x20000000);
        assert_eq!(prp_list.get_physical_address(), 0x20000000);
        assert!(prp_list.is_empty());

        let prp = PrpEntry::new(0x1000);
        assert!(prp_list.add_entry(prp).is_ok());
        assert_eq!(prp_list.len(), 1);
        assert!(!prp_list.is_empty());
    }

    #[test]
    fn test_prp_list_full() {
        let mut prp_list = PrpList::new(0x20000000);

        // Add maximum entries
        for i in 0..NVME_MAX_PRP_ENTRIES {
            let prp = PrpEntry::new((i as u64 + 1) * NVME_PAGE_SIZE);
            assert!(prp_list.add_entry(prp).is_ok());
        }

        // Should fail when full
        let prp = PrpEntry::new(0x1000);
        assert!(prp_list.add_entry(prp).is_err());
    }

    #[test]
    fn test_nvme_submission_queue_entry() {
        let mut sqe = NvmeSubmissionQueueEntry::new();

        sqe.set_opcode(NVME_CMD_READ);
        sqe.set_command_id(42);
        sqe.set_slba(0x1000);
        sqe.set_nlb(15); // 16 sectors (0-based)
        sqe.set_prp1(0x2000);
        sqe.set_prp2(0x3000);

        assert_eq!(sqe.dword0 & 0xFF, NVME_CMD_READ as u32);
        assert_eq!(sqe.dword0 & 0xFFFF, 42);
    }

    #[test]
    fn test_nvme_controller_prp_allocation() {
        let mut controller = NvmeController::new(0x0001, "0000:00:1f.0");

        let prp_addr = controller.allocate_prp_list();
        assert!(prp_addr.is_ok());
        assert_eq!(prp_addr.unwrap(), 0x10000000);

        let prp_addr2 = controller.allocate_prp_list();
        assert!(prp_addr2.is_ok());
        assert_eq!(
            prp_addr2.unwrap(),
            0x10000000 + NVME_PRP_LIST_PAGE_SIZE as u64
        );
    }

    #[test]
    fn test_nvme_controller_build_prp_small() {
        let mut controller = NvmeController::new(0x0001, "0000:00:1f.0");

        // Small transfer (1 page)
        let (prp1, prp2) = controller.build_prp_entries(0x5000, 4096).unwrap();
        assert_eq!(prp1, 0x5000);
        assert_eq!(prp2, 0);

        // Medium transfer (2 pages)
        let (prp1, prp2) = controller.build_prp_entries(0x6000, 8192).unwrap();
        assert_eq!(prp1, 0x6000);
        assert_eq!(prp2, 0x6000 + 4096);
    }

    #[test]
    fn test_nvme_controller_build_prp_large() {
        let mut controller = NvmeController::new(0x0001, "0000:00:1f.0");

        // Large transfer (needs PRP list)
        let (prp1, prp2) = controller.build_prp_entries(0x8000, 16384).unwrap();
        assert_eq!(prp1, 0x8000);
        assert_eq!(prp2, 0x10000000); // PRP list address
    }
}
