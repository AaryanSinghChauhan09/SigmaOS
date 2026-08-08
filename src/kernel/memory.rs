// SigmaOS Kernel Memory Management
// Implements buddy allocator and paging

use core::ptr::NonNull;

/// Memory page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Memory block
#[derive(Debug, Clone, Copy)]
pub struct MemoryBlock {
    pub addr: NonNull<u8>,
    pub size: usize,
}

/// Buddy allocator for memory management
pub struct BuddyAllocator {
    free_lists: [Vec<MemoryBlock>; 12], // 2^0 to 2^11 pages (4KB to 8MB)
}

impl BuddyAllocator {
    pub fn new() -> Self {
        Self {
            free_lists: Default::default(),
        }
    }

    pub fn with_memory(base_addr: usize, size: usize) -> Self {
        let mut allocator = Self::new();
        allocator.initialize_memory(base_addr, size);
        allocator
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
        let mut order = 0;
        let mut size = 1;
        while size < pages {
            size *= 2;
            order += 1;
        }
        order
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
        let buddy_addr = block_addr ^ (1 << (order + 12)); // Calculate buddy address
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

            if let Some(addr) = NonNull::new(merged_addr as *mut u8) {
                Ok(MemoryBlock {
                    addr,
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
}

impl VirtualMemoryManager {
    pub fn new(root_directory: NonNull<PageTable>) -> Self {
        Self { root_directory }
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
}
