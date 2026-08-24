// SigmaOS Kernel Memory Management
// Implements buddy allocator and paging with zero std dependency

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
use std::collections::HashMap;
#[cfg(not(test))]
use crate::klib::HashMap;

/// Memory page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Memory block
#[derive(Debug, Clone, Copy)]
pub struct MemoryBlock {
    pub addr: NonNull<u8>,
    pub size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    Paged,    // Swappable (virtual pages can be swapped out to disk)
    NonPaged, // Always resident in physical memory (for critical drivers and ISRs)
}

#[derive(Debug, Clone)]
pub struct PoolBlock {
    pub addr: usize,
    pub size: usize,
    pub pool_type: PoolType,
    pub tag: [u8; 4], // 4-character driver tag (standard Windows NT Pool Tag, e.g. "File")
}

pub struct KernelPoolManager {
    pub paged_pool: Vec<PoolBlock>,
    pub non_paged_pool: Vec<PoolBlock>,
    pub total_paged_bytes: usize,
    pub total_non_paged_bytes: usize,
}

impl KernelPoolManager {
    pub fn new() -> Self {
        Self {
            paged_pool: Vec::new(),
            non_paged_pool: Vec::new(),
            total_paged_bytes: 0,
            total_non_paged_bytes: 0,
        }
    }

    /// Allocate a block from the specific kernel pool with a pool tag (Inspired by Windows NT ExAllocatePoolWithTag)
    pub fn allocate_pool(&mut self, pool_type: PoolType, size: usize, tag: &[u8; 4]) -> Result<PoolBlock, &'static str> {
        if size == 0 {
            return Err("Cannot allocate 0-byte pool block");
        }

        // Emulate allocating pool virtual address range
        let addr = match pool_type {
            PoolType::Paged => 0xD000_0000 + self.total_paged_bytes,
            PoolType::NonPaged => 0xF000_0000 + self.total_non_paged_bytes,
        };

        let block = PoolBlock {
            addr,
            size,
            pool_type,
            tag: *tag,
        };

        match pool_type {
            PoolType::Paged => {
                self.paged_pool.push(block.clone());
                self.total_paged_bytes += size;
            }
            PoolType::NonPaged => {
                self.non_paged_pool.push(block.clone());
                self.total_non_paged_bytes += size;
            }
        }

        println!(
            "Windows NT Pool Alloc: Allocated {:?} pool block of {} bytes with tag '{}' at address 0x{:X}",
            pool_type, size, core::str::from_utf8(tag).unwrap_or("????"), addr
        );

        Ok(block)
    }

    /// Free a block from the kernel pool (Inspired by Windows NT ExFreePool)
    pub fn free_pool(&mut self, addr: usize) -> Result<(), &'static str> {
        if let Some(pos) = self.paged_pool.iter().position(|b| b.addr == addr) {
            let block = self.paged_pool.remove(pos);
            self.total_paged_bytes -= block.size;
            Ok(())
        } else if let Some(pos) = self.non_paged_pool.iter().position(|b| b.addr == addr) {
            let block = self.non_paged_pool.remove(pos);
            self.total_non_paged_bytes -= block.size;
            Ok(())
        } else {
            Err("Invalid pool address; double free or corruption detected")
        }
    }
}

impl Default for KernelPoolManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Zone {
    pub present_pages: u64,
}

pub struct Page {
    pub flags: AtomicUsize,
    pub count: AtomicUsize,
    pub mapping: Option<usize>,
    pub index: u64,
    pub private: Option<usize>,
    pub zone: Option<*const Zone>,
}

impl Page {
    pub fn dec_ref(&self) -> bool {
        self.count.fetch_sub(1, Ordering::SeqCst) == 1
    }
}

/// Buddy allocator for memory management
#[derive(Debug, Clone)]
pub struct BuddyAllocatorCheckpoint {
    pub free_lists: [Vec<MemoryBlock>; 12],
}

pub struct BuddyAllocator {
    pub free_lists: [Vec<MemoryBlock>; 12],
    pub free_pages: usize,
    pub total_pages: usize,
    pub zones: Vec<Zone>,
}

impl BuddyAllocator {
    pub fn new() -> Self {
        Self {
            free_lists: Default::default(),
            free_pages: 0,
            total_pages: 0,
            zones: Vec::new(),
        }
    }

    pub fn initialize_memory(&mut self, base_addr: usize, size: usize) {
        let pages = size / PAGE_SIZE;
        let order = self.calculate_order(pages);

        if order < 12 {
            if let Some(addr) = NonNull::new(base_addr as *mut u8) {
                let block = MemoryBlock { addr, size };
                self.free_lists[order].push(block);
            }
        }
    }

    /// Create a checkpoint of the allocator's current free list state (Phase 1.1)
    pub fn create_checkpoint(&self) -> [Vec<MemoryBlock>; 12] {
        let mut checkpoint: [Vec<MemoryBlock>; 12] = Default::default();
        for order in 0..12 {
            for block in &self.free_lists[order] {
                checkpoint[order].push(*block);
            }
        }
        checkpoint
    }

    /// Restore the allocator to a previously checkpointed state to recover from crash exceptions (Phase 1.1)
    pub fn restore_checkpoint(&mut self, checkpoint: [Vec<MemoryBlock>; 12]) {
        self.free_lists = checkpoint;
    }

    pub fn get_free_memory(&self) -> usize {
        self.free_lists
            .iter()
            .enumerate()
            .map(|(order, blocks)| blocks.len() * (1 << order) * PAGE_SIZE)
            .sum()
    }

    pub fn get_total_memory(&self) -> usize {
        self.free_lists
            .iter()
            .enumerate()
            .map(|(order, blocks)| blocks.len() * (1 << order) * PAGE_SIZE)
            .sum()
    }

    pub fn allocate(&mut self, size: usize) -> Option<MemoryBlock> {
        // Prevent integer overflow in size calculation
        if size == 0 || size > usize::MAX - PAGE_SIZE + 1 {
            return None;
        }

        let pages = size.div_ceil(PAGE_SIZE);
        let order = self.calculate_order(pages);

        // Find smallest block that can satisfy request
        for current_order in order..12 {
            if let Some(block) = self.get_block(current_order) {
                // Split block if necessary
                if current_order > order {
                    let split_block = self.split_block(block, current_order - order)?;
                    return Some(split_block);
                }
                return Some(block);
            }
        }

        None
    }

    pub fn deallocate(&mut self, block: MemoryBlock) {
        let pages = block.size / PAGE_SIZE;
        let order = self.calculate_order(pages);

        // Try to merge with buddy
        match self.try_merge(block, order) {
            Ok(merged_block) => self.deallocate(merged_block),
            Err(original_block) => self.free_lists[order].push(original_block),
        }
    }

    fn calculate_order(&self, pages: usize) -> usize {
        // Bolt Optimization: Replace O(n) linear search loop with O(1) branchless bitwise operations.
        // On modern hardware, next_power_of_two() and trailing_zeros() map directly to specialized
        // CPU instructions (e.g., LZCNT/TZCNT/BSR), enabling nanosecond-level execution speeds and supporting HW acceleration.
        if pages <= 1 {
            0
        } else {
            let next_pow = pages.next_power_of_two();
            next_pow.trailing_zeros() as usize
        }
    }

    fn get_block(&mut self, order: usize) -> Option<MemoryBlock> {
        if order < 12 {
            self.free_lists[order].pop()
        } else {
            None
        }
    }

    fn split_block(&mut self, block: MemoryBlock, target_order: usize) -> Option<MemoryBlock> {
        let mut current_block = block;
        let mut current_order = self.calculate_order(current_block.size / PAGE_SIZE);

        while current_order > target_order {
            current_order -= 1;
            let half_size = current_block.size / 2;
            let addr = current_block.addr.as_ptr() as usize + half_size;

            let buddy = MemoryBlock {
                addr: NonNull::new(addr as *mut u8)?,
                size: half_size,
            };

            current_block.size = half_size;
            self.free_lists[current_order].push(buddy);
        }

        Some(current_block)
    }

    fn try_merge(&mut self, block: MemoryBlock, order: usize) -> Result<MemoryBlock, MemoryBlock> {
        if order >= 11 {
            return Err(block); // Maximum order
        }

        let block_addr = block.addr.as_ptr() as usize;
        // Calculate buddy address by XORing with block size (standard buddy system)
        let buddy_addr = block_addr ^ block.size;
        let buddy_size = block.size * 2;

        // Find buddy in free list
        if let Some(pos) = self.free_lists[order]
            .iter()
            .position(|b| b.addr.as_ptr() as usize == buddy_addr && b.size == block.size)
        {
            let _buddy = self.free_lists[order].remove(pos);

            // Merge blocks
            let merged_addr = if block_addr < buddy_addr {
                block_addr
            } else {
                buddy_addr
            };

            if let Some(non_null) = NonNull::new(merged_addr as *mut u8) {
                Ok(MemoryBlock {
                    addr: non_null,
                    size: buddy_size,
                })
            } else {
                Err(block)
            }
        } else {
            Err(block)
        }
    }
}

impl Default for BuddyAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Page table entry flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFlags(pub u64);

impl PageFlags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const CACHE_DISABLE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE_PAGE: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
    pub const NO_EXECUTE: u64 = 1 << 63;
}

/// A standard 4KB page table entry
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry(u64);

impl Default for PageTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl PageTableEntry {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set_addr(&mut self, addr: u64, flags: PageFlags) {
        // Clear everything but flags, and mask the address to align with 4KB
        self.0 = (addr & 0x0000_00FF_FFFF_F000) | flags.0;
    }

    pub fn get_addr(&self) -> u64 {
        self.0 & 0x0000_00FF_FFFF_F000
    }

    pub fn flags(&self) -> PageFlags {
        PageFlags(self.0 & 0xFFF0_0000_0000_0FFF)
    }

    pub fn is_present(&self) -> bool {
        (self.0 & PageFlags::PRESENT) != 0
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

/// A standard Page Table (containing 512 entries on x86_64)
#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: [PageTableEntry::new(); 512],
        }
    }
}

/// Virtual Memory Manager (VMM) handling paging
pub struct VirtualMemoryManager {
    pub root_directory: NonNull<PageTable>,
    pub buddy_allocator: BuddyAllocator,
    pub page_ref_counts: HashMap<u64, u32>, // physical frame addr -> reference count (for Copy-on-Write)
    pub shadow_snapshots: HashMap<u64, String>, // virtual_addr -> snapshot copy (for snapshot isolation)
}

impl VirtualMemoryManager {
    pub fn new(root_directory: NonNull<PageTable>) -> Self {
        Self {
            root_directory,
            buddy_allocator: BuddyAllocator::new(),
            page_ref_counts: HashMap::new(),
            shadow_snapshots: HashMap::new(),
        }
    }

    pub fn with_allocator(root_directory: NonNull<PageTable>, allocator: BuddyAllocator) -> Self {
        Self {
            root_directory,
            buddy_allocator: allocator,
            page_ref_counts: HashMap::new(),
            shadow_snapshots: HashMap::new(),
        }
    }

    /// Allocate pages using buddy allocator (wires alloc_pages to VMM)
    pub fn alloc_pages(&mut self, num_pages: usize) -> Option<MemoryBlock> {
        let size = num_pages * PAGE_SIZE;
        self.buddy_allocator.allocate(size)
    }

    /// Free pages using buddy allocator (wires free_pages to VMM)
    pub fn free_pages(&mut self, block: MemoryBlock) {
        self.buddy_allocator.deallocate(block);
    }

    /// Translates a virtual address into a physical address
    pub fn translate(&self, virtual_addr: u64) -> Option<u64> {
        // Mock translation logic for SigmaOS OOP structure
        // In a real x86_64 system, we would walk PML4 -> PDPT -> PD -> PT
        let pt_index = (virtual_addr >> 12) & 0x1FF;
        let root = unsafe { self.root_directory.as_ref() };

        let entry = &root.entries[pt_index as usize];
        if entry.is_present() {
            Some(entry.get_addr() + (virtual_addr & 0xFFF))
        } else {
            None
        }
    }

    /// Maps a virtual page to a physical frame
    pub fn map_page(
        &mut self,
        virtual_addr: u64,
        physical_addr: u64,
        flags: PageFlags,
    ) -> Result<(), &'static str> {
        let pt_index = (virtual_addr >> 12) & 0x1FF;
        let root = unsafe { self.root_directory.as_mut() };

        let entry = &mut root.entries[pt_index as usize];
        if entry.is_present() {
            return Err("Page already mapped!");
        }

        entry.set_addr(physical_addr, flags);
        Ok(())
    }

    /// Unmaps a virtual page
    pub fn unmap_page(&mut self, virtual_addr: u64) -> Result<(), &'static str> {
        let pt_index = (virtual_addr >> 12) & 0x1FF;
        let root = unsafe { self.root_directory.as_mut() };

        let entry = &mut root.entries[pt_index as usize];
        if !entry.is_present() {
            return Err("Page is not mapped!");
        }

        entry.clear();
        Ok(())
    }
}

// =========================================================================
// MEMORY DESCRIPTOR LIST (MDL) & ANCIENT ISA DMA BUFFER ABSTRACTIONS
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryProtection {
    ReadOnly,
    ReadWrite,
    ExecuteRead,
    ExecuteReadWrite,
}

#[derive(Debug, Clone)]
pub struct MemoryDescriptorList {
    pub start_virtual_addr: u64,
    pub byte_length: usize,
    pub is_locked_pinned: bool,
    pub protection_flags: MemoryProtection,
    pub physical_page_offsets: Vec<u64>,
}

impl MemoryDescriptorList {
    pub fn new(virtual_addr: u64, length: usize, protection: MemoryProtection) -> Self {
        Self {
            start_virtual_addr: virtual_addr,
            byte_length: length,
            is_locked_pinned: false,
            protection_flags: protection,
            physical_page_offsets: Vec::new(),
        }
    }

    pub fn lock_pages_and_pin(&mut self, physical_pages: &[u64]) -> Result<(), &'static str> {
        if self.is_locked_pinned {
            return Err("MDL pages are already pinned in physical memory");
        }
        self.physical_page_offsets = physical_pages.to_vec();
        self.is_locked_pinned = true;
        Ok(())
    }

    pub fn unlock_pages(&mut self) {
        self.is_locked_pinned = false;
        self.physical_page_offsets.clear();
    }
}

pub const ISA_DMA_MAX_PHYSICAL_ADDR: u64 = 16 * 1024 * 1024; // Strict 16MB physical boundary for ancient ISA DMA

pub struct FloppyDiskDmaBuffer {
    pub physical_addr: u64,
    pub channel: u8, // ISA DMA Channel 2
    pub buffer_length: usize, // Max 64KB
}

impl FloppyDiskDmaBuffer {
    pub fn allocate_below_16mb(phys_addr: u64, length: usize) -> Result<Self, &'static str> {
        if phys_addr >= ISA_DMA_MAX_PHYSICAL_ADDR {
            return Err("Floppy Disk ISA DMA allocation exceeds 16MB physical RAM boundary");
        }
        if length > 64 * 1024 {
            return Err("Floppy Disk DMA buffer exceeds 64KB transfer limit");
        }
        Ok(Self {
            physical_addr: phys_addr,
            channel: 2,
            buffer_length: length,
        })
    }
}

pub struct SoundBlaster16DmaBuffer {
    pub physical_addr: u64,
    pub channel: u8, // ISA DMA Channel 5 (16-bit audio)
    pub is_double_buffered: bool,
}

impl SoundBlaster16DmaBuffer {
    pub fn allocate_ping_pong_buffer(phys_addr: u64) -> Result<Self, &'static str> {
        if phys_addr >= ISA_DMA_MAX_PHYSICAL_ADDR {
            return Err("Sound Blaster 16 ISA DMA allocation exceeds 16MB physical RAM boundary");
        }
        Ok(Self {
            physical_addr: phys_addr,
            channel: 5,
            is_double_buffered: true,
        })
    }
}

pub struct Ne2000DmaBuffer {
    pub shared_ram_base: u16,
    pub ring_buffer_size: usize,
}

impl Ne2000DmaBuffer {
    pub fn new(ram_base: u16, size: usize) -> Self {
        Self {
            shared_ram_base: ram_base,
            ring_buffer_size: size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_creation() {
        let allocator = BuddyAllocator::new();
        assert!(allocator.free_lists.iter().all(|list| list.is_empty()));
    }

    #[test]
    fn test_order_calculation() {
        let allocator = BuddyAllocator::new();
        assert_eq!(allocator.calculate_order(1), 0);
        assert_eq!(allocator.calculate_order(2), 1);
        assert_eq!(allocator.calculate_order(4), 2);
        assert_eq!(allocator.calculate_order(5), 3);
        assert_eq!(allocator.calculate_order(8), 3);
        assert_eq!(allocator.calculate_order(9), 4);
    }

    #[test]
    fn test_allocate_deallocate() {
        let mut allocator = BuddyAllocator::new();
        // This would need actual memory to work properly
        // For now, just test the interface
        let _result = allocator.allocate(4096);
        // Will fail without actual memory, but tests the flow
    }

    #[test]
    fn test_checkpoint_and_state_recovery() {
        let mut allocator = BuddyAllocator::new();
        allocator.initialize_memory(0x1000, 4096); // 1 page (order 0)
        allocator.initialize_memory(0x3000, 8192); // 2 pages (order 1)
        assert_eq!(allocator.get_free_memory(), 12288);

        // Checkpoint original state
        let checkpoint = allocator.create_checkpoint();

        // Perform mock allocations which modify state
        let _block1 = allocator.allocate(4096).unwrap();
        let _block2 = allocator.allocate(8192).unwrap();
        assert_eq!(allocator.get_free_memory(), 0);

        // Simulated crash/unwinding: Restore from checkpoint to recover state
        allocator.restore_checkpoint(checkpoint);

        // State is perfectly restored
        assert_eq!(allocator.get_free_memory(), 12288);

        // Verify we can allocate the same blocks again successfully
        let block_retry = allocator.allocate(4096).unwrap();
        assert_eq!(block_retry.size, 4096);
    }

    #[test]
    fn test_windows_nt_pool_allocator() {
        let mut pool_manager = KernelPoolManager::new();

        // Allocate Paged Pool Block with Tag 'File'
        let paged_block = pool_manager.allocate_pool(PoolType::Paged, 1024, b"File").unwrap();
        assert_eq!(paged_block.size, 1024);
        assert_eq!(paged_block.pool_type, PoolType::Paged);
        assert_eq!(&paged_block.tag, b"File");
        assert_eq!(pool_manager.total_paged_bytes, 1024);

        // Allocate NonPaged Pool Block with Tag 'Net '
        let non_paged_block = pool_manager.allocate_pool(PoolType::NonPaged, 2048, b"Net ").unwrap();
        assert_eq!(non_paged_block.size, 2048);
        assert_eq!(non_paged_block.pool_type, PoolType::NonPaged);
        assert_eq!(&non_paged_block.tag, b"Net ");
        assert_eq!(pool_manager.total_non_paged_bytes, 2048);

        // Verify Address Separation
        assert!(paged_block.addr != non_paged_block.addr);

        // Free Paged Pool Block
        assert!(pool_manager.free_pool(paged_block.addr).is_ok());
        assert_eq!(pool_manager.total_paged_bytes, 0);

        // Free NonPaged Pool Block
        assert!(pool_manager.free_pool(non_paged_block.addr).is_ok());
        assert_eq!(pool_manager.total_non_paged_bytes, 0);

        // Double Free (Should Fail)
        assert!(pool_manager.free_pool(paged_block.addr).is_err());
    }

    #[test]
    fn test_memory_descriptor_list_mdl_pinning() {
        let mut mdl = MemoryDescriptorList::new(0x7FFF_0000, 8192, MemoryProtection::ReadWrite);
        assert!(!mdl.is_locked_pinned);

        let phys_pages = [0x1000, 0x2000];
        assert!(mdl.lock_pages_and_pin(&phys_pages).is_ok());
        assert!(mdl.is_locked_pinned);
        assert_eq!(mdl.physical_page_offsets, vec![0x1000, 0x2000]);

        // Attempting to lock twice fails
        assert!(mdl.lock_pages_and_pin(&phys_pages).is_err());

        mdl.unlock_pages();
        assert!(!mdl.is_locked_pinned);
        assert!(mdl.physical_page_offsets.is_empty());
    }

    #[test]
    fn test_ancient_isa_dma_buffer_boundaries() {
        // Floppy Disk ISA DMA test (<16MB and <=64KB)
        let floppy = FloppyDiskDmaBuffer::allocate_below_16mb(0x00A0_0000, 32 * 1024).unwrap();
        assert_eq!(floppy.channel, 2);

        assert!(FloppyDiskDmaBuffer::allocate_below_16mb(17 * 1024 * 1024, 1024).is_err()); // > 16MB
        assert!(FloppyDiskDmaBuffer::allocate_below_16mb(0x00A0_0000, 128 * 1024).is_err()); // > 64KB

        // Sound Blaster 16 ISA DMA test (<16MB)
        let sb16 = SoundBlaster16DmaBuffer::allocate_ping_pong_buffer(0x00B0_0000).unwrap();
        assert_eq!(sb16.channel, 5);
        assert!(sb16.is_double_buffered);

        assert!(SoundBlaster16DmaBuffer::allocate_ping_pong_buffer(18 * 1024 * 1024).is_err()); // > 16MB

        // NE2000 Shared RAM Ring Buffer test
        let ne2000 = Ne2000DmaBuffer::new(0x300, 16384);
        assert_eq!(ne2000.shared_ram_base, 0x300);
        assert_eq!(ne2000.ring_buffer_size, 16384);
    }
}
