#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(feature = "standalone_test"))]
use crate::klib::buddy_allocator::{BuddyAllocator, SimpleBuddyAllocator};
#[cfg(not(feature = "standalone_test"))]
use crate::kernel::memory::{BuddyAllocator as KernelBuddyAllocator, MemoryBlock, PAGE_SIZE};

#[cfg(feature = "standalone_test")]
pub mod dummy {
    use alloc::vec::Vec;
    use core::ptr::NonNull;
    use core::sync::atomic::{AtomicUsize, Ordering};

    pub const PAGE_SIZE: usize = 4096;

    #[derive(Debug, Clone, Copy)]
    pub struct MemoryBlock {
        pub addr: NonNull<u8>,
        pub size: usize,
    }

    pub type BlockID = usize;
    #[derive(Debug, Clone, Copy)]
    pub enum AllocError { Success, OutOfMemory, InvalidBlock, Fragmentation }

    pub trait BuddyAllocator {
        fn allocate(&mut self, order: usize) -> Result<BlockID, AllocError>;
        fn free(&mut self, block_id: BlockID, order: usize) -> Result<(), AllocError>;
        fn get_free_count(&self, order: usize) -> usize;
    }

    pub struct SimpleBuddyAllocator {
        pub max_order: usize,
    }
    impl SimpleBuddyAllocator {
        pub fn new(max_order: usize, _frames: usize) -> Self { Self { max_order } }
    }
    impl BuddyAllocator for SimpleBuddyAllocator {
        fn allocate(&mut self, _order: usize) -> Result<BlockID, AllocError> { Ok(1) }
        fn free(&mut self, _id: BlockID, _order: usize) -> Result<(), AllocError> { Ok(()) }
        fn get_free_count(&self, _order: usize) -> usize { 1 }
    }
}
#[cfg(feature = "standalone_test")]
use dummy::*;

/// SigmaOS Buddy Allocator Wrapper
///
/// Wraps the klib buddy allocator and exposes a kernel-friendly interface.
/// Integrates with the existing memory subsystem.
pub struct SigmaBuddyAllocator {
    pub inner: SimpleBuddyAllocator,
    pub base_addr: usize,
    pub total_size: usize,
    pub allocated: AtomicUsize,
}

impl SigmaBuddyAllocator {
    pub fn new(base_addr: usize, total_size: usize, max_order: usize) -> Self {
        SigmaBuddyAllocator {
            inner: SimpleBuddyAllocator::new(max_order, total_size / PAGE_SIZE),
            base_addr,
            total_size,
            allocated: AtomicUsize::new(0),
        }
    }

    pub fn init(&mut self) {
        self.inner = SimpleBuddyAllocator::new(10, self.total_size / PAGE_SIZE);
    }

    pub fn allocate(&mut self, size: usize) -> Option<MemoryBlock> {
        if size == 0 || size > self.total_size {
            return None;
        }
        let pages = size.div_ceil(PAGE_SIZE);
        let order = Self::calculate_order(pages);
        match self.inner.allocate(order) {
            Ok(block_id) => {
                let addr = self.base_addr + (block_id * PAGE_SIZE);
                let actual_size = (1 << order) * PAGE_SIZE;
                self.allocated.fetch_add(actual_size, Ordering::SeqCst);
                use core::ptr::NonNull;
                NonNull::new(addr as *mut u8).map(|addr| MemoryBlock {
                    addr,
                    size: actual_size,
                })
            }
            Err(_) => None,
        }
    }

    pub fn free(&mut self, block: &MemoryBlock) {
        let addr = block.addr.as_ptr() as usize;
        let block_id = (addr - self.base_addr) / PAGE_SIZE;
        let pages = block.size / PAGE_SIZE;
        let order = Self::calculate_order(pages);
        let _ = self.inner.free(block_id, order);
        self.allocated.fetch_sub(block.size, Ordering::SeqCst);
    }

    pub fn get_free_memory(&self) -> usize {
        self.total_size - self.allocated.load(Ordering::SeqCst)
    }

    pub fn get_total_memory(&self) -> usize {
        self.total_size
    }

    pub fn get_used_memory(&self) -> usize {
        self.allocated.load(Ordering::SeqCst)
    }

    fn calculate_order(pages: usize) -> usize {
        if pages <= 1 {
            0
        } else {
            pages.next_power_of_two().trailing_zeros() as usize
        }
    }
}

// =========================================================================
// Linux & BSD Inspired Buddy Memory Allocator Glue Subsystem
// =========================================================================

/// Anti-fragmentation page frame migration type (inspired by Linux pageblock migration types)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MigrateType {
    Unmovable,   // Kernel core structures, page tables, ISR stacks
    Reclaimable, // Page cache, dentry cache, slab caches
    Movable,     // User process pages, anonymous memory
    HighAtomic,  // High-priority emergency interrupt allocations
}

/// Linux-style memory zone watermarks for page frame pressure management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneWatermark {
    Min,  // Emergency kswapd/direct reclaim threshold
    Low,  // Background reclaim activation threshold
    High, // Normal memory state threshold
}

/// FreeBSD-style physical memory domain tracking for NUMA memory policy enforcement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmPhysDomain {
    pub domain_id: usize,
    pub total_pages: usize,
    pub free_pages: usize,
}

/// OpenBSD-style security guard for zeroing pages prior to process exposure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LazyPageZeroer {
    pub enforce_security_zeroing: bool,
}

impl LazyPageZeroer {
    pub fn new(enforce: bool) -> Self {
        Self {
            enforce_security_zeroing: enforce,
        }
    }

    pub fn prepare_page_for_user(&self, page_addr: usize, page_size: usize) {
        if self.enforce_security_zeroing && page_addr != 0 {
            #[cfg(not(test))]
            {
                let ptr = page_addr as *mut u8;
                unsafe {
                    core::ptr::write_bytes(ptr, 0, page_size);
                }
            }
        }
    }
}

/// Linux & BSD Buddy Memory Allocator Glue Engine
/// Glues Linux anti-fragmentation migration types, Linux zone watermarks,
/// FreeBSD NUMA physical domains, and OpenBSD security page zeroing into SigmaOS BuddyAllocator.
pub struct LinuxBsdBuddyGlueEngine {
    pub buddy: SigmaBuddyAllocator,
    pub migration_counters: [usize; 4], // Count per MigrateType
    pub min_watermark_pages: usize,
    pub low_watermark_pages: usize,
    pub high_watermark_pages: usize,
    pub numa_domains: Vec<VmPhysDomain>,
    pub zeroer: LazyPageZeroer,
}

impl LinuxBsdBuddyGlueEngine {
    pub fn new(base_addr: usize, total_size: usize) -> Self {
        let total_pages = total_size / PAGE_SIZE;
        let min_wm = (total_pages * 5) / 100;   // 5% min watermark
        let low_wm = (total_pages * 10) / 100;  // 10% low watermark
        let high_wm = (total_pages * 15) / 100; // 15% high watermark

        let mut numa_domains = Vec::new();
        numa_domains.push(VmPhysDomain {
            domain_id: 0,
            total_pages,
            free_pages: total_pages,
        });

        Self {
            buddy: SigmaBuddyAllocator::new(base_addr, total_size, 12),
            migration_counters: [0; 4],
            min_watermark_pages: min_wm,
            low_watermark_pages: low_wm,
            high_watermark_pages: high_wm,
            numa_domains,
            zeroer: LazyPageZeroer::new(true),
        }
    }

    /// Check if memory zone meets specified watermark constraint
    pub fn zone_watermark_ok(&self, watermark: ZoneWatermark) -> bool {
        let free_pages = self.buddy.get_free_memory() / PAGE_SIZE;
        let required = match watermark {
            ZoneWatermark::Min => self.min_watermark_pages,
            ZoneWatermark::Low => self.low_watermark_pages,
            ZoneWatermark::High => self.high_watermark_pages,
        };
        free_pages >= required
    }

    /// Allocate a page block with Linux-style migration type & OpenBSD security zeroing
    pub fn allocate_tagged(
        &mut self,
        size: usize,
        migrate_type: MigrateType,
    ) -> Option<MemoryBlock> {
        // Enforce min watermark for non-atomic allocations
        if migrate_type != MigrateType::HighAtomic && !self.zone_watermark_ok(ZoneWatermark::Min) {
            return None;
        }

        if let Some(block) = self.buddy.allocate(size) {
            let idx = match migrate_type {
                MigrateType::Unmovable => 0,
                MigrateType::Reclaimable => 1,
                MigrateType::Movable => 2,
                MigrateType::HighAtomic => 3,
            };
            self.migration_counters[idx] += block.size / PAGE_SIZE;

            // Update NUMA domain 0 free pages
            if !self.numa_domains.is_empty() {
                let pages = block.size / PAGE_SIZE;
                if self.numa_domains[0].free_pages >= pages {
                    self.numa_domains[0].free_pages -= pages;
                }
            }

            // OpenBSD lazy zeroing for user/movable pages
            if migrate_type == MigrateType::Movable {
                self.zeroer
                    .prepare_page_for_user(block.addr.as_ptr() as usize, block.size);
            }

            Some(block)
        } else {
            None
        }
    }

    /// Free a page block and restore domain page tracking
    pub fn free_tagged(&mut self, block: &MemoryBlock, migrate_type: MigrateType) {
        let pages = block.size / PAGE_SIZE;
        let idx = match migrate_type {
            MigrateType::Unmovable => 0,
            MigrateType::Reclaimable => 1,
            MigrateType::Movable => 2,
            MigrateType::HighAtomic => 3,
        };
        if self.migration_counters[idx] >= pages {
            self.migration_counters[idx] -= pages;
        }
        if !self.numa_domains.is_empty() {
            self.numa_domains[0].free_pages += pages;
        }
        self.buddy.free(block);
    }
}

impl Default for SigmaBuddyAllocator {
    fn default() -> Self {
        Self {
            inner: SimpleBuddyAllocator::new(10, 1024),
            base_addr: 0,
            total_size: 0,
            allocated: AtomicUsize::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buddy_wrapper_allocation() {
        let mut allocator = SigmaBuddyAllocator::new(0x1000_0000, 4 * 1024 * 1024, 12);
        allocator.init();
        let block = allocator.allocate(4096);
        assert!(block.is_some());
    }

    #[test]
    fn test_buddy_wrapper_stats() {
        let mut allocator = SigmaBuddyAllocator::new(0x1000_0000, 4 * 1024 * 1024, 12);
        allocator.init();
        let total = allocator.get_total_memory();
        assert_eq!(total, 4 * 1024 * 1024);
    }

    #[test]
    fn test_linux_bsd_buddy_glue_engine() {
        let mut glue = LinuxBsdBuddyGlueEngine::new(0x2000_0000, 4 * 1024 * 1024);
        glue.buddy.init();

        // Check watermarks
        assert!(glue.zone_watermark_ok(ZoneWatermark::Min));
        assert!(glue.zone_watermark_ok(ZoneWatermark::High));

        // Test tagged allocation with Movable migration type & OpenBSD lazy zeroing
        let block = glue.allocate_tagged(4096, MigrateType::Movable);
        assert!(block.is_some());
        let allocated_block = block.unwrap();
        assert_eq!(glue.migration_counters[2], 1); // 1 page movable

        // Free tagged allocation
        glue.free_tagged(&allocated_block, MigrateType::Movable);
        assert_eq!(glue.migration_counters[2], 0);
    }
}
