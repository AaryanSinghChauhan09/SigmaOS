// Memory Allocator - Buddy and Slab Allocators
// Inspired by Linux buddy allocator and slab allocator

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Memory block
#[derive(Debug, Clone)]
pub struct MemoryBlock {
    pub start: u64,
    pub size: usize,
    pub allocated: bool,
    pub order: usize,
}

/// Buddy allocator
pub struct BuddyAllocator {
    total_memory: usize,
    min_block_size: usize,
    max_order: usize,
    free_lists: Vec<Vec<MemoryBlock>>,
    allocated_blocks: HashMap<u64, MemoryBlock>,
    next_block_id: AtomicU64,
}

impl BuddyAllocator {
    pub fn new(total_memory: usize, min_block_size: usize) -> Self {
        let max_order = (total_memory / min_block_size).next_power_of_two().trailing_zeros() as usize;

        let mut free_lists = vec![Vec::new(); max_order + 1];

        // Initialize with one large block
        let initial_block = MemoryBlock {
            start: 0,
            size: total_memory,
            allocated: false,
            order: max_order,
        };
        free_lists[max_order].push(initial_block);

        Self {
            total_memory,
            min_block_size,
            max_order,
            free_lists,
            allocated_blocks: HashMap::new(),
            next_block_id: AtomicU64::new(1),
        }
    }

    /// Allocate memory of given size
    pub fn allocate(&mut self, size: usize) -> Result<u64, &'static str> {
        let required_order = self.size_to_order(size);

        // Find a free block of at least required order
        let order = self.find_free_block(required_order)?;

        // Split blocks if necessary
        let block = self.split_block(order, required_order)?;

        // Mark as allocated
        let block_id = self.next_block_id.fetch_add(1, Ordering::SeqCst);
        let mut allocated_block = block.clone();
        allocated_block.allocated = true;

        self.allocated_blocks.insert(block_id, allocated_block);

        Ok(block.start)
    }

    /// Free memory
    pub fn free(&mut self, address: u64) -> Result<(), &'static str> {
        // Find allocated block
        let block_id = self.allocated_blocks
            .iter()
            .find(|(_, b)| b.start == address && b.allocated)
            .map(|(id, _)| *id)
            .ok_or("Block not found")?;

        let mut block = self.allocated_blocks.remove(&block_id).unwrap();
        block.allocated = false;

        // Merge with buddy if possible
        self.merge_buddy(block);

        Ok(())
    }

    /// Convert size to order
    fn size_to_order(&self, size: usize) -> usize {
        let block_size = size.next_power_of_two().max(self.min_block_size);
        (block_size / self.min_block_size).trailing_zeros() as usize
    }

    /// Find a free block of at least given order
    fn find_free_block(&mut self, required_order: usize) -> Result<usize, &'static str> {
        for order in required_order..=self.max_order {
            if !self.free_lists[order].is_empty() {
                return Ok(order);
            }
        }
        Err("No free block available")
    }

    /// Split block to required order
    fn split_block(&mut self, current_order: usize, required_order: usize) -> Result<MemoryBlock, &'static str> {
        let mut block = self.free_lists[current_order].pop().ok_or("Block not found")?;

        while block.order > required_order {
            let new_order = block.order - 1;
            let new_size = block.size / 2;

            // Split into two buddies
            let buddy1 = MemoryBlock {
                start: block.start,
                size: new_size,
                allocated: false,
                order: new_order,
            };

            let buddy2 = MemoryBlock {
                start: block.start + new_size as u64,
                size: new_size,
                allocated: false,
                order: new_order,
            };

            self.free_lists[new_order].push(buddy2);
            block = buddy1;
        }

        Ok(block)
    }

    /// Merge block with its buddy
    fn merge_buddy(&mut self, block: MemoryBlock) {
        let buddy_address = block.start ^ (1 << block.order);

        // Find buddy in free list
        for order in block.order..self.max_order {
            if let Some(pos) = self.free_lists[order].iter().position(|b| b.start == buddy_address) {
                // Found buddy - merge
                let buddy = self.free_lists[order].remove(pos);

                let merged_block = MemoryBlock {
                    start: block.start.min(buddy.start),
                    size: block.size * 2,
                    allocated: false,
                    order: order + 1,
                };

                // Try to merge further
                self.merge_buddy(merged_block);
                return;
            }
        }

        // No buddy found - add to free list
        self.free_lists[block.order].push(block);
    }

    /// Get total memory
    pub fn total_memory(&self) -> usize {
        self.total_memory
    }

    /// Get free memory
    pub fn free_memory(&self) -> usize {
        self.free_lists.iter()
            .flatten()
            .filter(|b| !b.allocated)
            .map(|b| b.size)
            .sum()
    }

    /// Get allocated memory
    pub fn allocated_memory(&self) -> usize {
        self.allocated_blocks.values()
            .filter(|b| b.allocated)
            .map(|b| b.size)
            .sum()
    }
}

/// Slab object
#[derive(Debug, Clone)]
pub struct SlabObject {
    pub address: u64,
    pub in_use: bool,
}

/// Slab cache
pub struct SlabCache {
    pub name: String,
    pub object_size: usize,
    pub objects_per_slab: usize,
    pub slabs: Vec<Vec<SlabObject>>,
    pub free_objects: AtomicUsize,
}

impl SlabCache {
    pub fn new(name: String, object_size: usize, objects_per_slab: usize) -> Self {
        Self {
            name,
            object_size,
            objects_per_slab,
            slabs: Vec::new(),
            free_objects: AtomicUsize::new(0),
        }
    }

    /// Allocate an object from the slab
    pub fn allocate(&mut self) -> Result<u64, &'static str> {
        // Find a free object
        for slab in &mut self.slabs {
            for obj in slab {
                if !obj.in_use {
                    obj.in_use = true;
                    self.free_objects.fetch_sub(1, Ordering::SeqCst);
                    return Ok(obj.address);
                }
            }
        }

        // No free object - create new slab
        self.grow_slab();

        // Try again
        self.allocate()
    }

    /// Free an object
    pub fn free(&mut self, address: u64) -> Result<(), &'static str> {
        for slab in &mut self.slabs {
            for obj in slab {
                if obj.address == address && obj.in_use {
                    obj.in_use = false;
                    self.free_objects.fetch_add(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err("Object not found")
    }

    /// Grow slab cache
    fn grow_slab(&mut self) {
        let base_address = self.slabs.len() as u64 * self.object_size as u64 * self.objects_per_slab as u64;

        let mut slab = Vec::new();
        for i in 0..self.objects_per_slab {
            slab.push(SlabObject {
                address: base_address + i as u64 * self.object_size as u64,
                in_use: false,
            });
        }

        self.free_objects.fetch_add(self.objects_per_slab, Ordering::SeqCst);
        self.slabs.push(slab);
    }

    /// Get number of free objects
    pub fn free_objects(&self) -> usize {
        self.free_objects.load(Ordering::SeqCst)
    }

    /// Get total objects
    pub fn total_objects(&self) -> usize {
        self.slabs.len() * self.objects_per_slab
    }
}

/// Slab allocator manager
pub struct SlabAllocator {
    caches: HashMap<String, SlabCache>,
}

impl SlabAllocator {
    pub fn new() -> Self {
        Self {
            caches: HashMap::new(),
        }
    }

    /// Create a slab cache
    pub fn create_cache(&mut self, name: String, object_size: usize, objects_per_slab: usize) -> Result<(), &'static str> {
        if self.caches.contains_key(&name) {
            return Err("Cache already exists");
        }

        let cache = SlabCache::new(name.clone(), object_size, objects_per_slab);
        self.caches.insert(name, cache);
        Ok(())
    }

    /// Allocate from a cache
    pub fn allocate(&mut self, cache_name: &str) -> Result<u64, &'static str> {
        let cache = self.caches.get_mut(cache_name).ok_or("Cache not found")?;
        cache.allocate()
    }

    /// Free to a cache
    pub fn free(&mut self, cache_name: &str, address: u64) -> Result<(), &'static str> {
        let cache = self.caches.get_mut(cache_name).ok_or("Cache not found")?;
        cache.free(address)
    }

    /// Get cache info
    pub fn get_cache_info(&self, cache_name: &str) -> Option<(usize, usize)> {
        let cache = self.caches.get(cache_name)?;
        Some((cache.free_objects(), cache.total_objects()))
    }

    /// Get cache count
    pub fn cache_count(&self) -> usize {
        self.caches.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buddy_allocate() {
        let mut allocator = BuddyAllocator::new(1024 * 1024, 4096);

        let address = allocator.allocate(4096).unwrap();
        assert!(address > 0);

        assert!(allocator.free(address).is_ok());
    }

    #[test]
    fn test_buddy_split() {
        let mut allocator = BuddyAllocator::new(1024 * 1024, 4096);

        allocator.allocate(4096).unwrap();
        allocator.allocate(4096).unwrap();

        assert!(allocator.allocated_memory() > 0);
    }

    #[test]
    fn test_slab_cache() {
        let mut cache = SlabCache::new("test".to_string(), 64, 10);

        let address = cache.allocate().unwrap();
        assert!(address > 0);

        assert!(cache.free(address).is_ok());
    }

    #[test]
    fn test_slab_allocator() {
        let mut allocator = SlabAllocator::new();

        allocator.create_cache("test".to_string(), 64, 10).unwrap();

        let address = allocator.allocate("test").unwrap();
        assert!(allocator.free("test", address).is_ok());
    }

    #[test]
    fn test_memory_stats() {
        let mut allocator = BuddyAllocator::new(1024 * 1024, 4096);

        assert_eq!(allocator.total_memory(), 1024 * 1024);
        assert_eq!(allocator.free_memory(), 1024 * 1024);

        allocator.allocate(4096).unwrap();
        assert!(allocator.allocated_memory() > 0);
    }
}
