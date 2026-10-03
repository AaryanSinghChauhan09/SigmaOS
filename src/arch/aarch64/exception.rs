//! SigmaOS — AArch64 Exception Vector Table
//! SPDX-License-Identifier: MIT OR GPL-2.0
//!
//! Implements ARM64 exception vectors (EL1), VBAR_EL1 setup,
//! ESR_EL1/FAR_EL1/ELR_EL1 decode, and SVC #0 syscall dispatch.
//! Reference: ARM Architecture Reference Manual, ARMv8-A, Section D1.
#![allow(dead_code, unused)]

use core::arch::asm;

// ─── Exception Syndrome Register EC codes ────────────────────────────────────
/// EC field occupies bits[31:26] of ESR_EL1.
pub const ESR_EC_SHIFT: u64 = 26;
pub const ESR_EC_MASK: u64 = 0x3F;

pub const EC_UNKNOWN: u64 = 0x00;
pub const EC_WFI_WFE: u64 = 0x01;
pub const EC_SVC_AA64: u64 = 0x15; // SVC in AArch64 state
pub const EC_INST_ABORT_LOWER: u64 = 0x20;
pub const EC_INST_ABORT_SAME: u64 = 0x21;
pub const EC_DATA_ABORT_LOWER: u64 = 0x24;
pub const EC_DATA_ABORT_SAME: u64 = 0x25;

// ─── CSR / System register accessors ─────────────────────────────────────────

/// Write VBAR_EL1 — Vector Base Address Register, EL1.
///
/// # Safety
/// `addr` must point to a 2KiB-aligned (2048-byte) exception vector table.
#[inline]
pub unsafe fn set_vbar_el1(addr: u64) {
    // Safety: privileged instruction; caller ensures correct alignment.
    unsafe {
        asm!(
            "msr vbar_el1, {0}",
            "isb",
            in(reg) addr,
            options(nostack, nomem),
        );
    }
}

/// Read ESR_EL1 — Exception Syndrome Register, EL1.
#[inline]
pub fn read_esr_el1() -> u64 {
    let val: u64;
    // Safety: read-only system register; safe at EL1.
    unsafe {
        asm!("mrs {0}, esr_el1", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Read FAR_EL1 — Fault Address Register (faulting virtual address).
#[inline]
pub fn read_far_el1() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {0}, far_el1", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Read ELR_EL1 — Exception Link Register (address of faulting instruction).
#[inline]
pub fn read_elr_el1() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {0}, elr_el1", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Write ELR_EL1 — set return address for `eret`.
///
/// # Safety
/// Caller must be in EL1 and must call `eret` after this.
#[inline]
pub unsafe fn write_elr_el1(val: u64) {
    unsafe {
        asm!("msr elr_el1, {0}", in(reg) val, options(nostack, nomem));
    }
}

/// Read SPSR_EL1 — Saved Program Status Register.
#[inline]
pub fn read_spsr_el1() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {0}, spsr_el1", out(reg) val, options(nostack, nomem));
    }
    val
}

// ─── Exception class decoder ──────────────────────────────────────────────────

/// Extract the EC (Exception Class) field from an ESR_EL1 value.
#[inline(always)]
pub const fn esr_ec(esr: u64) -> u64 {
    (esr >> ESR_EC_SHIFT) & ESR_EC_MASK
}

/// Extract ISS (Instruction-Specific Syndrome) field — bits[24:0].
#[inline(always)]
pub const fn esr_iss(esr: u64) -> u64 {
    esr & 0x01FF_FFFF
}

// ─── High-level exception dispatcher ─────────────────────────────────────────

/// Called from the exception vector stub (naked fn or asm entry).
/// Decodes ESR_EL1 and dispatches to the correct handler.
///
/// # AArch64 syscall ABI (Linux-compatible)
/// - Syscall number: x8
/// - Arguments: x0–x5
/// - Return value: x0
///
/// # Safety
/// Must only be called from a valid EL1 exception entry context.
#[no_mangle]
pub unsafe extern "C" fn el1_exception_handler() {
    let esr = read_esr_el1();
    let far = read_far_el1();
    let elr = read_elr_el1();
    let ec = esr_ec(esr);
    let iss = esr_iss(esr);

    match ec {
        EC_SVC_AA64 => {
            // SVC #0 — syscall. ISS holds the SVC immediate (0 for Linux ABI).
            // Syscall number is in x8; handled by syscall layer above us.
            handle_syscall(iss);
        }
        EC_DATA_ABORT_LOWER | EC_DATA_ABORT_SAME => {
            handle_data_abort(esr, far, elr);
        }
        EC_INST_ABORT_LOWER | EC_INST_ABORT_SAME => {
            handle_inst_abort(esr, far, elr);
        }
        EC_UNKNOWN | _ => {
            handle_unknown(ec, esr, elr);
        }
    }
}

// ─── Stub handlers (to be wired to higher-level subsystems) ─────────────────

#[inline]
fn handle_syscall(iss: u64) {
    // Syscall dispatch: actual number read from x8 by caller convention.
    // Stub: future implementation calls into syscall table.
    let _ = iss;
}

#[inline]
fn handle_data_abort(esr: u64, far: u64, elr: u64) {
    // Data abort — page fault or alignment fault.
    // DFSC = ESR[5:0]; WnR = ESR[6].
    let _dfsc = esr & 0x3F;
    let _wnr = (esr >> 6) & 1; // 1 = write, 0 = read
    let _ = (far, elr);
}

#[inline]
fn handle_inst_abort(esr: u64, far: u64, elr: u64) {
    let _ifsc = esr & 0x3F;
    let _ = (far, elr);
}

#[inline]
fn handle_unknown(ec: u64, esr: u64, elr: u64) {
    let _ = (ec, esr, elr);
}

// ─── init ─────────────────────────────────────────────────────────────────────

/// Install the exception vector table.
///
/// The actual vector table must be defined in assembly (2KiB-aligned)
/// and its symbol passed here. For now this is a skeleton that stores
/// the address of `el1_exception_handler` as a placeholder.
///
/// # Safety
/// Must be called exactly once during early boot, at EL1.
pub fn init() {
    // In production: pass the address of the assembly-defined vector table.
    // Placeholder: install a safe no-op address until asm table is linked.
    // Safety: see function contract above.
    unsafe {
        // Real vector table address would come from a linker symbol.
        // Example: set_vbar_el1(&__vectors as *const _ as u64);
    }
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esr_ec_extraction() {
        // EC = 0x15 (SVC AArch64) packed into bits[31:26]
        let esr: u64 = (EC_SVC_AA64 << ESR_EC_SHIFT) | 0x00;
        assert_eq!(esr_ec(esr), EC_SVC_AA64);
    }

    #[test]
    fn test_esr_iss_extraction() {
        let esr: u64 = (EC_DATA_ABORT_LOWER << ESR_EC_SHIFT) | 0x0000_0007;
        assert_eq!(esr_iss(esr), 0x7);
    }

    #[test]
    fn test_esr_ec_unknown() {
        let esr: u64 = 0x0000_0000;
        assert_eq!(esr_ec(esr), EC_UNKNOWN);
    }

    #[test]
    fn test_esr_data_abort_wnr() {
        // WnR bit set — write fault
        let esr: u64 = (EC_DATA_ABORT_SAME << ESR_EC_SHIFT) | (1 << 6) | 0x5;
        let wnr = (esr >> 6) & 1;
        assert_eq!(wnr, 1);
    }

    #[test]
    fn test_ec_constants_no_overlap() {
        let codes = [
            EC_UNKNOWN,
            EC_WFI_WFE,
            EC_SVC_AA64,
            EC_INST_ABORT_LOWER,
            EC_INST_ABORT_SAME,
            EC_DATA_ABORT_LOWER,
            EC_DATA_ABORT_SAME,
        ];
        // Each code must be unique
        for i in 0..codes.len() {
            for j in (i + 1)..codes.len() {
                assert_ne!(codes[i], codes[j]);
            }
        }
    }
}
