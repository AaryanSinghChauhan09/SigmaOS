// SigmaOS Ioremap Subsystem
// Provides kernel virtual-to-physical MMIO region mapping with non-cached/write-combining flags.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Memory mapping caching attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFlags {
    NoCache,
    WriteCombining,
    WriteThrough,
    WriteBack,
    ReadOnly,
}

/// A mapped MMIO memory region
#[derive(Debug, Clone)]
pub struct IoremapRegion {
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub size: usize,
    pub flags: PageFlags,
    pub is_mapped: bool,
}

/// Dynamic Ioremap MMIO Allocator & Page Table Binder
#[derive(Debug, Clone)]
pub struct IoremapAllocator {
    pub base_virt_addr: u64,
    pub current_offset: u64,
    pub regions: Vec<IoremapRegion>,
}

impl IoremapAllocator {
    pub fn new(base_virt_addr: u64) -> Self {
        Self {
            base_virt_addr,
            current_offset: 0,
            regions: Vec::new(),
        }
    }

    /// Map physical MMIO range into kernel virtual address space with specific caching attributes
    pub fn map_mmio(&mut self, phys_addr: u64, size: usize, flags: PageFlags) -> u64 {
        // Check if already mapped
        for reg in &self.regions {
            if reg.phys_addr == phys_addr && reg.size >= size && reg.flags == flags && reg.is_mapped {
                return reg.virt_addr;
            }
        }

        let aligned_size = (size + 4095) & !4095;
        let virt_addr = self.base_virt_addr + self.current_offset;
        self.current_offset += aligned_size as u64;

        let region = IoremapRegion {
            phys_addr,
            virt_addr,
            size: aligned_size,
            flags,
            is_mapped: true,
        };

        self.regions.push(region);
        virt_addr
    }

    /// Convenience wrapper for standard non-cached MMIO mapping (ioremap_nocache)
    pub fn ioremap_nocache(&mut self, phys_addr: u64, size: usize) -> u64 {
        self.map_mmio(phys_addr, size, PageFlags::NoCache)
    }

    /// Convenience wrapper for write-combining framebuffer MMIO mapping (ioremap_wc)
    pub fn ioremap_wc(&mut self, phys_addr: u64, size: usize) -> u64 {
        self.map_mmio(phys_addr, size, PageFlags::WriteCombining)
    }

    /// Unmap an MMIO virtual address region
    pub fn iounmap(&mut self, virt_addr: u64) -> bool {
        for reg in &mut self.regions {
            if reg.virt_addr == virt_addr && reg.is_mapped {
                reg.is_mapped = false;
                return true;
            }
        }
        false
    }

    /// Look up physical address for a mapped virtual address
    pub fn virt_to_phys(&self, virt_addr: u64) -> Option<u64> {
        for reg in &self.regions {
            if reg.is_mapped && virt_addr >= reg.virt_addr && virt_addr < reg.virt_addr + reg.size as u64 {
                let offset = virt_addr - reg.virt_addr;
                return Some(reg.phys_addr + offset);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioremap_allocation_and_lookup() {
        let mut allocator = IoremapAllocator::new(0xFFFF_8000_0000_0000);
        let bar0_phys = 0xF720_0000;
        let virt = allocator.ioremap_nocache(bar0_phys, 0x4000);

        assert_eq!(virt, 0xFFFF_8000_0000_0000);
        assert_eq!(allocator.virt_to_phys(virt + 0x100), Some(bar0_phys + 0x100));

        assert!(allocator.iounmap(virt));
        assert_eq!(allocator.virt_to_phys(virt), None);
    }
}
