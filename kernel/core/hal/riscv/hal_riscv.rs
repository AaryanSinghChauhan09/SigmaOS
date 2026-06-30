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

// ─── Module: Sigma::hal_riscv ─────────────────────

#[no_mangle]
pub unsafe extern "C" fn riscv_halt() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_pause() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_fence() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_irq_en() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_irq_dis() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_irq_init() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_timer_init() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_port_out8() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_mmio_write32() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_free_pages() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_mmu_map() {
}

#[no_mangle]
pub unsafe extern "C" fn riscv_mmu_flush() {
}

#[no_mangle]
pub unsafe extern "C" fn hal_init() {
}

