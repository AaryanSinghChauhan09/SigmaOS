// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: SOVEREIGN MMIO PRIMITIVES
//! =========================================================================
//!
//! Language: Rust  #![no_std]  #![no_builtins]
//!
//! Freestanding Memory-Mapped I/O read/write helpers.
//! ZERO standard library. ZERO crates. ZERO predefined functions.
//!
//! All access is performed via raw pointer dereferences wrapped in
//! `core::ptr::read_volatile` / `write_volatile` — the ONLY core:: item
//! used here, because volatile semantics cannot be expressed via raw
//! pointer syntax alone without compiler reordering. Everything else
//! (types, bounds, results) is defined from scratch in this module.
//! =========================================================================

#![no_std]
#![no_builtins]

// ── Types (re-declared here to remain self-contained) ─────────────────────
pub type U8  = u8;
pub type U16 = u16;
pub type U32 = u32;
pub type U64 = u64;

// ═══════════════════════════════════════════════════════════════════════════
// § 1. Raw MMIO register read/write (8 / 16 / 32 / 64-bit)
//      Implements volatile semantics directly — no wrapper crate.
// ═══════════════════════════════════════════════════════════════════════════

/// Read an 8-bit MMIO register.
///
/// # Safety
/// `base + offset` must be a valid, mapped MMIO address.
#[inline(always)]
pub unsafe fn mmio_read8(base: U64, offset: U32) -> U8 {
    let addr = (base + offset as U64) as *const U8;
    // Volatile read prevents compiler from caching or reordering.
    core::ptr::read_volatile(addr)
}

/// Write an 8-bit MMIO register.
///
/// # Safety
/// `base + offset` must be a valid, mapped MMIO address.
#[inline(always)]
pub unsafe fn mmio_write8(base: U64, offset: U32, val: U8) {
    let addr = (base + offset as U64) as *mut U8;
    core::ptr::write_volatile(addr, val);
}

/// Read a 16-bit MMIO register.
///
/// # Safety
/// `base + offset` must be 2-byte aligned and a valid MMIO address.
#[inline(always)]
pub unsafe fn mmio_read16(base: U64, offset: U32) -> U16 {
    let addr = (base + offset as U64) as *const U16;
    core::ptr::read_volatile(addr)
}

/// Write a 16-bit MMIO register.
#[inline(always)]
pub unsafe fn mmio_write16(base: U64, offset: U32, val: U16) {
    let addr = (base + offset as U64) as *mut U16;
    core::ptr::write_volatile(addr, val);
}

/// Read a 32-bit MMIO register.
///
/// # Safety
/// `base + offset` must be 4-byte aligned and a valid MMIO address.
#[inline(always)]
pub unsafe fn mmio_read32(base: U64, offset: U32) -> U32 {
    let addr = (base + offset as U64) as *const U32;
    core::ptr::read_volatile(addr)
}

/// Write a 32-bit MMIO register.
#[inline(always)]
pub unsafe fn mmio_write32(base: U64, offset: U32, val: U32) {
    let addr = (base + offset as U64) as *mut U32;
    core::ptr::write_volatile(addr, val);
}

/// Read a 64-bit MMIO register.
///
/// # Safety
/// `base + offset` must be 8-byte aligned and a valid MMIO address.
#[inline(always)]
pub unsafe fn mmio_read64(base: U64, offset: U32) -> U64 {
    let addr = (base + offset as U64) as *const U64;
    core::ptr::read_volatile(addr)
}

/// Write a 64-bit MMIO register.
#[inline(always)]
pub unsafe fn mmio_write64(base: U64, offset: U32, val: U64) {
    let addr = (base + offset as U64) as *mut U64;
    core::ptr::write_volatile(addr, val);
}

// ═══════════════════════════════════════════════════════════════════════════
// § 2. Register set/clear bit helpers (32-bit — most common in HW drivers)
// ═══════════════════════════════════════════════════════════════════════════

/// Set specific bits in a 32-bit MMIO register (read-modify-write).
#[inline(always)]
pub unsafe fn mmio_set_bits32(base: U64, offset: U32, mask: U32) {
    let cur = mmio_read32(base, offset);
    mmio_write32(base, offset, cur | mask);
}

/// Clear specific bits in a 32-bit MMIO register (read-modify-write).
#[inline(always)]
pub unsafe fn mmio_clear_bits32(base: U64, offset: U32, mask: U32) {
    let cur = mmio_read32(base, offset);
    mmio_write32(base, offset, cur & !mask);
}

// ═══════════════════════════════════════════════════════════════════════════
// § 3. Spin-wait helpers — no OS sleep, no timer dependency
// ═══════════════════════════════════════════════════════════════════════════

/// Spin until a 32-bit MMIO field matches `expected_val` (masked by `mask`).
///
/// Returns `true` if the condition was met within `max_iters` iterations,
/// `false` on timeout.
///
/// # Safety
/// `base + offset` must be a valid MMIO address.
#[inline]
pub unsafe fn mmio_poll32(
    base: U64,
    offset: U32,
    mask: U32,
    expected_val: U32,
    max_iters: u32,
) -> bool {
    let mut i: u32 = 0;
    loop {
        let v = mmio_read32(base, offset);
        if (v & mask) == expected_val {
            return true;
        }
        i += 1;
        if i >= max_iters {
            return false;
        }
        // CPU pause — reduces power and prevents memory ordering issues.
        // Implemented as inline asm — no std::thread::sleep.
        core::arch::asm!("pause", options(nomem, nostack, preserves_flags));
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// § 4. Memory barrier — ensures MMIO ordering on x86-64
// ═══════════════════════════════════════════════════════════════════════════

/// Full memory fence — serialises all prior MMIO writes before continuing.
/// Implemented as `mfence` inline assembly — no core::sync::atomic.
#[inline(always)]
pub fn mmio_fence() {
    unsafe {
        core::arch::asm!("mfence", options(nomem, nostack, preserves_flags));
    }
}

/// Store fence — ensures all prior stores are visible before new stores.
#[inline(always)]
pub fn mmio_sfence() {
    unsafe {
        core::arch::asm!("sfence", options(nomem, nostack, preserves_flags));
    }
}
