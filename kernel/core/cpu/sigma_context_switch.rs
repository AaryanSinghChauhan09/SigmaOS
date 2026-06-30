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

// ─── Module: SigmaOS::sigma_context_switch ─────────────────────

/// TaskContext — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub rsp: SigmaU64,
    pub rip: SigmaU64,
    pub rflags: SigmaU64,
}

#[no_mangle]
pub unsafe extern "C" fn switch_context_x86_64() {
}

#[no_mangle]
pub unsafe extern "C" fn switch_context_arm64() {
}

#[no_mangle]
pub unsafe extern "C" fn switch_context_riscv() {
}

