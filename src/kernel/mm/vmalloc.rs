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

extern crate alloc;
/// SigmaOS vmalloc interface for virtual contiguous memory allocation
/// Maps non-contiguous physical pages into contiguous virtual space
use crate::klib::BTreeMap;
use alloc::vec::Vec;

pub struct VmallocRegion {
    pub start_addr: usize,
    pub size: usize,
    pub pages: Vec<usize>, // Physical page addresses
}

pub struct VmallocManager {
    allocated_regions: BTreeMap<usize, VmallocRegion>,
    next_virtual_addr: usize,
}

impl VmallocManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        VmallocManager {
            allocated_regions: BTreeMap::new(),
            next_virtual_addr: 0xD000_0000, // Typically high-memory space
        }
    }

    pub fn vmalloc(&mut self, size: usize) -> Option<usize> {
        if size == 0 {
            return None;
        }

        // Align size to page boundary (4KB)
        let page_size = 4096;
        let num_pages = (size + page_size - 1) / page_size;
        let aligned_size = num_pages * page_size;

        let start_addr = self.next_virtual_addr;

        // Mock physical page allocation (e.g. 0x10000, 0x11000, etc.)
        let mut pages = Vec::with_capacity(num_pages);
        for i in 0..num_pages {
            pages.push(0x1000_0000 + i * page_size);
        }

        let region = VmallocRegion {
            start_addr,
            size: aligned_size,
            pages,
        };

        self.allocated_regions.insert(start_addr, region);
        self.next_virtual_addr += aligned_size;

        Some(start_addr)
    }

    pub fn vfree(&mut self, start_addr: usize) -> Result<(), &'static str> {
        if self.allocated_regions.remove(&start_addr).is_some() {
            Ok(())
        } else {
            Err("Address not allocated via vmalloc")
        }
    }

    pub fn get_pages(&self, start_addr: usize) -> Option<&Vec<usize>> {
        self.allocated_regions.get(&start_addr).map(|r| &r.pages)
    }
}

impl Default for VmallocManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vmalloc_lifecycle() {
        let mut vmm = VmallocManager::new();
        let addr = vmm.vmalloc(10000).unwrap(); // Will align to 3 pages (12288 bytes)
        assert_eq!(addr, 0xD000_0000);

        let pages = vmm.get_pages(addr).unwrap();
        assert_eq!(pages.len(), 3);

        vmm.vfree(addr).unwrap();
        assert!(vmm.vfree(addr).is_err());
    }
}
