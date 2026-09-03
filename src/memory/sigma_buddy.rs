#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS GlueBuddy Memory Subsystem
// Linux & BSD inspired Buddy Allocator Glue, Migration Types, CMA, Watermarks, and FreeBSD VM Page Queues

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

use super::{BuddyAllocator as KernelBuddyAllocator, MemoryBlock, PAGE_SIZE};
use crate::klib::buddy_allocator::{BuddyAllocator, SimpleBuddyAllocator};


/// SigmaOS Buddy Allocator Wrapper
///
/// Wraps the klib buddy allocator and exposes a kernel-friendly interface.
/// Integrates with the existing memory subsystem, migration types, CMA, and watermarks.
pub struct SigmaBuddyAllocator {
    pub inner: SimpleBuddyAllocator,
    pub base_addr: usize,
    pub total_size: usize,
    pub allocated: AtomicUsize,
    pub cma_glue: Option<CmaBuddyReservationGlue>,
    pub bsd_zone: Option<BsdVmZoneAllocator>,
}

impl SigmaBuddyAllocator {
    pub fn new(base_addr: usize, total_size: usize, max_order: usize) -> Self {
        let total_pages = total_size / PAGE_SIZE;
        SigmaBuddyAllocator {
            inner: SimpleBuddyAllocator::new(max_order, total_pages),
            base_addr,
            total_size,
            allocated: AtomicUsize::new(0),
            cma_glue: Some(CmaBuddyReservationGlue::new(base_addr + (total_size / 2), total_pages / 4)),
            bsd_zone: Some(BsdVmZoneAllocator::new(VmZone::Normal, total_pages)),
        }
    }

    pub fn init(&mut self) {
        self.inner = SimpleBuddyAllocator::new(10, self.total_size / PAGE_SIZE);
    }

    /// Allocates memory block with specified Linux migration type
    pub fn allocate_typed(&mut self, size: usize, migrate_type: MigrateType) -> Option<MemoryBlock> {
        if size == 0 || size > self.total_size {
            return None;
        }

        // Routing logic: CMA allocations route through reserved CMA glue
        if migrate_type == MigrateType::Cma {
            if let Some(ref cma) = self.cma_glue {
                let pages = size.div_ceil(PAGE_SIZE);
                if let Ok(phys_addr) = cma.allocate_contiguous(pages) {
                    self.allocated.fetch_add(pages * PAGE_SIZE, Ordering::SeqCst);
                    use core::ptr::NonNull;
                    return NonNull::new(phys_addr as *mut u8).map(|addr| MemoryBlock {
                        addr,
                        size: pages * PAGE_SIZE,
                    });
                }
            }
        }

        self.allocate(size)
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

                if let Some(ref zone) = self.bsd_zone {
                    let _ = zone.transition_queue(PageQueueType::Free, PageQueueType::Active, pages);
                }

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
        let pages = block.size / PAGE_SIZE;

        // Check if block was allocated in CMA region
        if let Some(ref cma) = self.cma_glue {
            if addr >= cma.cma_base_addr && addr < cma.cma_base_addr + (cma.cma_total_pages * PAGE_SIZE) {
                let _ = cma.release_contiguous(addr, pages);
                self.allocated.fetch_sub(block.size, Ordering::SeqCst);
                return;
            }
        }

        let block_id = (addr - self.base_addr) / PAGE_SIZE;
        let order = Self::calculate_order(pages);
        let _ = self.inner.free(block_id, order);
        self.allocated.fetch_sub(block.size, Ordering::SeqCst);

        if let Some(ref zone) = self.bsd_zone {
            let _ = zone.transition_queue(PageQueueType::Active, PageQueueType::Free, pages);
        }
    }

    /// Evaluates current memory watermarks for anti-fragmentation & compaction
    pub fn evaluate_watermarks(&self) -> WatermarkStatus {
        let total_pages = self.total_size / PAGE_SIZE;
        let used_pages = self.allocated.load(Ordering::SeqCst) / PAGE_SIZE;
        let free_pages = total_pages.saturating_sub(used_pages);

        let min_pages = total_pages / 20;  // 5% min threshold
        let low_pages = total_pages / 10;  // 10% low threshold
        let high_pages = total_pages / 5;  // 20% high threshold

        let level = if free_pages < min_pages {
            WatermarkLevel::WatermarkMin
        } else if free_pages < low_pages {
            WatermarkLevel::WatermarkLow
        } else {
            WatermarkLevel::WatermarkHigh
        };

        WatermarkStatus {
            free_pages,
            min_pages,
            low_pages,
            high_pages,
            level,
            requires_compaction: level != WatermarkLevel::WatermarkHigh,
        }
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

impl Default for SigmaBuddyAllocator {
    fn default() -> Self {
        Self {
            inner: SimpleBuddyAllocator::new(10, 1024),
            base_addr: 0,
            total_size: 0,
            allocated: AtomicUsize::new(0),
            cma_glue: None,
            bsd_zone: None,
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
    fn test_cma_contiguous_memory_reservation_glue() {
        let cma = CmaBuddyReservationGlue::new(0x2000_0000, 256);
        assert_eq!(cma.free_cma_pages(), 256);

        let phys_addr = cma.allocate_contiguous(16).unwrap();
        assert_eq!(phys_addr, 0x2000_0000);
        assert_eq!(cma.free_cma_pages(), 240);

        assert!(cma.release_contiguous(phys_addr, 16).is_ok());
        assert_eq!(cma.free_cma_pages(), 256);
    }

    #[test]
    fn test_freebsd_vm_zone_queue_transitions() {
        let zone = BsdVmZoneAllocator::new(VmZone::Normal, 100);
        assert_eq!(zone.free_pages.load(Ordering::SeqCst), 100);

        assert!(zone.transition_queue(PageQueueType::Free, PageQueueType::Active, 20).is_ok());
        assert_eq!(zone.free_pages.load(Ordering::SeqCst), 80);
        assert_eq!(zone.active_pages.load(Ordering::SeqCst), 20);

        assert!(zone.transition_queue(PageQueueType::Active, PageQueueType::Wired, 5).is_ok());
        assert_eq!(zone.active_pages.load(Ordering::SeqCst), 15);
        assert_eq!(zone.wired_pages.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn test_migrate_type_routing_and_watermark_eval() {
        let mut allocator = SigmaBuddyAllocator::new(0x1000_0000, 4 * 1024 * 1024, 12);
        allocator.init();

        let block = allocator.allocate_typed(4096, MigrateType::Movable);
        assert!(block.is_some());

        let status = allocator.evaluate_watermarks();
        assert_eq!(status.level, WatermarkLevel::WatermarkHigh);
        assert!(!status.requires_compaction);
    }
}
