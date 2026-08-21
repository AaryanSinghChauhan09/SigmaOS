// SigmaOS BSD-inspired Zone / UMA (Universal Memory Allocator) Allocator
// Implements type-stable object caching for kernel resources

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct ZoneStats {
    pub item_size: usize,
    pub max_items: usize,
    pub allocated_items: usize,
    pub free_items: usize,
    pub total_slabs: usize,
}

pub struct Slab {
    pub start_address: usize,
    pub size: usize,
    pub free_slots: Vec<usize>,
}

impl Slab {
    pub fn new(start_address: usize, slab_size: usize, item_size: usize) -> Self {
        let mut free_slots = Vec::new();
        let num_items = slab_size / item_size;
        for i in 0..num_items {
            free_slots.push(start_address + i * item_size);
        }
        Slab {
            start_address,
            size: slab_size,
            free_slots,
        }
    }

    pub fn alloc(&mut self) -> Option<usize> {
        self.free_slots.pop()
    }

    pub fn free(&mut self, address: usize, item_size: usize) {
        // Double-free protection & boundary check
        if address >= self.start_address && address < self.start_address + self.size {
            if !self.free_slots.contains(&address) {
                self.free_slots.push(address);
            }
        }
    }

    pub fn has_address(&self, address: usize) -> bool {
        address >= self.start_address && address < self.start_address + self.size
    }
}

pub struct Zone {
    pub name: String,
    pub item_size: usize,
    pub max_items: usize,
    pub allocated_items: usize,
    pub slabs: Vec<Slab>,
    next_slab_addr: usize,
}

impl Zone {
    pub fn new(name: &str, item_size: usize, max_items: usize, base_addr: usize) -> Self {
        Zone {
            name: name.to_string(),
            item_size,
            max_items,
            allocated_items: 0,
            slabs: Vec::new(),
            next_slab_addr: base_addr,
        }
    }

    pub fn alloc(&mut self) -> Option<usize> {
        if self.allocated_items >= self.max_items {
            return None; // Limit exceeded
        }

        // 1. Try to allocate from existing slabs
        for slab in &mut self.slabs {
            if let Some(addr) = slab.alloc() {
                self.allocated_items += 1;
                return Some(addr);
            }
        }

        // 2. No free slot in existing slabs, grow by allocating a new slab
        let slab_size = 4096; // Standard 4KB page size
        if self.item_size > slab_size {
            return None; // Item size exceeds standard slab size
        }

        let new_slab = Slab::new(self.next_slab_addr, slab_size, self.item_size);
        self.next_slab_addr += slab_size;
        self.slabs.push(new_slab);

        // Try allocation on the newly added slab
        let last_idx = self.slabs.len() - 1;
        if let Some(addr) = self.slabs[last_idx].alloc() {
            self.allocated_items += 1;
            return Some(addr);
        }

        None
    }

    pub fn free(&mut self, address: usize) -> Result<(), &'static str> {
        for slab in &mut self.slabs {
            if slab.has_address(address) {
                let prev_free = slab.free_slots.len();
                slab.free(address, self.item_size);
                if slab.free_slots.len() > prev_free {
                    self.allocated_items = self.allocated_items.saturating_sub(1);
                    return Ok(());
                } else {
                    return Err("Double free or invalid release");
                }
            }
        }
        Err("Address not found in this zone")
    }

    pub fn stats(&self) -> ZoneStats {
        let total_slots: usize = self.slabs.len() * (4096 / self.item_size);
        let free_items = total_slots.saturating_sub(self.allocated_items);
        ZoneStats {
            item_size: self.item_size,
            max_items: self.max_items,
            allocated_items: self.allocated_items,
            free_items,
            total_slabs: self.slabs.len(),
        }
    }
}

pub struct BsdZoneAllocator {
    pub zones: Vec<Zone>,
    base_heap_address: usize,
}

impl BsdZoneAllocator {
    pub fn new(base_heap_address: usize) -> Self {
        BsdZoneAllocator {
            zones: Vec::new(),
            base_heap_address,
        }
    }

    pub fn uma_zcreate(&mut self, name: &str, item_size: usize, max_items: usize) -> usize {
        let base_addr = self.base_heap_address;
        // Shift base heap address up to avoid overlapping virtual addresses
        self.base_heap_address += max_items * item_size + 4096; // cushion
        let zone = Zone::new(name, item_size, max_items, base_addr);
        let zone_idx = self.zones.len();
        self.zones.push(zone);
        zone_idx
    }

    pub fn uma_zalloc(&mut self, zone_idx: usize) -> Option<usize> {
        if zone_idx < self.zones.len() {
            self.zones[zone_idx].alloc()
        } else {
            None
        }
    }

    pub fn uma_zfree(&mut self, zone_idx: usize, address: usize) -> Result<(), &'static str> {
        if zone_idx < self.zones.len() {
            self.zones[zone_idx].free(address)
        } else {
            Err("Invalid zone index")
        }
    }

    pub fn get_zone_stats(&self, zone_idx: usize) -> Option<ZoneStats> {
        if zone_idx < self.zones.len() {
            Some(self.zones[zone_idx].stats())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_creation_and_alloc() {
        let mut allocator = BsdZoneAllocator::new(0x10000000);
        let thread_zone_idx = allocator.uma_zcreate("ThreadControlBlock", 256, 100);

        let addr1 = allocator.uma_zalloc(thread_zone_idx).unwrap();
        let addr2 = allocator.uma_zalloc(thread_zone_idx).unwrap();

        assert_ne!(addr1, addr2);
        assert!(addr1 >= 0x10000000);

        let stats = allocator.get_zone_stats(thread_zone_idx).unwrap();
        assert_eq!(stats.allocated_items, 2);

        allocator.uma_zfree(thread_zone_idx, addr1).unwrap();
        let stats_after_free = allocator.get_zone_stats(thread_zone_idx).unwrap();
        assert_eq!(stats_after_free.allocated_items, 1);
    }
}
