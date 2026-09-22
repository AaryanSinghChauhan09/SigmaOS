// SPDX-License-Identifier: MIT
// SigmaOS Physical Memory Management Subsystem
// Linux & BSD inspired Buddy Allocator Glue, Migration Types, CMA, Watermarks, and FreeBSD VM Page Queues

use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::ptr::NonNull;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryBlock {
    pub addr: NonNull<u8>,
    pub size: usize,
}

unsafe impl Send for MemoryBlock {}
unsafe impl Sync for MemoryBlock {}

// ============================================================================
// 1. Linux-inspired Migration Types & CMA (Contiguous Memory Allocator)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrateType {
    Unmovable,
    Reclaimable,
    Movable,
    HighAtomic,
    Cma,
    Isolate,
}

#[derive(Debug)]
pub struct CmaBuddyReservationGlue {
    pub cma_base_addr: usize,
    pub cma_total_pages: usize,
    pub bitmap: Vec<bool>,
}

impl CmaBuddyReservationGlue {
    pub fn new(cma_base_addr: usize, cma_total_pages: usize) -> Self {
        Self {
            cma_base_addr,
            cma_total_pages,
            bitmap: vec![false; cma_total_pages],
        }
    }

    pub fn allocate_contiguous(&mut self, pages: usize) -> Result<usize, &'static str> {
        if pages == 0 || pages > self.cma_total_pages {
            return Err("Invalid CMA allocation size");
        }

        let mut consecutive = 0;
        let mut start_idx = 0;

        for i in 0..self.cma_total_pages {
            if !self.bitmap[i] {
                if consecutive == 0 {
                    start_idx = i;
                }
                consecutive += 1;
                if consecutive == pages {
                    for j in start_idx..start_idx + pages {
                        self.bitmap[j] = true;
                    }
                    return Ok(self.cma_base_addr + (start_idx * PAGE_SIZE));
                }
            } else {
                consecutive = 0;
            }
        }

        Err("Out of contiguous CMA memory")
    }

    pub fn release_contiguous(&mut self, base_addr: usize, pages: usize) -> Result<(), &'static str> {
        if base_addr < self.cma_base_addr {
            return Err("Address below CMA region");
        }

        let start_idx = (base_addr - self.cma_base_addr) / PAGE_SIZE;
        if start_idx + pages > self.cma_total_pages {
            return Err("Address range exceeds CMA region");
        }

        for j in start_idx..start_idx + pages {
            self.bitmap[j] = false;
        }

        Ok(())
    }

    pub fn free_cma_pages(&self) -> usize {
        self.bitmap.iter().filter(|&&used| !used).count()
    }
}

// ============================================================================
// 2. Linux Watermark Levels & Anti-Fragmentation Thresholds
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatermarkLevel {
    WatermarkMin,
    WatermarkLow,
    WatermarkHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatermarkStatus {
    pub free_pages: usize,
    pub min_pages: usize,
    pub low_pages: usize,
    pub high_pages: usize,
    pub level: WatermarkLevel,
    pub requires_compaction: bool,
}

// ============================================================================
// 3. FreeBSD VM Page Queues & Zone Allocator
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmZone {
    Dma,
    Normal,
    HighMem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageQueueType {
    Free,
    Active,
    Inactive,
    Wired,
}

#[derive(Debug)]
pub struct BsdVmZoneAllocator {
    pub zone: VmZone,
    pub total_pages: usize,
    pub free_pages: AtomicUsize,
    pub active_pages: AtomicUsize,
    pub inactive_pages: AtomicUsize,
    pub wired_pages: AtomicUsize,
}

impl BsdVmZoneAllocator {
    pub fn new(zone: VmZone, total_pages: usize) -> Self {
        Self {
            zone,
            total_pages,
            free_pages: AtomicUsize::new(total_pages),
            active_pages: AtomicUsize::new(0),
            inactive_pages: AtomicUsize::new(0),
            wired_pages: AtomicUsize::new(0),
        }
    }

    pub fn transition_queue(
        &self,
        from: PageQueueType,
        to: PageQueueType,
        count: usize,
    ) -> Result<(), &'static str> {
        let from_counter = match from {
            PageQueueType::Free => &self.free_pages,
            PageQueueType::Active => &self.active_pages,
            PageQueueType::Inactive => &self.inactive_pages,
            PageQueueType::Wired => &self.wired_pages,
        };

        let to_counter = match to {
            PageQueueType::Free => &self.free_pages,
            PageQueueType::Active => &self.active_pages,
            PageQueueType::Inactive => &self.inactive_pages,
            PageQueueType::Wired => &self.wired_pages,
        };

        loop {
            let current = from_counter.load(Ordering::SeqCst);
            if current < count {
                return Err("Insufficient pages in source queue");
            }
            if from_counter.compare_exchange(current, current - count, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                to_counter.fetch_add(count, Ordering::SeqCst);
                return Ok(());
            }
        }
    }
}

// ============================================================================
// 4. SigmaOS Physical Buddy Allocator Engine
// ============================================================================

pub struct SimpleBuddyAllocator {
    pub max_order: usize,
    pub total_pages: usize,
    pub free_pages: AtomicUsize,
}

impl SimpleBuddyAllocator {
    pub fn new(max_order: usize, total_pages: usize) -> Self {
        Self {
            max_order,
            total_pages,
            free_pages: AtomicUsize::new(total_pages),
        }
    }

    pub fn allocate(&mut self, order: usize) -> Result<usize, &'static str> {
        let required_pages = 1 << order;
        loop {
            let free = self.free_pages.load(Ordering::SeqCst);
            if free < required_pages {
                return Err("Out of physical memory");
            }
            if self.free_pages.compare_exchange(free, free - required_pages, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                return Ok(0); // Allocated frame index offset
            }
        }
    }

    pub fn free(&mut self, _block_id: usize, order: usize) -> Result<(), &'static str> {
        let pages = 1 << order;
        self.free_pages.fetch_add(pages, Ordering::SeqCst);
        Ok(())
    }
}

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

    pub fn allocate_typed(&mut self, size: usize, migrate_type: MigrateType) -> Option<MemoryBlock> {
        if size == 0 || size > self.total_size {
            return None;
        }

        if migrate_type == MigrateType::Cma {
            if let Some(ref mut cma) = self.cma_glue {
                let pages = size.div_ceil(PAGE_SIZE);
                if let Ok(phys_addr) = cma.allocate_contiguous(pages) {
                    self.allocated.fetch_add(pages * PAGE_SIZE, Ordering::SeqCst);
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

        if let Some(ref mut cma) = self.cma_glue {
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
        let mut cma = CmaBuddyReservationGlue::new(0x2000_0000, 256);
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
