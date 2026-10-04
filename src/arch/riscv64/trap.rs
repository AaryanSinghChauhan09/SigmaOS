//! SigmaOS — RISC-V 64-bit Trap Handling (M-mode + S-mode)
//! SPDX-License-Identifier: MIT OR GPL-2.0
//!
//! Implements CSR accessors for trap management, mcause/mepc/mtval decode,
//! and the trap dispatcher called from the raw asm trap entry.
//! Reference: RISC-V Privileged ISA Specification v1.12, Chapter 3 & 4.
#![allow(dead_code, unused)]

use core::arch::asm;

// ─── Trap causes (mcause / scause) ───────────────────────────────────────────
/// Interrupt bit: bit 63 of mcause/scause.
pub const CAUSE_INTERRUPT_BIT: u64 = 1 << 63;

/// Exception codes (non-interrupt, mcause[62:0]):
pub const EXC_INST_ADDR_MISALIGNED: u64 = 0;
pub const EXC_INST_ACCESS_FAULT: u64 = 1;
pub const EXC_ILLEGAL_INST: u64 = 2;
pub const EXC_BREAKPOINT: u64 = 3;
pub const EXC_LOAD_ADDR_MISALIGNED: u64 = 4;
pub const EXC_LOAD_FAULT: u64 = 5;
pub const EXC_STORE_ADDR_MISALIGNED: u64 = 6;
pub const EXC_STORE_FAULT: u64 = 7;
pub const EXC_ECALL_UMODE: u64 = 8; // User ecall
pub const EXC_ECALL_SMODE: u64 = 9; // Supervisor ecall
pub const EXC_ECALL_MMODE: u64 = 11; // Machine ecall
pub const EXC_INST_PAGE_FAULT: u64 = 12;
pub const EXC_LOAD_PAGE_FAULT: u64 = 13;
pub const EXC_STORE_PAGE_FAULT: u64 = 15;

/// Interrupt codes (mcause[62:0] when bit 63 = 1):
pub const INT_SUPERVISOR_SOFT: u64 = 1;
pub const INT_MACHINE_SOFT: u64 = 3;
pub const INT_SUPERVISOR_TIMER: u64 = 5;
pub const INT_MACHINE_TIMER: u64 = 7;
pub const INT_SUPERVISOR_EXT: u64 = 9;
pub const INT_MACHINE_EXT: u64 = 11;

// ─── mstatus field masks ───────────────────────────────────────────────────────
pub const MSTATUS_MIE: u64 = 1 << 3; // Machine Interrupt Enable
pub const MSTATUS_MPIE: u64 = 1 << 7; // Machine Previous IE
pub const MSTATUS_MPP_MASK: u64 = 0b11 << 11; // Previous privilege
pub const MSTATUS_MPP_USER: u64 = 0b00 << 11;
pub const MSTATUS_MPP_SUPER: u64 = 0b01 << 11;
pub const MSTATUS_MPP_MACHINE: u64 = 0b11 << 11;

// ─── CSR Accessors ────────────────────────────────────────────────────────────

/// Read mcause — machine exception cause.
#[inline]
pub fn read_mcause() -> u64 {
    let val: u64;
    unsafe {
        asm!("csrr {0}, mcause", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Read mepc — machine exception program counter.
#[inline]
pub fn read_mepc() -> u64 {
    let val: u64;
    unsafe {
        asm!("csrr {0}, mepc", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Read mtval — machine trap value (fault address or bad instruction).
#[inline]
pub fn read_mtval() -> u64 {
    let val: u64;
    unsafe {
        asm!("csrr {0}, mtval", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Read mstatus.
#[inline]
pub fn read_mstatus() -> u64 {
    let val: u64;
    unsafe {
        asm!("csrr {0}, mstatus", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Write mepc — set exception program counter (return address from trap).
///
/// # Safety
/// Must be called from M-mode; `val` should be a valid instruction address.
#[inline]
pub unsafe fn write_mepc(val: u64) {
    unsafe {
        asm!("csrw mepc, {0}", in(reg) val, options(nostack, nomem));
    }
}

/// Write mstatus.
///
/// # Safety
/// Changing interrupt enable bits affects hardware interrupt delivery.
#[inline]
pub unsafe fn write_mstatus(val: u64) {
    unsafe {
        asm!("csrw mstatus, {0}", in(reg) val, options(nostack, nomem));
    }
}

/// Write mtvec — Machine Trap Vector Base.
/// Direct mode: bits[1:0] = 00. Vectored mode: bits[1:0] = 01.
///
/// # Safety
/// `addr` must be 4-byte aligned; handler at `addr` must be a valid trap entry.
#[inline]
pub unsafe fn write_mtvec(addr: u64) {
    unsafe {
        // Force direct mode by clearing lower 2 bits.
        let direct = addr & !0x3;
        asm!("csrw mtvec, {0}", in(reg) direct, options(nostack, nomem));
    }
}

/// Write stvec — Supervisor Trap Vector Base.
///
/// # Safety
/// `addr` must be 4-byte aligned; must be called from S-mode or higher.
#[inline]
pub unsafe fn write_stvec(addr: u64) {
    unsafe {
        let direct = addr & !0x3;
        asm!("csrw stvec, {0}", in(reg) direct, options(nostack, nomem));
    }
}

/// Read scause — supervisor exception cause.
#[inline]
pub fn read_scause() -> u64 {
    let val: u64;
    unsafe {
        asm!("csrr {0}, scause", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Read sepc — supervisor exception program counter.
#[inline]
pub fn read_sepc() -> u64 {
    let val: u64;
    unsafe {
        asm!("csrr {0}, sepc", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Read stval — supervisor trap value.
#[inline]
pub fn read_stval() -> u64 {
    let val: u64;
    unsafe {
        asm!("csrr {0}, stval", out(reg) val, options(nostack, nomem));
    }
    val
}

// ─── Cause helpers ────────────────────────────────────────────────────────────

/// Returns true if the mcause value represents an interrupt (async).
#[inline(always)]
pub const fn is_interrupt(cause: u64) -> bool {
    cause & CAUSE_INTERRUPT_BIT != 0
}

/// Extract the exception code from mcause/scause (strips interrupt bit).
#[inline(always)]
pub const fn exception_code(cause: u64) -> u64 {
    cause & !CAUSE_INTERRUPT_BIT
}

// ─── High-level trap dispatcher ───────────────────────────────────────────────

/// Saved register file for all 32 RISC-V integer registers.
/// Populated by the raw assembly trap entry stub.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    /// x0 — always zero (stored for uniform layout)
    pub x: [u64; 32],
}

/// High-level trap handler. Called by the low-level assembly stub after
/// all registers are saved into `frame`.
///
/// # RISC-V syscall ABI
/// - Syscall number: a7 (x17)
/// - Arguments: a0–a5 (x10–x15)
/// - Return value: a0 (x10)
///
/// # Safety
/// Must only be called from the assembly trap entry with a fully-populated
/// `TrapFrame`. The frame must live for the duration of this function.
#[no_mangle]
pub unsafe extern "C" fn riscv_trap_handler(frame: *mut TrapFrame) {
    let cause = read_mcause();
    let epc = read_mepc();
    let tval = read_mtval();

    if is_interrupt(cause) {
        handle_interrupt(exception_code(cause));
    } else {
        handle_exception(exception_code(cause), epc, tval, frame);
    }
}

// ─── Interrupt / exception stubs ──────────────────────────────────────────────

#[inline]
fn handle_interrupt(code: u64) {
    match code {
        INT_MACHINE_TIMER => {
            // ACK the timer interrupt by setting a far-future compare value.
            // Real implementation calls into the timer module.
        }
        INT_MACHINE_EXT => {
            // Delegate to PLIC claim/complete cycle.
        }
        _ => {}
    }
}

#[inline]
fn handle_exception(code: u64, epc: u64, tval: u64, frame: *mut TrapFrame) {
    match code {
        EXC_ECALL_UMODE => {
            // User ecall — syscall. a7 = syscall#, a0–a5 = args.
            // Advance mepc by 4 to skip the ecall instruction.
            unsafe {
                write_mepc(epc.wrapping_add(4));
            }
            dispatch_syscall(frame);
        }
        EXC_LOAD_PAGE_FAULT | EXC_STORE_PAGE_FAULT | EXC_INST_PAGE_FAULT => {
            // Page fault — forward to VM subsystem.
            handle_page_fault(code, tval, epc);
        }
        EXC_ILLEGAL_INST => {
            // Illegal instruction — possibly emulate CSR/FPU access in user mode.
            let _ = tval;
        }
        _ => {
            // Unhandled — kernel panic in production.
            let _ = (code, epc, tval);
        }
    }
}

#[inline]
fn dispatch_syscall(frame: *mut TrapFrame) {
    if frame.is_null() {
        return;
    }
    // Safety: caller guarantees frame is valid and non-null.
    let _syscall_nr = unsafe { (*frame).x[17] }; // a7
    let _arg0 = unsafe { (*frame).x[10] }; // a0
                                           // Future: index into syscall table.
}

#[inline]
fn handle_page_fault(code: u64, fault_addr: u64, epc: u64) {
    let _ = (code, fault_addr, epc);
    // Future: call into VMM to allocate/map page.
}

// ─── init ─────────────────────────────────────────────────────────────────────

/// Install the machine trap vector.
///
/// # Safety
/// Must be called in M-mode. The assembly trap entry symbol must be linked.
pub fn init() {
    // In production, pass the address of the asm trap entry:
    // unsafe { write_mtvec(&_trap_entry as *const () as u64); }
    // Placeholder: configures mstatus for machine-mode interrupts.
    unsafe {
        let status = read_mstatus();
        // Enable machine-mode interrupts (MIE)
        write_mstatus(status | MSTATUS_MIE);
    }
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interrupt_set() {
        let cause = CAUSE_INTERRUPT_BIT | INT_MACHINE_TIMER;
        assert!(is_interrupt(cause));
    }

    #[test]
    fn test_is_not_interrupt() {
        let cause = EXC_ECALL_UMODE;
        assert!(!is_interrupt(cause));
    }

    #[test]
    fn test_exception_code_strips_interrupt_bit() {
        let cause = CAUSE_INTERRUPT_BIT | INT_MACHINE_EXT;
        assert_eq!(exception_code(cause), INT_MACHINE_EXT);
    }

    #[test]
    fn test_exception_code_no_interrupt_bit() {
        let cause = EXC_LOAD_PAGE_FAULT;
        assert_eq!(exception_code(cause), EXC_LOAD_PAGE_FAULT);
    }

    #[test]
    fn test_exc_ecall_umode_code() {
        assert_eq!(EXC_ECALL_UMODE, 8);
    }

    #[test]
    fn test_exc_store_page_fault_code() {
        assert_eq!(EXC_STORE_PAGE_FAULT, 15);
    }

    #[test]
    fn test_trap_frame_size() {
        assert_eq!(core::mem::size_of::<TrapFrame>(), 32 * 8);
    }

    #[test]
    fn test_mstatus_mie_bit() {
        assert_eq!(MSTATUS_MIE, 0b1000);
    }
}
