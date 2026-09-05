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

/// SigmaOS Huge Pages and hugetlbfs memory support
/// Standard huge pages: 2MB or 1GB configurations to reduce TLB misses
use crate::klib::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HugePageSize {
    Size2Mb,
    Size1Gb,
}

impl HugePageSize {
    pub fn bytes(self) -> usize {
        match self {
            HugePageSize::Size2Mb => 2 * 1024 * 1024,
            HugePageSize::Size1Gb => 1024 * 1024 * 1024,
        }
    }
}

pub struct HugePageManager {
    allocated_pages: BTreeMap<usize, HugePageSize>,
    free_pages: BTreeMap<HugePageSize, usize>,
}

impl HugePageManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut free_pages = BTreeMap::new();
        free_pages.insert(HugePageSize::Size2Mb, 512); // Pre-reserve 1GB worth of 2MB pages
        free_pages.insert(HugePageSize::Size1Gb, 4); // Pre-reserve 4GB worth of 1GB pages

        HugePageManager {
            allocated_pages: BTreeMap::new(),
            free_pages,
        }
    }

    pub fn allocate_huge_page(&mut self, size: HugePageSize) -> Result<usize, &'static str> {
        let available = self
            .free_pages
            .get_mut(&size)
            .ok_or("Invalid huge page size")?;
        if *available == 0 {
            return Err("No huge pages available");
        }

        *available -= 1;

        // Mock virtual address assignment
        let base_addr = match size {
            HugePageSize::Size2Mb => 0xE000_0000 + (*available * 2 * 1024 * 1024),
            HugePageSize::Size1Gb => 0xF000_0000 + (*available * 1024 * 1024 * 1024),
        };

        self.allocated_pages.insert(base_addr, size);
        Ok(base_addr)
    }

    pub fn release_huge_page(&mut self, base_addr: usize) -> Result<(), &'static str> {
        let size = self
            .allocated_pages
            .remove(&base_addr)
            .ok_or("Invalid huge page base address")?;
        let available = self.free_pages.get_mut(&size).ok_or("Invalid state")?;
        *available += 1;
        Ok(())
    }
}

impl Default for HugePageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_huge_pages() {
        let mut hpm = HugePageManager::new();
        let addr = hpm.allocate_huge_page(HugePageSize::Size2Mb).unwrap();
        assert!(addr >= 0xE000_0000);
        hpm.release_huge_page(addr).unwrap();
    }
}
