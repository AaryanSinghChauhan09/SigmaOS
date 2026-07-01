// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Physical Memory Manager (Rust, no_std)
//! =========================================================================

type U32 = u32;
type U64 = u64;

const PAGE_SIZE: U64 = 4096;
const BITMAP_SIZE: usize = 1024 * 1024 / 8; // Supports up to 4GB of RAM at 4KB pages
const NULL_PTR: *mut u8 = core::ptr::null_mut();

pub struct SovereignPmm {
    bitmap: [U32; BITMAP_SIZE],
}

impl SovereignPmm {
    pub const fn new() -> Self {
        SovereignPmm {
            bitmap: [0; BITMAP_SIZE],
        }
    }

    pub fn init(&mut self, _mem_size: U64) {
        // Zero out bitmap
        let mut i = 0;
        while i < BITMAP_SIZE {
            self.bitmap[i] = 0;
            i += 1;
        }

        // Lock first 2MB (0x200000) for kernel area
        let mut addr: U64 = 0;
        while addr < 0x200000 {
            self.lock_page(addr);
            addr += PAGE_SIZE;
        }
    }

    pub fn allocate_page(&mut self) -> *mut u8 {
        let mut i = 0;
        while i < BITMAP_SIZE {
            if self.bitmap[i] != 0xFFFFFFFF {
                let mut j = 0;
                while j < 32 {
                    if (self.bitmap[i] & (1 << j)) == 0 {
                        let addr = (i as U64 * 32 + j as U64) * PAGE_SIZE;
                        self.lock_page(addr);
                        return addr as *mut u8;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        NULL_PTR
    }

    pub fn lock_page(&mut self, addr: U64) {
        let index = (addr / PAGE_SIZE) as usize;
        self.bitmap[index / 32] |= 1 << (index % 32);
    }

    pub fn unlock_page(&mut self, addr: U64) {
        let index = (addr / PAGE_SIZE) as usize;
        self.bitmap[index / 32] &= !(1 << (index % 32));
    }

    pub fn compact_memory(&mut self) {
        // Placeholder for memory compaction logic
    }

    pub fn get_used_memory(&self) -> U64 {
        let mut used: U64 = 0;
        let mut i = 0;
        while i < BITMAP_SIZE {
            if self.bitmap[i] != 0 {
                let mut j = 0;
                while j < 32 {
                    if (self.bitmap[i] & (1 << j)) != 0 {
                        used += 1;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        used * PAGE_SIZE
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_PMM: SovereignPmm = SovereignPmm::new();

// ── C-ABI Exports (Replacing SovereignPMM.cpp) ─────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn pmm_init_shard(mem_size: U64) {
    G_PMM.init(mem_size);
}

#[no_mangle]
pub unsafe extern "C" fn pmm_alloc_shard() -> *mut u8 {
    G_PMM.allocate_page()
}

#[no_mangle]
pub unsafe extern "C" fn pmm_free_shard(addr: *mut u8) {
    G_PMM.unlock_page(addr as U64);
}

#[no_mangle]
pub unsafe extern "C" fn pmm_compact_shard() {
    G_PMM.compact_memory();
}

#[no_mangle]
pub unsafe extern "C" fn pmm_get_used_shard() -> U64 {
    G_PMM.get_used_memory()
}
