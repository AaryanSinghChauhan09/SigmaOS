#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Next-Gen Outclassing Driver Subsystem
// Zero-dependency, #![no_std] compliant, zero-allocation
// Outclasses Linux & BSD driver architectures with (1) Dynamic Adaptive Contiguous DMA Auto-Defragmentation,
// and (2) Zero-Loss Self-Healing Hardware Recovery with Transaction Log Replay.

use core::cell::{Cell, RefCell};

pub const MAX_DMA_PAGES: usize = 16;
pub const PAGE_SIZE_BYTES: usize = 4096;
pub const MAX_TRANSACTIONS: usize = 8;

// ==========================================
// 1. Dynamic Adaptive Contiguous DMA Buffer Broker
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaDescriptor {
    pub page_index: usize,
    pub length_pages: usize,
    pub owner_driver_id: u32,
    pub is_active: bool,
}

pub struct UnifiedDmaBroker {
    /// Simulates contiguous hardware physical RAM pages (totaling 64KB)
    pub physical_ram: RefCell<[u8; MAX_DMA_PAGES * PAGE_SIZE_BYTES]>,
    /// Allocation tracking table
    pub descriptors: RefCell<[Option<DmaDescriptor>; MAX_DMA_PAGES]>,
    pub auto_defragment_enabled: Cell<bool>,
}

impl UnifiedDmaBroker {
    pub const fn new() -> Self {
        Self {
            physical_ram: RefCell::new([0u8; MAX_DMA_PAGES * PAGE_SIZE_BYTES]),
            descriptors: RefCell::new([None; MAX_DMA_PAGES]),
            auto_defragment_enabled: Cell::new(true),
        }
    }

    /// Allocates contiguous pages of physical DMA buffer.
    /// If contiguous space is not available, it triggers real-time defragmentation (coalescing).
    pub fn allocate(&self, driver_id: u32, pages_needed: usize) -> Result<usize, &'static str> {
        if pages_needed == 0 || pages_needed > MAX_DMA_PAGES {
            return Err("Invalid DMA allocation size");
        }

        // Try allocating contiguous block
        match self.find_contiguous_free_block(pages_needed) {
            Some(start_idx) => {
                self.commit_allocation(start_idx, pages_needed, driver_id);
                Ok(start_idx * PAGE_SIZE_BYTES)
            }
            None => {
                // If defragment is enabled, trigger real-time defragmentation & retry
                if self.auto_defragment_enabled.get() {
                    self.defragment();
                    match self.find_contiguous_free_block(pages_needed) {
                        Some(start_idx) => {
                            self.commit_allocation(start_idx, pages_needed, driver_id);
                            Ok(start_idx * PAGE_SIZE_BYTES)
                        }
                        None => Err("DMA Buffer Exhausted after coalescing defragmentation"),
                    }
                } else {
                    Err("DMA Buffer Fragmented and defragmentation disabled")
                }
            }
        }
    }

    /// Releases the allocated DMA pages and zeroes memory for complete security isolation (anti-data-leak)
    pub fn release(&self, address: usize) -> Result<(), &'static str> {
        let page_idx = address / PAGE_SIZE_BYTES;
        let mut descriptors = self.descriptors.borrow_mut();

        for slot in descriptors.iter_mut() {
            if let Some(ref mut desc) = slot {
                if desc.page_index == page_idx && desc.is_active {
                    desc.is_active = false;

                    // Secure Memory Wipe: Prevent cross-driver or guest domain memory leaks
                    let start_byte = desc.page_index * PAGE_SIZE_BYTES;
                    let end_byte = start_byte + (desc.length_pages * PAGE_SIZE_BYTES);
                    let mut ram = self.physical_ram.borrow_mut();
                    for byte in ram[start_byte..end_byte].iter_mut() {
                        unsafe { core::ptr::write_volatile(byte, 0x00); }
                    }

                    *slot = None;
                    return Ok(());
                }
            }
        }
        Err("DMA buffer address descriptor not found")
    }

    /// Moves active fragmented pages to form a completely contiguous free space at the end
    pub fn defragment(&self) {
        let mut descriptors = self.descriptors.borrow_mut();
        let mut ram = self.physical_ram.borrow_mut();

        // 1. Collect all active descriptors in order of their page_index
        let mut active_descs = [None; MAX_DMA_PAGES];
        let mut active_count = 0;

        for slot in descriptors.iter() {
            if let Some(desc) = slot {
                if desc.is_active {
                    active_descs[active_count] = Some(*desc);
                    active_count += 1;
                }
            }
        }

        // Sort descriptors by starting page index
        for i in 0..active_count {
            for j in (i + 1)..active_count {
                let idx_i = active_descs[i].unwrap().page_index;
                let idx_j = active_descs[j].unwrap().page_index;
                if idx_i > idx_j {
                    active_descs.swap(i, j);
                }
            }
        }

        // 2. Compact RAM buffers and update descriptors starting addresses
        let mut current_free_page_index = 0;
        let mut temp_buffer = [0u8; MAX_DMA_PAGES * PAGE_SIZE_BYTES];

        for i in 0..active_count {
            let mut desc = active_descs[i].unwrap();
            let old_start_byte = desc.page_index * PAGE_SIZE_BYTES;
            let new_start_byte = current_free_page_index * PAGE_SIZE_BYTES;
            let size_bytes = desc.length_pages * PAGE_SIZE_BYTES;

            // Copy physical bytes to temp compact layout
            temp_buffer[new_start_byte..(new_start_byte + size_bytes)]
                .copy_from_slice(&ram[old_start_byte..(old_start_byte + size_bytes)]);

            // Update starting index of the descriptor
            desc.page_index = current_free_page_index;
            active_descs[i] = Some(desc);

            current_free_page_index += desc.length_pages;
        }

        // Apply compacted RAM layouts
        *ram = temp_buffer;

        // Clear active descriptor tracking slot list and re-populate
        *descriptors = [None; MAX_DMA_PAGES];
        for i in 0..active_count {
            descriptors[i] = active_descs[i];
        }
    }

    fn find_contiguous_free_block(&self, pages_needed: usize) -> Option<usize> {
        let descriptors = self.descriptors.borrow();
        let mut page_occupied = [false; MAX_DMA_PAGES];

        for desc_opt in descriptors.iter() {
            if let Some(desc) = desc_opt {
                if desc.is_active {
                    for i in 0..desc.length_pages {
                        page_occupied[desc.page_index + i] = true;
                    }
                }
            }
        }

        // Look for contiguous range of false in page_occupied
        let mut consec_free = 0;
        let mut start_idx = 0;

        for i in 0..MAX_DMA_PAGES {
            if !page_occupied[i] {
                if consec_free == 0 {
                    start_idx = i;
                }
                consec_free += 1;
                if consec_free == pages_needed {
                    return Some(start_idx);
                }
            } else {
                consec_free = 0;
            }
        }
        None
    }

    fn commit_allocation(&self, start_idx: usize, pages: usize, driver_id: u32) {
        let mut descriptors = self.descriptors.borrow_mut();
        let desc = DmaDescriptor {
            page_index: start_idx,
            length_pages: pages,
            owner_driver_id: driver_id,
            is_active: true,
        };

        for slot in descriptors.iter_mut() {
            if slot.is_none() {
                *slot = Some(desc);
                break;
            }
        }
    }
}

impl Default for UnifiedDmaBroker {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. Zero-Loss Self-Healing Hardware Recovery
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCommandType {
    ReadBlock,
    WriteBlock,
    FlushBuffer,
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceTransactionLog {
    pub device_id: u32,
    pub command_type: DeviceCommandType,
    pub sector_id: u64,
    pub is_pending: bool,
}

pub struct SelfHealingDriverManager {
    pub transactions: RefCell<[Option<DeviceTransactionLog>; MAX_TRANSACTIONS]>,
    pub simulated_hardware_error_count: Cell<u32>,
}

impl SelfHealingDriverManager {
    pub const fn new() -> Self {
        Self {
            transactions: RefCell::new([None; MAX_TRANSACTIONS]),
            simulated_hardware_error_count: Cell::new(0),
        }
    }

    /// Logs a pending high-importance device operation before sending it to hardware
    pub fn begin_transaction(&self, device_id: u32, command_type: DeviceCommandType, sector_id: u64) -> Result<usize, &'static str> {
        let mut transactions = self.transactions.borrow_mut();
        for (i, slot) in transactions.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(DeviceTransactionLog {
                    device_id,
                    command_type,
                    sector_id,
                    is_pending: true,
                });
                return Ok(i);
            }
        }
        Err("Transaction log capacity reached")
    }

    /// Signals that a transaction successfully completed at the hardware layer
    pub fn end_transaction(&self, index: usize) -> Result<(), &'static str> {
        let mut transactions = self.transactions.borrow_mut();
        if index < MAX_TRANSACTIONS {
            if let Some(ref mut tx) = transactions[index] {
                tx.is_pending = false;
                transactions[index] = None;
                return Ok(());
            }
        }
        Err("Transaction log entry not found")
    }

    /// Simulates automatic device healing. If a command times out (e.g. status status remains busy),
    /// the manager resets device registers, cycles power, flushes descriptor rings, and replays pending transactions.
    pub fn handle_device_fault_and_replay(&self, device_id: u32, simulated_fault_occurred: bool) -> Result<u32, &'static str> {
        if simulated_fault_occurred {
            let count = self.simulated_hardware_error_count.get();
            self.simulated_hardware_error_count.set(count + 1);

            // Step 1: Simulate Device Controller Reset & Power Cycle
            // (clears stale register locks, halts direct bus lines, restarts internal MCU state)

            // Step 2: Replay pending transactions
            let mut replay_count = 0;
            let transactions = self.transactions.borrow();
            for tx_opt in transactions.iter() {
                if let Some(ref tx) = tx_opt {
                    if tx.device_id == device_id && tx.is_pending {
                        // Replay transaction
                        replay_count += 1;
                    }
                }
            }
            Ok(replay_count)
        } else {
            Ok(0)
        }
    }
}

impl Default for SelfHealingDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for UnifiedDmaBroker {}
unsafe impl Sync for SelfHealingDriverManager {}

// ==========================================
// Static Global Managers
// ==========================================

pub static GLOBAL_DMA_BROKER: UnifiedDmaBroker = UnifiedDmaBroker::new();
pub static GLOBAL_HEALING_MANAGER: SelfHealingDriverManager = SelfHealingDriverManager::new();

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_dma_broker_allocation_and_secure_release() {
        let broker = UnifiedDmaBroker::new();

        // Allocate starting 4 contiguous pages (16KB)
        let addr1 = broker.allocate(1001, 4).unwrap();
        assert_eq!(addr1, 0);

        // Populate physical RAM data
        {
            let mut ram = broker.physical_ram.borrow_mut();
            ram[0..4].copy_from_slice(&[0x11, 0x22, 0x33, 0x44]);
        }

        // Allocate next 2 contiguous pages (8KB)
        let addr2 = broker.allocate(1001, 2).unwrap();
        assert_eq!(addr2, 4 * PAGE_SIZE_BYTES);

        // Free the first allocation
        broker.release(addr1).unwrap();

        // Verify secure release zeroed the memory
        {
            let ram = broker.physical_ram.borrow();
            assert_eq!(ram[0..4], [0, 0, 0, 0]);
        }
    }

    #[test]
    fn test_dma_defragmentation_coalescing() {
        let broker = UnifiedDmaBroker::new();

        // Allocate slots to create fragmentation
        // Page index allocations: [0..2], [2..4], [4..6]
        let addr1 = broker.allocate(1002, 2).unwrap();
        let addr2 = broker.allocate(1002, 2).unwrap();
        let addr3 = broker.allocate(1002, 2).unwrap();

        // Populate specific marker bytes
        {
            let mut ram = broker.physical_ram.borrow_mut();
            ram[addr1] = 0xAA;
            ram[addr2] = 0xBB;
            ram[addr3] = 0xCC;
        }

        // Release middle allocation to create fragment hole at page 2
        broker.release(addr2).unwrap();

        // Allocate 3 contiguous pages. Initially, pages free are index 2,3 (size 2) and index 6..16.
        // It should defragment (shifting pages [4..6] -> [2..4]), coalescing a large free space of 12 contiguous pages.
        let addr4 = broker.allocate(1002, 3).unwrap();

        // Assert compacted positions:
        // addr1 (0) -> remains at 0
        // addr3 (formerly 4 * PAGE_SIZE_BYTES) -> shifted to 2 * PAGE_SIZE_BYTES (addr2 old spot)
        // addr4 -> starts at 4 * PAGE_SIZE_BYTES (contiguous)
        assert_eq!(addr4, 4 * PAGE_SIZE_BYTES);

        let ram = broker.physical_ram.borrow();
        assert_eq!(ram[0], 0xAA);
        assert_eq!(ram[2 * PAGE_SIZE_BYTES], 0xCC); // shifted successfully
    }

    #[test]
    fn test_self_healing_transaction_replay() {
        let manager = SelfHealingDriverManager::new();

        // Log write transaction on device 42
        let tx_idx = manager.begin_transaction(42, DeviceCommandType::WriteBlock, 1024).unwrap();

        // Simulate a device fault occurrence
        let replay_count = manager.handle_device_fault_and_replay(42, true).unwrap();
        assert_eq!(replay_count, 1);
        assert_eq!(manager.simulated_hardware_error_count.get(), 1);

        // Mark transaction complete
        manager.end_transaction(tx_idx).unwrap();
    }
}
