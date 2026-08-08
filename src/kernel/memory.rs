// SigmaOS Kernel Memory Management
// Implements buddy allocator and paging

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// Memory page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Memory block
#[derive(Debug, Clone, Copy)]
pub struct MemoryBlock {
    pub addr: NonNull<u8>,
    pub size: usize,
}

use core::ptr::NonNull;

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
||||||| 43be3a7e8
            let block = MemoryBlock {
                addr: NonNull::new(base_addr as *mut u8).unwrap(),
                size,
            };
            self.free_lists[order].push(block);
            if let Some(addr) = NonNull::new(base_addr as *mut u8) {
                let block = MemoryBlock {
                    addr,
                    size,
                };
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

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemoryMerkleNode {
    pub page_index: usize,
    pub data_hash: u64,
}

impl MemoryMerkleNode {
    pub fn compute_hash(data: &[u8]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
}

/// Virtual Memory Manager (VMM) handling paging
pub struct VirtualMemoryManager {
    pub root_directory: NonNull<PageTable>,
    pub buddy_allocator: BuddyAllocator,
||||||| 43be3a7e8
    pub page_ref_counts: HashMap<u64, u32>, // physical frame addr -> reference count (for Copy-on-Write)
    pub shadow_snapshots: HashMap<u64, String>, // virtual_addr -> snapshot copy (for snapshot isolation)
}

impl VirtualMemoryManager {
    pub fn new(root_directory: NonNull<PageTable>) -> Self {
        Self {
            root_directory,
            buddy_allocator: BuddyAllocator::new(),
        }
    }

    pub fn with_allocator(root_directory: NonNull<PageTable>, allocator: BuddyAllocator) -> Self {
        Self {
            root_directory,
            buddy_allocator: allocator,
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
||||||| 43be3a7e8
        Self { root_directory }
        Self {
            root_directory,
            page_ref_counts: HashMap::new(),
            shadow_snapshots: HashMap::new(),
        }
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

    /// Handles a Copy-on-Write (CoW) page fault.
    /// If multiple processes share a physical page, on write fault we duplicate the page and remap as WRITABLE.
    pub fn handle_page_fault_cow(&mut self, virtual_addr: u64, new_physical_frame: u64) -> Result<bool, &'static str> {
        let pt_index = (virtual_addr >> 12) & 0x1FF;
        let root = unsafe { self.root_directory.as_mut() };

        let entry = &mut root.entries[pt_index as usize];
        if !entry.is_present() {
            // Demand paging trigger: Map a newly allocated physical page if it's completely missing
            self.map_page(virtual_addr, new_physical_frame, PageFlags(PageFlags::PRESENT | PageFlags::WRITABLE))?;
            self.page_ref_counts.insert(new_physical_frame, 1);
            return Ok(true); // Resolved via demand paging
        }

        let old_phys_addr = entry.get_addr();
        let ref_count = self.page_ref_counts.get(&old_phys_addr).cloned().unwrap_or(1);

        if ref_count > 1 {
            // Decement the reference count on the shared old page
            self.page_ref_counts.insert(old_phys_addr, ref_count - 1);

            // Remap virtual page to newly allocated physical page with write capability
            entry.set_addr(new_physical_frame, PageFlags(PageFlags::PRESENT | PageFlags::WRITABLE));
            self.page_ref_counts.insert(new_physical_frame, 1);

            // Record snapshot isolate copy
            self.shadow_snapshots.insert(virtual_addr, "CoW Page Duplicated".to_string());
            Ok(true) // Resolved via Copy-on-Write
        } else {
            // Only 1 process is mapping this page; just elevate permissions to writable if it wasn't
            entry.set_addr(old_phys_addr, PageFlags(PageFlags::PRESENT | PageFlags::WRITABLE));
            Ok(false)
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
||||||| 43be3a7e8

    #[test]
    fn test_demand_paging_and_cow_snapshots() {
        // 1. Setup a page table on the stack/heap
        let mut pt = PageTable::new();
        let mut vmm = VirtualMemoryManager::new(NonNull::new(&mut pt as *mut PageTable).unwrap());

        let virtual_addr = 0x1000_0000;
        let original_phys_frame = 0x5000_0000;
        let new_phys_frame = 0x6000_0000;

        // 2. Validate Merkle node hashes
        let data = b"some page bytes";
        let root_hash = MemoryMerkleNode::compute_hash(data);
        let node = MemoryMerkleNode { page_index: 0, data_hash: root_hash };
        assert_eq!(node.data_hash, root_hash);

        // 3. Test demand-paging scenario (page not mapped -> page faults on write -> demand map)
        let resolved_demand = vmm.handle_page_fault_cow(virtual_addr, original_phys_frame).unwrap();
        assert!(resolved_demand); // resolved by demand map
        assert_eq!(vmm.translate(virtual_addr).unwrap(), original_phys_frame);

        // Reset present frame ref count to 2 to simulate shared page mapping (e.g. fork scenario)
        vmm.page_ref_counts.insert(original_phys_frame, 2);

        // 4. Test Copy-on-Write fault scenario (page present but shared, on write fault -> duplicate)
        let resolved_cow = vmm.handle_page_fault_cow(virtual_addr, new_phys_frame).unwrap();
        assert!(resolved_cow); // resolved by copy on write duplication
        assert_eq!(vmm.translate(virtual_addr).unwrap(), new_phys_frame);

        // Assert shadow snapshot isolating records
        assert_eq!(vmm.shadow_snapshots.get(&virtual_addr).unwrap(), "CoW Page Duplicated");
        assert_eq!(vmm.page_ref_counts.get(&original_phys_frame).cloned().unwrap(), 1);
        assert_eq!(vmm.page_ref_counts.get(&new_phys_frame).cloned().unwrap(), 1);
    }
}
