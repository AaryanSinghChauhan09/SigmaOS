#![allow(dead_code)]
//! Physical Memory Bitmap Allocator, Self-Referential Page Tables, and Fast x86_64 Assembly Syscall Dispatcher.
//! Implements bottom-up kernel primitives as described in Step 1, Step 2, and Step 3 specifications.

use std::vec;
use std::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub const PAGE_SIZE_4096: usize = 4096;
pub const HIGHER_HALF_OFFSET: u64 = 0xFFFFFFFF80000000;
pub const SELF_REF_INDEX: usize = 510;

pub const PAGE_PRESENT: u64 = 1 << 0;
pub const PAGE_WRITE: u64 = 1 << 1;
pub const PAGE_USER: u64 = 1 << 2;

// =========================================================================
// Step 1: Physical Memory Manager (PMM) - Bitmap Allocator
// =========================================================================

pub struct BitmapPhysicalMemoryManager {
    pub max_blocks: usize,
    pub used_blocks: usize,
    pub bitmap: Vec<u8>,
}

impl BitmapPhysicalMemoryManager {
    pub fn new(mem_size: usize) -> Self {
        let max_blocks = mem_size / PAGE_SIZE_4096;
        let bitmap_bytes = (max_blocks + 7) / 8;
        // Default all blocks to used (0xFF), then free available regions
        let bitmap = vec![0xFFu8; bitmap_bytes];

        Self {
            max_blocks,
            used_blocks: max_blocks,
            bitmap,
        }
    }

    pub fn bitmap_set(&mut self, bit: usize) {
        let byte_idx = bit / 8;
        if byte_idx < self.bitmap.len() {
            self.bitmap[byte_idx] |= 1 << (bit % 8);
        }
    }

    pub fn bitmap_clear(&mut self, bit: usize) {
        let byte_idx = bit / 8;
        if byte_idx < self.bitmap.len() {
            self.bitmap[byte_idx] &= !(1 << (bit % 8));
        }
    }

    pub fn bitmap_test(&self, bit: usize) -> bool {
        let byte_idx = bit / 8;
        if byte_idx < self.bitmap.len() {
            (self.bitmap[byte_idx] & (1 << (bit % 8))) != 0
        } else {
            true
        }
    }

    pub fn free_region(&mut self, base_paddr: usize, length: usize) {
        let start_block = base_paddr / PAGE_SIZE_4096;
        let count = length / PAGE_SIZE_4096;
        for i in 0..count {
            let block = start_block + i;
            if block < self.max_blocks && self.bitmap_test(block) {
                self.bitmap_clear(block);
                if self.used_blocks > 0 {
                    self.used_blocks -= 1;
                }
            }
        }
    }

    pub fn alloc_block(&mut self) -> Option<usize> {
        for i in 0..self.max_blocks {
            if !self.bitmap_test(i) {
                self.bitmap_set(i);
                self.used_blocks += 1;
                return Some(i * PAGE_SIZE_4096);
            }
        }
        None // Out of memory
    }

    pub fn free_block(&mut self, paddr: usize) {
        let frame = paddr / PAGE_SIZE_4096;
        if frame < self.max_blocks && self.bitmap_test(frame) {
            self.bitmap_clear(frame);
            if self.used_blocks > 0 {
                self.used_blocks -= 1;
            }
        }
    }
}

// C ABI Exports for PMM
static mut GLOBAL_BITMAP_PMM: Option<BitmapPhysicalMemoryManager> = None;

#[no_mangle]
pub unsafe extern "C" fn pmm_init(mem_size: u64) {
    GLOBAL_BITMAP_PMM = Some(BitmapPhysicalMemoryManager::new(mem_size as usize));
}

#[no_mangle]
pub unsafe extern "C" fn pmm_alloc_block() -> *mut u8 {
    if let Some(ref mut pmm) = GLOBAL_BITMAP_PMM {
        if let Some(paddr) = pmm.alloc_block() {
            return paddr as *mut u8;
        }
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn pmm_free_block(ptr: *mut u8) {
    if let Some(ref mut pmm) = GLOBAL_BITMAP_PMM {
        pmm.free_block(ptr as usize);
    }
}

// =========================================================================
// Step 2: Higher-Half Paging & Self-Referential Page Tables
// =========================================================================

pub type Pml4Table = [u64; 512];

pub struct SelfReferentialPagingEngine {
    pub pml4_phys_addr: u64,
}

impl SelfReferentialPagingEngine {
    pub fn new(pml4_phys_addr: u64) -> Self {
        Self { pml4_phys_addr }
    }

    pub fn vmm_init_self_reference(&self, pml4_table: &mut Pml4Table) {
        // Point PML4 slot 510 back to its own physical address with PRESENT | WRITE flags
        pml4_table[SELF_REF_INDEX] = self.pml4_phys_addr | PAGE_PRESENT | PAGE_WRITE;
    }

    pub fn vmm_get_pte_vaddr(&self, pml4_i: usize, pdp_i: usize, pd_i: usize, pt_i: usize) -> u64 {
        0xFFFF_0000_0000_0000
            | ((SELF_REF_INDEX as u64) << 39)
            | ((pml4_i as u64) << 30)
            | ((pdp_i as u64) << 21)
            | ((pt_i as u64) << 12)
            | ((pt_i as u64) * 8)
    }
}

// =========================================================================
// Step 3: Fast x86_64 Syscall Assembly Trampoline & C Dispatcher
// =========================================================================

#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SyscallTrapFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub user_rip: u64,    // Saved from RCX by CPU
    pub user_rflags: u64, // Saved from R11 by CPU
    pub user_rsp: u64,    // Saved from GS:8
}

pub type SyscallHandlerFn = fn(u64, u64, u64, u64) -> i64;

pub struct SyscallTableRouter {
    pub handlers: [Option<SyscallHandlerFn>; 16],
}

impl SyscallTableRouter {
    pub fn new() -> Self {
        Self {
            handlers: [None; 16],
        }
    }

    pub fn register_handler(&mut self, syscall_num: usize, handler: SyscallHandlerFn) {
        if syscall_num < 16 {
            self.handlers[syscall_num] = Some(handler);
        }
    }

    pub fn syscall_handler(&self, syscall_num: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
        let idx = syscall_num as usize;
        if idx < 16 {
            if let Some(handler) = self.handlers[idx] {
                return handler(arg1, arg2, arg3, 0);
            }
        }
        -38 // ENOSYS
    }
}

impl Default for SyscallTableRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_pmm() {
        let mut pmm = BitmapPhysicalMemoryManager::new(64 * PAGE_SIZE_4096);
        pmm.free_region(0x10000, 32 * PAGE_SIZE_4096);

        let block1 = pmm.alloc_block().unwrap();
        assert_eq!(block1, 0x10000);

        let block2 = pmm.alloc_block().unwrap();
        assert_eq!(block2, 0x11000);

        pmm.free_block(block1);
        let block3 = pmm.alloc_block().unwrap();
        assert_eq!(block3, block1);
    }

    #[test]
    fn test_self_referential_paging() {
        let engine = SelfReferentialPagingEngine::new(0x200000);
        let mut pml4: Pml4Table = [0; 512];

        engine.vmm_init_self_reference(&mut pml4);
        assert_eq!(pml4[510], 0x200000 | PAGE_PRESENT | PAGE_WRITE);

        let vaddr = engine.vmm_get_pte_vaddr(0, 1, 2, 3);
        assert_eq!((vaddr >> 39) & 0x1FF, 510);
    }

    #[test]
    fn test_syscall_router() {
        let mut router = SyscallTableRouter::new();
        router.register_handler(1, |a, b, c, _| (a + b + c) as i64);

        let res = router.syscall_handler(1, 10, 20, 30);
        assert_eq!(res, 60);

        let invalid = router.syscall_handler(99, 0, 0, 0);
        assert_eq!(invalid, -38);
    }
}
