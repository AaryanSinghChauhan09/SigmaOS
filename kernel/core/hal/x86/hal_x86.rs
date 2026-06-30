/// SigmaOS: =========================================================================
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Module: Sigma::hal_x86 ─────────────────────

#[no_mangle]
pub unsafe extern "C" fn hal_init() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_halt() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_pause() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_fence() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_irq_en() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_irq_dis() {
}

#[no_mangle]
pub unsafe extern "C" fn pic_remap() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_irq_init() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_timer_init() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_port_out8() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_mmio_write32() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_free_pages() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_mmu_map() {
}

#[no_mangle]
pub unsafe extern "C" fn x86_mmu_flush() {
}

