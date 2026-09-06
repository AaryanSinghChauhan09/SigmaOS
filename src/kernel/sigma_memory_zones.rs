#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! # SigmaOS Zone-Based Memory Allocator
//!
//! A zone-based physical memory allocator for SigmaOS, inspired by the Linux buddy
//! allocator and BSD UMA (Universal Memory Allocator).
//!
//! ## Architecture
//! Physical memory is divided into [`MemoryZone`]s.  Within each zone a
//! [`BuddyAllocator`] manages pages in power-of-two "orders" (0 = 4 KiB, up to
//! 11 = 8 MiB).  A [`SlabCache`] sits on top for sub-page, fixed-size object
//! allocation.
//!
//! ## Zone Watermarks
//! Each zone tracks `min`, `low`, and `high` watermarks (in pages).  When the
//! free-page count drops below `min`, allocation fails; below `low`, background
//! reclaim is hinted; below `high`, proactive reclaim is hinted.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::{HashMap, VecDeque};
use std::vec::Vec;
use std::string::String;

// ── Constants ────────────────────────────────────────────────────────────────

/// Base page size in bytes (4 KiB).
pub const PAGE_SIZE: usize = 4096;

/// Maximum buddy order supported (order 11 = 2^11 pages = 8 MiB block).
pub const MAX_ORDER: usize = 12; // exclusive upper bound; orders 0..=11

/// Sentinel value for "no physical address allocated".
pub const INVALID_ADDR: usize = usize::MAX;

// ── MemoryZone ───────────────────────────────────────────────────────────────

/// Physical memory zone classification.
///
/// Mirrors the Linux `ZONE_DMA`, `ZONE_NORMAL`, `ZONE_HIGHMEM` model plus a
/// device-memory zone for MMIO/persistent memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryZone {
    /// ISA DMA-able memory (0 – 16 MiB on x86).
    Dma,
    /// Normal kernel-addressable RAM (16 MiB – 896 MiB on 32-bit x86, all 64-bit).
    Normal,
    /// High memory, not permanently mapped in 32-bit kernels.
    HighMem,
    /// Device / persistent memory (MMIO, CXL, PMEM).
    Device,
}

impl MemoryZone {
    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            MemoryZone::Dma => "DMA",
            MemoryZone::Normal => "Normal",
            MemoryZone::HighMem => "HighMem",
            MemoryZone::Device => "Device",
        }
    }
}

// ── ZoneWatermarks ───────────────────────────────────────────────────────────

/// Watermark thresholds for a memory zone (in pages).
///
/// | Level | Meaning                                         |
/// |-------|-------------------------------------------------|
/// | `min` | Hard floor; allocation fails below this level. |
/// | `low` | Trigger background (kswapd-equivalent) reclaim. |
/// | `high` | Target after reclaim; reclaim stops here.      |
#[derive(Debug, Clone)]
pub struct ZoneWatermarks {
    /// Minimum free pages; below this allocation returns `None`.
    pub min: usize,
    /// Low watermark; below this, background reclaim is triggered.
    pub low: usize,
    /// High watermark; reclaim target.
    pub high: usize,
}

impl ZoneWatermarks {
    /// Create watermarks scaled to `total_pages`.
    ///
    /// Default fractions: min = 1%, low = 2.5%, high = 5%.
    pub fn from_total(total_pages: usize) -> Self {
        let min = (total_pages / 100).max(4);
        let low = (total_pages * 5 / 200).max(min + 4);
        let high = (total_pages / 20).max(low + 4);
        ZoneWatermarks { min, low, high }
    }

    /// Classify the current pressure level given `free_pages`.
    pub fn pressure(&self, free_pages: usize) -> PressureLevel {
        if free_pages < self.min {
            PressureLevel::Critical
        } else if free_pages < self.low {
            PressureLevel::High
        } else if free_pages < self.high {
            PressureLevel::Moderate
        } else {
            PressureLevel::Normal
        }
    }
}

/// Memory pressure classification returned by [`ZoneWatermarks::pressure`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureLevel {
    /// Plenty of free pages; no reclaim needed.
    Normal,
    /// Free pages below `high`; proactive reclaim is beneficial.
    Moderate,
    /// Free pages below `low`; background reclaim must run.
    High,
    /// Free pages below `min`; allocation will fail.
    Critical,
}

// ── BuddyAllocator ───────────────────────────────────────────────────────────

/// Buddy-system physical page allocator.
///
/// Free pages are grouped into order-`n` blocks (each block = 2^n pages).
/// Allocation splits higher-order free blocks down to the requested order.
/// Freeing merges adjacent buddies back to higher orders.
///
/// # Addresses
/// All addresses are *physical frame numbers* (page address / PAGE_SIZE) cast
/// to `usize`.  Multiply by `PAGE_SIZE` to get byte addresses.
#[derive(Debug)]
pub struct BuddyAllocator {
    /// Free lists per order: `free_lists[order]` holds free block base addresses.
    free_lists: [VecDeque<usize>; MAX_ORDER],
    /// Total number of pages managed.
    pub total_pages: usize,
    /// Number of currently free pages.
    pub free_pages: usize,
    /// Zone this allocator serves.
    pub zone: MemoryZone,
    /// Watermarks for this zone.
    pub watermarks: ZoneWatermarks,
}

impl BuddyAllocator {
    /// Create a buddy allocator for a contiguous region starting at `base_addr`
    /// (page-aligned) spanning `total_pages` pages, belonging to `zone`.
    ///
    /// The entire region is inserted into the free list at the largest possible order.
    pub fn new(zone: MemoryZone, base_addr: usize, total_pages: usize) -> Self {
        // BTreeMap is not const-initializable; use array init trick.
        let free_lists: [VecDeque<usize>; MAX_ORDER] =
            std::array::from_fn(|_| VecDeque::new());

        let watermarks = ZoneWatermarks::from_total(total_pages);

        let mut alloc = BuddyAllocator {
            free_lists,
            total_pages,
            free_pages: 0,
            zone,
            watermarks,
        };

        // Insert region into free lists using largest aligned blocks.
        alloc.add_free_region(base_addr, total_pages);
        alloc
    }

    /// Insert a contiguous free region into the buddy lists.
    fn add_free_region(&mut self, mut addr: usize, mut pages: usize) {
        while pages > 0 {
            // Find largest order that fits and is naturally aligned.
            let order = (0..MAX_ORDER)
                .rev()
                .find(|&o| {
                    let block_pages = 1usize << o;
                    pages >= block_pages && (addr & (block_pages - 1)) == 0
                })
                .unwrap_or(0);

            let block_pages = 1usize << order;
            self.free_lists[order].push_back(addr);
            self.free_pages += block_pages;
            addr += block_pages;
            pages -= block_pages;
        }
    }

    /// Allocate 2^`order` contiguous pages.
    ///
    /// Returns the physical base address of the block, or `None` if unavailable
    /// (OOM or below `min` watermark).
    pub fn alloc_pages(&mut self, order: usize) -> Option<usize> {
        if order >= MAX_ORDER {
            return None;
        }

        // Watermark check.
        let needed = 1usize << order;
        if self.free_pages < self.watermarks.min + needed {
            return None;
        }

        // Find the smallest available order >= requested order.
        let avail_order = (order..MAX_ORDER)
            .find(|&o| !self.free_lists[o].is_empty())?;

        let mut block = self.free_lists[avail_order].pop_front().unwrap();
        self.free_pages -= 1 << avail_order;

        // Split down to the requested order.
        let mut current_order = avail_order;
        while current_order > order {
            current_order -= 1;
            // The buddy is at block + 2^current_order.
            let buddy = block + (1 << current_order);
            self.free_lists[current_order].push_front(buddy);
            self.free_pages += 1 << current_order;
        }

        self.free_pages -= 1 << order; // already subtracted avail_order above; fix double count
        // Recalculate: we subtracted 2^avail_order, then added back split buddies.
        // The net is: we consumed 2^order pages.
        // Re-add the over-subtracted amount:
        self.free_pages += (1 << avail_order) - (1 << order);

        Some(block * PAGE_SIZE)
    }

    /// Free a block of 2^`order` pages at physical `addr`.
    ///
    /// Attempts to merge with the buddy block repeatedly up to `MAX_ORDER - 1`.
    pub fn free_pages(&mut self, addr: usize, order: usize) {
        if order >= MAX_ORDER {
            return;
        }

        let mut block = addr / PAGE_SIZE;
        let mut current_order = order;

        self.free_pages += 1 << current_order;

        // Merge with buddy while possible.
        while current_order < MAX_ORDER - 1 {
            let buddy = block ^ (1 << current_order);
            // Find buddy in the free list.
            let pos = self.free_lists[current_order]
                .iter()
                .position(|&b| b == buddy);

            if let Some(idx) = pos {
                self.free_lists[current_order].remove(idx);
                self.free_pages -= 1 << current_order; // buddy removed from free count
                // Merge: lower block address becomes the merged block.
                block = block.min(buddy);
                current_order += 1;
            } else {
                break;
            }
        }

        self.free_lists[current_order].push_back(block);
    }

    /// Return current memory pressure level.
    pub fn pressure(&self) -> PressureLevel {
        self.watermarks.pressure(self.free_pages)
    }

    /// Return free page count.
    pub fn free_count(&self) -> usize {
        self.free_pages
    }
}

// ── SlabCache ────────────────────────────────────────────────────────────────

/// Fixed-size object slab cache.
///
/// Maintains a pool of equally-sized memory objects backed by pages from a
/// [`BuddyAllocator`].  Objects are raw `usize` addresses (simulating pointers
/// in a no-alloc context).
///
/// Inspired by the BSD UMA and Linux slab/slub allocators.
#[derive(Debug)]
pub struct SlabCache {
    /// Name of this cache (for diagnostics).
    pub name: String,
    /// Size of each object in bytes (rounded up to alignment).
    pub obj_size: usize,
    /// Alignment requirement in bytes.
    pub alignment: usize,
    /// Free object addresses.
    free_objects: VecDeque<usize>,
    /// Total objects ever allocated into this cache.
    pub total_allocated: usize,
    /// Total objects currently in use.
    pub in_use: usize,
}

impl SlabCache {
    /// Create a new slab cache with the given object size and alignment.
    ///
    /// `alignment` must be a power of two; if 0 it defaults to 8.
    pub fn new(name: impl Into<String>, obj_size: usize, alignment: usize) -> Self {
        let alignment = if alignment == 0 || !alignment.is_power_of_two() {
            8
        } else {
            alignment
        };
        // Round up obj_size to alignment.
        let obj_size = (obj_size + alignment - 1) & !(alignment - 1);
        SlabCache {
            name: name.into(),
            obj_size,
            alignment,
            free_objects: VecDeque::new(),
            total_allocated: 0,
            in_use: 0,
        }
    }

    /// Pre-populate the cache with objects from a pre-allocated memory slab.
    ///
    /// `slab_base` is the page-aligned base address; `slab_size` is in bytes.
    pub fn grow(&mut self, slab_base: usize, slab_size: usize) {
        let n = slab_size / self.obj_size;
        for i in 0..n {
            self.free_objects.push_back(slab_base + i * self.obj_size);
            self.total_allocated += 1;
        }
    }

    /// Allocate one object from the cache.
    ///
    /// Returns the object's address, or `None` if the cache is empty.
    pub fn alloc(&mut self) -> Option<usize> {
        let addr = self.free_objects.pop_front()?;
        self.in_use += 1;
        Some(addr)
    }

    /// Return an object to the cache.
    pub fn free(&mut self, addr: usize) {
        self.free_objects.push_back(addr);
        self.in_use = self.in_use.saturating_sub(1);
    }

    /// Number of free objects in the cache.
    pub fn free_count(&self) -> usize {
        self.free_objects.len()
    }
}

// ── ZoneAllocator ─────────────────────────────────────────────────────────────

/// Top-level allocator combining multiple zones and slab caches.
///
/// Allocation attempts zones in preference order: DMA < Normal < HighMem < Device
/// (lower zones are tried last to preserve DMA memory).
pub struct ZoneAllocator {
    /// Per-zone buddy allocators.
    pub zones: HashMap<MemoryZone, BuddyAllocator>,
    /// Named slab caches.
    pub slab_caches: HashMap<String, SlabCache>,
}

impl ZoneAllocator {
    /// Create a zone allocator from a list of zone descriptors.
    ///
    /// Each entry: `(zone, base_addr_bytes, total_pages)`.
    pub fn new(zone_descs: &[(MemoryZone, usize, usize)]) -> Self {
        let mut zones = HashMap::new();
        for &(zone, base, pages) in zone_descs {
            zones.insert(zone, BuddyAllocator::new(zone, base / PAGE_SIZE, pages));
        }
        ZoneAllocator {
            zones,
            slab_caches: HashMap::new(),
        }
    }

    /// Allocate `2^order` pages from the best available zone.
    ///
    /// Prefers `Normal`, falls back to `HighMem`, then `Device`, avoids `Dma`.
    pub fn alloc_pages(&mut self, order: usize) -> Option<usize> {
        let preference = [
            MemoryZone::Normal,
            MemoryZone::HighMem,
            MemoryZone::Device,
            MemoryZone::Dma,
        ];
        for &zone in &preference {
            if let Some(buddy) = self.zones.get_mut(&zone) {
                if let Some(addr) = buddy.alloc_pages(order) {
                    return Some(addr);
                }
            }
        }
        None
    }

    /// Free pages back to whichever zone owns that address range.
    ///
    /// Caller must track which zone an address belongs to.
    pub fn free_pages(&mut self, zone: MemoryZone, addr: usize, order: usize) {
        if let Some(buddy) = self.zones.get_mut(&zone) {
            buddy.free_pages(addr, order);
        }
    }

    /// Register a new slab cache.
    pub fn register_cache(&mut self, cache: SlabCache) {
        self.slab_caches.insert(cache.name.clone(), cache);
    }

    /// Allocate one object from a named slab cache.
    pub fn slab_alloc(&mut self, name: &str) -> Option<usize> {
        self.slab_caches.get_mut(name)?.alloc()
    }

    /// Free one object to a named slab cache.
    pub fn slab_free(&mut self, name: &str, addr: usize) {
        if let Some(cache) = self.slab_caches.get_mut(name) {
            cache.free(addr);
        }
    }

    /// Print a summary of zone free-page counts to a `String`.
    pub fn stats(&self) -> String {
        let mut out = String::from("Zone Allocator Stats:\n");
        for (zone, buddy) in &self.zones {
            out.push_str(&format!(
                "  {:8} : free={} / total={} [pressure={:?}]\n",
                zone.name(),
                buddy.free_count(),
                buddy.total_pages,
                buddy.pressure(),
            ));
        }
        out
    }
}

// ── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buddy_alloc_free() {
        // 64 pages of Normal memory starting at page frame 0.
        let mut buddy = BuddyAllocator::new(MemoryZone::Normal, 0, 64);
        let initial_free = buddy.free_count();

        // Allocate 4 pages (order 2).
        let addr = buddy.alloc_pages(2);
        assert!(addr.is_some(), "should allocate order-2 block");
        assert_eq!(buddy.free_count(), initial_free - 4);

        // Free them back.
        buddy.free_pages(addr.unwrap(), 2);
        assert_eq!(buddy.free_count(), initial_free);
    }

    #[test]
    fn test_buddy_order0() {
        let mut buddy = BuddyAllocator::new(MemoryZone::Normal, 0, 16);
        // Allocate single pages.
        let mut addrs: Vec<usize> = (0..8).filter_map(|_| buddy.alloc_pages(0)).collect();
        assert_eq!(addrs.len(), 8);
        // Free all back.
        for a in addrs.drain(..) {
            buddy.free_pages(a, 0);
        }
        // Free count should be restored.
        assert_eq!(buddy.free_count(), 16);
    }

    #[test]
    fn test_slab_cache() {
        let mut cache = SlabCache::new("test_cache", 64, 8);
        // Grow with 4096 bytes = 64 objects of size 64.
        cache.grow(0x1000_0000, 4096);
        assert_eq!(cache.free_count(), 64);

        let addr = cache.alloc().unwrap();
        assert_eq!(cache.in_use, 1);
        cache.free(addr);
        assert_eq!(cache.in_use, 0);
    }

    #[test]
    fn test_watermarks() {
        let wm = ZoneWatermarks::from_total(1024);
        assert!(wm.min < wm.low);
        assert!(wm.low < wm.high);

        assert_eq!(wm.pressure(0), PressureLevel::Critical);
        assert_eq!(wm.pressure(wm.high + 1), PressureLevel::Normal);
    }

    #[test]
    fn test_zone_allocator() {
        let descs = [
            (MemoryZone::Dma, 0, 16),
            (MemoryZone::Normal, 16 * PAGE_SIZE, 512),
        ];
        let mut za = ZoneAllocator::new(&descs);
        let addr = za.alloc_pages(1);
        // Should pick Normal zone.
        assert!(addr.is_some());
    }
}
