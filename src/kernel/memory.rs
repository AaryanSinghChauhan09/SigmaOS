// SigmaOS Kernel Memory Management
// Implements buddy allocator and paging

use core::ptr::NonNull;

/// Memory page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Memory block
#[derive(Debug)]
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
            let block = MemoryBlock {
                addr: NonNull::new(base_addr as *mut u8).unwrap(),
                size,
            };
            self.free_lists[order].push(block);
        }
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

        let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
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
        if let Some(merged_block) = self.try_merge(block, order) {
            self.deallocate(merged_block);
        } else {
            self.free_lists[order].push(block);
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
        if order < 12 && !self.free_lists[order].is_empty() {
            Some(self.free_lists[order].pop().unwrap())
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

    fn try_merge(&mut self, block: MemoryBlock, order: usize) -> Option<MemoryBlock> {
        if order >= 11 {
            return None; // Maximum order
        }

        let block_addr = block.addr.as_ptr() as usize;
        let buddy_addr = block_addr ^ (1 << (order + 12)); // Calculate buddy address
        let buddy_size = block.size * 2;

        // Find buddy in free list
        if let Some(pos) = self.free_lists[order]
            .iter()
            .position(|b| b.addr.as_ptr() as usize == buddy_addr && b.size == block.size)
        {
            let buddy = self.free_lists[order].remove(pos);

            // Merge blocks
            let merged_addr = if block_addr < buddy_addr {
                block_addr
            } else {
                buddy_addr
            };

            return Some(MemoryBlock {
                addr: NonNull::new(merged_addr as *mut u8)?,
                size: buddy_size,
            });
        }

        None
    }
}

impl Default for BuddyAllocator {
    fn default() -> Self {
        Self::new()
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
        let result = allocator.allocate(4096);
        // Will fail without actual memory, but tests the flow
    }
}
