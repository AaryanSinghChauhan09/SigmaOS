// SigmaOS Kernel Memory Management
// Implements buddy allocator and paging

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// Memory page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Memory block
#[derive(Debug, Clone, Copy)]
pub struct MemoryBlock {
    pub addr: NonNull<u8>,
    pub size: usize,
}

use core::ptr::NonNull;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    Paged,    // Swappable (virtual pages can be swapped out to disk)
    NonPaged, // Always resident in physical memory (for critical drivers and ISRs)
}

#[derive(Debug, Clone)]
pub struct PoolBlock {
    pub addr: usize,
    pub size: usize,
    pub pool_type: PoolType,
    pub tag: [u8; 4], // 4-character driver tag (standard Windows NT Pool Tag, e.g. "File")
}

pub struct KernelPoolManager {
    pub paged_pool: Vec<PoolBlock>,
    pub non_paged_pool: Vec<PoolBlock>,
    pub total_paged_bytes: usize,
    pub total_non_paged_bytes: usize,
}

impl KernelPoolManager {
    pub fn new() -> Self {
        Self {
            paged_pool: Vec::new(),
            non_paged_pool: Vec::new(),
            total_paged_bytes: 0,
            total_non_paged_bytes: 0,
        }
    }

    /// Allocate a block from the specific kernel pool with a pool tag (Inspired by Windows NT ExAllocatePoolWithTag)
    pub fn allocate_pool(&mut self, pool_type: PoolType, size: usize, tag: &[u8; 4]) -> Result<PoolBlock, &'static str> {
        if size == 0 {
            return Err("Cannot allocate 0-byte pool block");
        }

        // Emulate allocating pool virtual address range
        let addr = match pool_type {
            PoolType::Paged => 0xD000_0000 + self.total_paged_bytes,
            PoolType::NonPaged => 0xF000_0000 + self.total_non_paged_bytes,
        };

        let block = PoolBlock {
            addr,
            size,
            pool_type,
            tag: *tag,
        };

        match pool_type {
            PoolType::Paged => {
                self.paged_pool.push(block.clone());
                self.total_paged_bytes += size;
            }
            PoolType::NonPaged => {
                self.non_paged_pool.push(block.clone());
                self.total_non_paged_bytes += size;
            }
        }

        println!(
            "Windows NT Pool Alloc: Allocated {:?} pool block of {} bytes with tag '{}' at address 0x{:X}",
            pool_type, size, core::str::from_utf8(tag).unwrap_or("????"), addr
        );

        Ok(block)
    }

    /// Free a block from the kernel pool (Inspired by Windows NT ExFreePool)
    pub fn free_pool(&mut self, addr: usize) -> Result<(), &'static str> {
        if let Some(pos) = self.paged_pool.iter().position(|b| b.addr == addr) {
            let block = self.paged_pool.remove(pos);
            self.total_paged_bytes -= block.size;
            Ok(())
        } else if let Some(pos) = self.non_paged_pool.iter().position(|b| b.addr == addr) {
            let block = self.non_paged_pool.remove(pos);
            self.total_non_paged_bytes -= block.size;
            Ok(())
        } else {
            Err("Invalid pool address; double free or corruption detected")
        }
    }
}

impl Default for KernelPoolManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Zone {
    pub present_pages: u64,
}

pub struct Page {
    pub flags: AtomicUsize,
    pub count: AtomicUsize,
    pub mapping: Option<usize>,
    pub index: u64,
    pub private: Option<usize>,
    pub zone: Option<*const Zone>,
}

impl Page {
    pub fn dec_ref(&self) -> bool {
        self.count.fetch_sub(1, Ordering::SeqCst) == 1
    }
}

pub struct BuddyAllocator {
    pub free_lists: [Vec<MemoryBlock>; 12],
    pub free_pages: usize,
    pub total_pages: usize,
    pub zones: Vec<Zone>,
}

impl BuddyAllocator {
    pub fn new() -> Self {
        Self {
            free_lists: Default::default(),
            free_pages: 0,
            total_pages: 0,
            zones: Vec::new(),
        }
    }

    pub fn initialize_memory(&mut self, base_addr: usize, size: usize) {
        let pages = size / PAGE_SIZE;
        let order = self.calculate_order(pages);

        if order < 12 {
            if let Some(addr) = NonNull::new(base_addr as *mut u8) {
                let block = MemoryBlock { addr, size };
                self.free_lists[order].push(block);
            }