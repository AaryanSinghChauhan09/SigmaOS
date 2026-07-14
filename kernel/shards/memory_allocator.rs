/// SigmaOS: ============================================================================
/// Memory Allocator - OOP-Based Virtual Memory Management
/// ============================================================================
/// Implements virtual memory paging with trait-based allocators.
/// Supports buddy allocator, slab allocator, and custom allocators via traits.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

mod shard_traits;
use shard_traits::{MemoryManager, Shard, MemoryError, MemoryStats};

// ============================================================================
// ALLOCATOR TRAITS
// ============================================================================

/// Base trait for memory allocators
pub trait Allocator {
    /// Allocate a block of memory
    fn allocate(&mut self, size: usize, align: usize) -> Result<u64, MemoryError>;
    
    /// Free a previously allocated block
    fn free(&mut self, addr: u64) -> Result<(), MemoryError>;
    
    /// Get allocator statistics
    fn stats(&self) -> AllocatorStats;
}

/// Trait for buddy allocator
pub trait BuddyAllocator: Allocator {
    /// Get order of allocation
    fn get_order(&self, size: usize) -> usize;
    
    /// Split block into smaller blocks
    fn split_block(&mut self, addr: u64, order: usize) -> Result<(), MemoryError>;
    
    /// Merge adjacent blocks
    fn merge_blocks(&mut self, addr: u64, order: usize) -> Result<(), MemoryError>;
}

/// Trait for slab allocator
pub trait SlabAllocator: Allocator {
    /// Create a new slab for objects of given size
    fn create_slab(&mut self, size: usize) -> Result<u64, MemoryError>;
    
    /// Allocate from slab
    fn slab_alloc(&mut self, slab_id: u64) -> Result<u64, MemoryError>;
    
    /// Free to slab
    fn slab_free(&mut self, slab_id: u64, addr: u64) -> Result<(), MemoryError>;
}

// ============================================================================
// STATISTICS STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct AllocatorStats {
    pub total_allocated: u64,
    pub total_freed: u64,
    pub current_usage: u64,
    pub fragmentation_ratio: f64,
    pub alloc_count: u64,
    pub free_count: u64,
}

// ============================================================================
// BUDDY ALLOCATOR IMPLEMENTATION
// ============================================================================

pub struct SigmaBuddyAllocator {
    base_addr: u64,
    total_size: u64,
    min_order: usize,
    max_order: usize,
    free_lists: [AtomicU64; 64], // One list per order
    stats: AllocatorStats,
}

impl SigmaBuddyAllocator {
    pub const fn new(base_addr: u64, total_size: u64) -> Self {
        Self {
            base_addr,
            total_size,
            min_order: 4,  // 16 bytes minimum
            max_order: 20, // 1MB maximum
            free_lists: [AtomicU64::new(0); 64],
            stats: AllocatorStats {
                total_allocated: 0,
                total_freed: 0,
                current_usage: 0,
                fragmentation_ratio: 0.0,
                alloc_count: 0,
                free_count: 0,
            },
        }
    }

    fn calculate_order(&self, size: usize) -> usize {
        let mut order = self.min_order;
        while (1usize << order) < size && order < self.max_order {
            order += 1;
        }
        order
    }
}

impl Allocator for SigmaBuddyAllocator {
    fn allocate(&mut self, size: usize, _align: usize) -> Result<u64, MemoryError> {
        let order = self.calculate_order(size);
        
        // Find free block of appropriate order
        for current_order in order..=self.max_order {
            let block = self.free_lists[current_order].load(Ordering::Relaxed);
            if block != 0 {
                // Remove from free list
                self.free_lists[current_order].store(0, Ordering::Relaxed);
                
                // Split if necessary
                if current_order > order {
                    for split_order in (order + 1)..=current_order {
                        let buddy = block + (1usize << (split_order - 1)) as u64;
                        self.free_lists[split_order - 1].store(buddy, Ordering::Relaxed);
                    }
                }
                
                self.stats.total_allocated += (1usize << order) as u64;
                self.stats.current_usage += (1usize << order) as u64;
                self.stats.alloc_count += 1;
                
                return Ok(block);
            }
        }
        
        Err(MemoryError::OutOfMemory)
    }

    fn free(&mut self, addr: u64) -> Result<(), MemoryError> {
        // Determine block order (simplified - in real implementation would track this)
        let order = self.min_order;
        
        // Add to free list
        self.free_lists[order].store(addr, Ordering::Relaxed);
        
        // Try to merge with buddy
        let buddy = addr ^ (1usize << order) as u64;
        let buddy_in_list = self.free_lists[order].load(Ordering::Relaxed) == buddy;
        
        if buddy_in_list {
            // Merge blocks
            let merged_addr = addr.min(buddy);
            self.free_lists[order].store(0, Ordering::Relaxed);
            self.free_lists[order + 1].store(merged_addr, Ordering::Relaxed);
        }
        
        self.stats.total_freed += (1usize << order) as u64;
        self.stats.current_usage -= (1usize << order) as u64;
        self.stats.free_count += 1;
        
        Ok(())
    }

    fn stats(&self) -> AllocatorStats {
        self.stats
    }
}

impl BuddyAllocator for SigmaBuddyAllocator {
    fn get_order(&self, size: usize) -> usize {
        self.calculate_order(size)
    }

    fn split_block(&mut self, addr: u64, order: usize) -> Result<(), MemoryError> {
        if order <= self.min_order {
            return Err(MemoryError::AlignmentError);
        }
        
        let buddy = addr + (1usize << (order - 1)) as u64;
        self.free_lists[order - 1].store(buddy, Ordering::Relaxed);
        Ok(())
    }

    fn merge_blocks(&mut self, addr: u64, order: usize) -> Result<(), MemoryError> {
        if order >= self.max_order {
            return Err(MemoryError::AlignmentError);
        }
        
        let buddy = addr ^ (1usize << order) as u64;
        let merged_addr = addr.min(buddy);
        self.free_lists[order + 1].store(merged_addr, Ordering::Relaxed);
        Ok(())
    }
}

// ============================================================================
// SLAB ALLOCATOR IMPLEMENTATION
// ============================================================================

pub struct SigmaSlabAllocator {
    slabs: [Slab; 32], // Up to 32 different slab sizes
    slab_count: AtomicUsize,
    stats: AllocatorStats,
}

#[derive(Clone, Copy)]
struct Slab {
    object_size: usize,
    free_objects: AtomicU64,
    base_addr: u64,
    total_objects: u64,
}

impl Slab {
    pub const fn empty() -> Self {
        Self {
            object_size: 0,
            free_objects: AtomicU64::new(0),
            base_addr: 0,
            total_objects: 0,
        }
    }
}

impl SigmaSlabAllocator {
    pub const fn new() -> Self {
        Self {
            slabs: [Slab::empty(); 32],
            slab_count: AtomicUsize::new(0),
            stats: AllocatorStats {
                total_allocated: 0,
                total_freed: 0,
                current_usage: 0,
                fragmentation_ratio: 0.0,
                alloc_count: 0,
                free_count: 0,
            },
        }
    }
}

impl Allocator for SigmaSlabAllocator {
    fn allocate(&mut self, size: usize, _align: usize) -> Result<u64, MemoryError> {
        // Find appropriate slab
        for i in 0..self.slabs.len() {
            if self.slabs[i].object_size == size && self.slabs[i].free_objects.load(Ordering::Relaxed) > 0 {
                let obj_idx = self.slabs[i].free_objects.fetch_sub(1, Ordering::Relaxed) - 1;
                let addr = self.slabs[i].base_addr + obj_idx * size as u64;
                
                self.stats.total_allocated += size as u64;
                self.stats.current_usage += size as u64;
                self.stats.alloc_count += 1;
                
                return Ok(addr);
            }
        }
        
        // Create new slab if needed
        self.create_slab(size)?;
        self.slab_alloc(self.slab_count.load(Ordering::Relaxed) as u64 - 1)
    }

    fn free(&mut self, addr: u64) -> Result<(), MemoryError> {
        // Find which slab this address belongs to (simplified)
        for i in 0..self.slabs.len() {
            let slab = &self.slabs[i];
            if slab.base_addr <= addr && addr < slab.base_addr + slab.total_objects * slab.object_size as u64 {
                let obj_idx = (addr - slab.base_addr) / slab.object_size as u64;
                slab.free_objects.fetch_add(1, Ordering::Relaxed);
                
                self.stats.total_freed += slab.object_size as u64;
                self.stats.current_usage -= slab.object_size as u64;
                self.stats.free_count += 1;
                
                return Ok(());
            }
        }
        
        Err(MemoryError::InvalidAddress)
    }

    fn stats(&self) -> AllocatorStats {
        self.stats
    }
}

impl SlabAllocator for SigmaSlabAllocator {
    fn create_slab(&mut self, size: usize) -> Result<u64, MemoryError> {
        let idx = self.slab_count.load(Ordering::Relaxed);
        if idx >= 32 {
            return Err(MemoryError::ResourceExhausted);
        }
        
        let objects_per_slab = 1024;
        let slab = Slab {
            object_size: size,
            free_objects: AtomicU64::new(objects_per_slab),
            base_addr: 0x10000000 + idx as u64 * 0x100000, // Placeholder addresses
            total_objects: objects_per_slab,
        };
        
        self.slabs[idx] = slab;
        self.slab_count.fetch_add(1, Ordering::Relaxed);
        
        Ok(idx as u64)
    }

    fn slab_alloc(&mut self, slab_id: u64) -> Result<u64, MemoryError> {
        let idx = slab_id as usize;
        if idx >= self.slab_count.load(Ordering::Relaxed) {
            return Err(MemoryError::InvalidAddress);
        }
        
        let slab = &mut self.slabs[idx];
        if slab.free_objects.load(Ordering::Relaxed) == 0 {
            return Err(MemoryError::OutOfMemory);
        }
        
        let obj_idx = slab.free_objects.fetch_sub(1, Ordering::Relaxed) - 1;
        let addr = slab.base_addr + obj_idx * slab.object_size as u64;
        
        self.stats.total_allocated += slab.object_size as u64;
        self.stats.current_usage += slab.object_size as u64;
        self.stats.alloc_count += 1;
        
        Ok(addr)
    }

    fn slab_free(&mut self, slab_id: u64, addr: u64) -> Result<(), MemoryError> {
        let idx = slab_id as usize;
        if idx >= self.slab_count.load(Ordering::Relaxed) {
            return Err(MemoryError::InvalidAddress);
        }
        
        let slab = &mut self.slabs[idx];
        slab.free_objects.fetch_add(1, Ordering::Relaxed);
        
        self.stats.total_freed += slab.object_size as u64;
        self.stats.current_usage -= slab.object_size as u64;
        self.stats.free_count += 1;
        
        Ok(())
    }
}

// ============================================================================
// UNIFIED MEMORY MANAGER
// ============================================================================

pub struct SigmaMemoryManager {
    buddy: SigmaBuddyAllocator,
    slab: SigmaSlabAllocator,
    total_memory: AtomicU64,
    used_memory: AtomicU64,
}

impl SigmaMemoryManager {
    pub const fn new(base_addr: u64, total_size: u64) -> Self {
        Self {
            buddy: SigmaBuddyAllocator::new(base_addr, total_size),
            slab: SigmaSlabAllocator::new(),
            total_memory: AtomicU64::new(total_size),
            used_memory: AtomicU64::new(0),
        }
    }

    /// Allocate using appropriate allocator based on size
    pub fn allocate_smart(&mut self, size: usize, align: usize) -> Result<u64, MemoryError> {
        // Use slab allocator for small objects (< 4KB)
        if size < 4096 {
            self.slab.allocate(size, align)
        } else {
            // Use buddy allocator for large allocations
            self.buddy.allocate(size, align)
        }
    }

    /// Free using appropriate allocator
    pub fn free_smart(&mut self, addr: u64) -> Result<(), MemoryError> {
        // Try slab first, then buddy
        if let Ok(()) = self.slab.free(addr) {
            return Ok(());
        }
        self.buddy.free(addr)
    }
}

impl Shard for SigmaMemoryManager {
    fn init(&self) -> Result<(), shard_traits::ShardError> {
        // Initialize allocators
        Ok(())
    }

    fn is_operational(&self) -> bool {
        self.total_memory.load(Ordering::Relaxed) > 0
    }

    fn shard_id(&self) -> u64 {
        0xDEAD_BEEF // Memory manager shard ID
    }

    fn shutdown(&self) -> Result<(), shard_traits::ShardError> {
        // Cleanup allocators
        Ok(())
    }
}

impl MemoryManager for SigmaMemoryManager {
    fn allocate(&self, size: usize) -> Result<u64, MemoryError> {
        // This would need interior mutability in real implementation
        Err(MemoryError::OutOfMemory)
    }

    fn free(&self, _addr: u64) -> Result<(), MemoryError> {
        // This would need interior mutability in real implementation
        Ok(())
    }

    fn memory_stats(&self) -> MemoryStats {
        MemoryStats {
            total: self.total_memory.load(Ordering::Relaxed),
            used: self.used_memory.load(Ordering::Relaxed),
            free: self.total_memory.load(Ordering::Relaxed) - self.used_memory.load(Ordering::Relaxed),
            cached: 0,
        }
    }
}
