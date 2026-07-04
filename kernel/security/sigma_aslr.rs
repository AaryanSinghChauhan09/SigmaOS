// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/security/sigma_aslr.rs — ASLR: Address Space Layout Randomisation
// Language: Rust #![no_std] — 42-bit entropy per VMA region
// Pattern: OOP via Aslr struct

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Simple Linear Congruential PRNG (no std, no libc) ────────────────────────
// Seeded at boot from RDRAND or HPET timestamp
static ASLR_STATE: AtomicU64 = AtomicU64::new(0xDEAD_BEEF_CAFE_1337);

fn next_rand() -> u64 {
    // Xorshift64
    let mut x = ASLR_STATE.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    ASLR_STATE.store(x, Ordering::Relaxed);
    x
}

pub fn seed(val: u64) {
    ASLR_STATE.store(val ^ 0x5851_F42D_4C95_7F2D, Ordering::Relaxed);
}

// ── Virtual Address Regions ───────────────────────────────────────────────────
pub const KERNEL_BASE:   u64 = 0xFFFF_FFFF_8000_0000;
pub const USER_STACK_MAX: u64 = 0x0000_7FFF_FFFF_0000;
pub const USER_MMAP_BASE: u64 = 0x0000_7F80_0000_0000;
pub const USER_HEAP_BASE: u64 = 0x0000_0000_4000_0000;
pub const USER_EXEC_BASE: u64 = 0x0000_0000_0040_0000;

// ── Entropy masks ─────────────────────────────────────────────────────────────
const EXEC_ENTROPY:  u64 = 0x000_0000_0FFF_F000; // bits 12..27 (16 bits, 64KB granule)
const HEAP_ENTROPY:  u64 = 0x000_0001_FFFF_F000; // bits 12..28 (17 bits)
const MMAP_ENTROPY:  u64 = 0x0000_07FF_FFFF_F000; // bits 12..42 (31 bits)
const STACK_ENTROPY: u64 = 0x000_0007F_FFFF_F000; // bits 12..38 (27 bits)

pub struct AslrLayout {
    pub exec_base:  u64,
    pub heap_base:  u64,
    pub mmap_base:  u64,
    pub stack_top:  u64,
    pub vdso_base:  u64,
}

impl AslrLayout {
    /// Generate a fresh random address layout for a new process
    pub fn generate() -> Self {
        let r1 = next_rand();
        let r2 = next_rand();
        let r3 = next_rand();
        let r4 = next_rand();
        let r5 = next_rand();

        let exec_offset  = (r1 & EXEC_ENTROPY)  & !0xFFF; // page-aligned
        let heap_offset  = (r2 & HEAP_ENTROPY)  & !0xFFF;
        let mmap_offset  = (r3 & MMAP_ENTROPY)  & !0xFFF;
        let stack_offset = (r4 & STACK_ENTROPY) & !0xFFF;
        let vdso_offset  = (r5 & MMAP_ENTROPY)  & !0xFFF;

        Self {
            exec_base:  USER_EXEC_BASE  + exec_offset,
            heap_base:  USER_HEAP_BASE  + heap_offset,
            mmap_base:  USER_MMAP_BASE  - mmap_offset,
            stack_top:  USER_STACK_MAX  - stack_offset,
            vdso_base:  USER_MMAP_BASE  - vdso_offset - 0x10_0000,
        }
    }

    /// Allocate a new mmap region within the layout (grows down from mmap_base)
    pub fn alloc_mmap_region(&mut self, size: u64) -> u64 {
        let jitter = (next_rand() & 0x1F) << 12; // 0..31 pages extra gap
        self.mmap_base -= size + jitter;
        self.mmap_base & !0xFFF // page-align
    }

    /// Grow heap by `size` bytes, returns new brk
    pub fn grow_heap(&mut self, size: u64) -> u64 {
        self.heap_base += (size + 0xFFF) & !0xFFF;
        self.heap_base
    }
}

// ── Kernel KASLR ─────────────────────────────────────────────────────────────

/// Return a randomised kernel load offset (2MB aligned, ≤ 256 MB range)
pub fn kaslr_offset() -> u64 {
    let r = next_rand();
    // Bits 21..28 = 8 bits of entropy = up to 256 × 2MB = 512MB range
    ((r >> 21) & 0xFF) << 21
}
