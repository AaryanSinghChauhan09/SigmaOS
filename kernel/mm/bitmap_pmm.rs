// SPDX-License-Identifier: GPL-3.0-or-later
//! Kernel Physical Memory Bitmap Allocator, Self-Referential Page Tables, and Syscall Trampoline.

#![allow(dead_code)]
#![allow(unused_variables)]

pub const PAGE_SIZE_4096: usize = 4096;
pub const HIGHER_HALF_OFFSET: u64 = 0xFFFFFFFF80000000;
pub const SELF_REF_INDEX: usize = 510;

pub const PAGE_PRESENT: u64 = 1 << 0;
pub const PAGE_WRITE: u64   = 1 << 1;

pub type Pml4Table = [u64; 512];

pub struct SelfReferentialPagingEngine {
    pub pml4_phys_addr: u64,
}

impl SelfReferentialPagingEngine {
    pub fn new(pml4_phys_addr: u64) -> Self {
        Self { pml4_phys_addr }
    }

    pub fn vmm_init_self_reference(&self, pml4_table: &mut Pml4Table) {
        pml4_table[SELF_REF_INDEX] = self.pml4_phys_addr | PAGE_PRESENT | PAGE_WRITE;
    }
}
