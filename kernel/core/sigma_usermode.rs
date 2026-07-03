// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_usermode.rs — Ring 3 usermode transition
//
// Provides:
//   - sigma_tss_init(kernel_stack: u64)   — configure TSS RSP0 for ring-0 re-entry
//   - sigma_jump_to_usermode(entry, user_stack) — iretq into ring 3
//   - Per-process kernel stack tracking via PROCESS_STACKS table
//
// Language: Rust #![no_std]
// No external crates. All hardware access via inline asm.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Segment selectors (must match gdt.asm) ────────────────────────────────
pub const GDT_KERNEL_CODE: u16 = 0x08;
pub const GDT_KERNEL_DATA: u16 = 0x10;
pub const GDT_USER_CODE:   u16 = 0x18 | 3;   // ring 3
pub const GDT_USER_DATA:   u16 = 0x20 | 3;   // ring 3
pub const GDT_TSS:         u16 = 0x28;

// ── TSS layout (64-bit, matches gdt.asm .bss sigma_tss) ───────────────────
// Offset 4 = RSP0 (kernel stack pointer when entering ring 0)
// We update this offset when switching processes.

// ── Per-process kernel stack table ────────────────────────────────────────
pub const MAX_PROCS: usize = 256;
/// Kernel stack base address for each process slot (0 = unused)
static PROCESS_STACKS: [AtomicU64; MAX_PROCS] = {
    // const initializer: array of AtomicU64::new(0)
    const ZERO: AtomicU64 = AtomicU64::new(0);
    [ZERO; MAX_PROCS]
};

// ── Serial debug (minimal, mirrors sigma_irq.rs) ──────────────────────────
#[inline(always)]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val,
                     options(nomem, nostack, preserves_flags));
}
#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    core::arch::asm!("in al, dx", out("al") v, in("dx") port,
                     options(nomem, nostack, preserves_flags));
    v
}
unsafe fn serial_putc(c: u8) {
    while inb(0x3FD) & 0x20 == 0 {}
    outb(0x3F8, c);
}
unsafe fn serial_puts(s: &[u8]) {
    for &b in s {
        if b == b'\n' { serial_putc(b'\r'); }
        serial_putc(b);
    }
}

// ── hex helper ────────────────────────────────────────────────────────────
fn hex64(v: u64) -> [u8; 18] {
    let mut s = [b'0'; 18];
    s[0] = b'0'; s[1] = b'x';
    let d = b"0123456789ABCDEF";
    for i in 0..16 {
        s[17 - i] = d[((v >> (i * 4)) & 0xF) as usize];
    }
    s
}

// ── TSS RSP0 update ───────────────────────────────────────────────────────
// Update the TSS RSP0 field so that when ring-3 code triggers an exception
// or syscall, the CPU switches to the correct kernel stack for this process.
//
// The TSS is allocated in gdt.asm (.bss section, sigma_tss label).
// sigma_gdt_ptr holds the IDTPointer; sigma_tss+4 = RSP0.
extern "C" {
    /// The sigma_tss symbol exported from gdt.asm
    static mut sigma_tss: [u8; 128];
}

/// Update TSS RSP0 to `kernel_stack` for the currently running process.
///
/// # Safety
/// Must be called with interrupts disabled. `kernel_stack` must point to
/// the top of a valid, per-process kernel stack (typically STACK_TOP = base + size).
#[no_mangle]
pub unsafe extern "C" fn sigma_tss_init(kernel_stack: u64) {
    // TSS layout (64-bit): bytes 4..12 = RSP0
    let rsp0_ptr = (&raw mut sigma_tss[4]) as *mut u64;
    core::ptr::write_volatile(rsp0_ptr, kernel_stack);

    serial_puts(b"[TSS] RSP0 = ");
    let h = hex64(kernel_stack);
    serial_puts(&h);
    serial_puts(b"\n");
}

/// Record process `pid`'s kernel stack address in the per-process table,
/// then update the TSS RSP0 for the newly scheduled process.
///
/// # Safety
/// Must be called from scheduler context with interrupts disabled.
#[no_mangle]
pub unsafe extern "C" fn sigma_set_process_kernel_stack(pid: u32, stack_top: u64) {
    if (pid as usize) < MAX_PROCS {
        PROCESS_STACKS[pid as usize].store(stack_top, Ordering::Relaxed);
    }
    // Immediately apply to TSS so the next privilege transition uses it
    sigma_tss_init(stack_top);
}

/// On process switch: update TSS RSP0 to the incoming process's kernel stack.
#[no_mangle]
pub unsafe extern "C" fn sigma_switch_process_stack(next_pid: u32) {
    if (next_pid as usize) < MAX_PROCS {
        let stack = PROCESS_STACKS[next_pid as usize].load(Ordering::Relaxed);
        if stack != 0 {
            sigma_tss_init(stack);
        } else {
            serial_puts(b"[TSS] WARNING: process has no kernel stack registered\n");
        }
    }
}

// ── jump_to_usermode ──────────────────────────────────────────────────────
/// Transfer CPU execution to ring-3 user code via IRETQ.
///
/// Stack frame pushed (64-bit long-mode IRETQ):
///   +40  SS    (user data selector)
///   +32  RSP   (user stack pointer)
///   +24  RFLAGS (IF=1, reserved bits set)
///   +16  CS    (user code selector)
///    +8  RIP   (entry point)
///     0  (iretq pops these automatically)
///
/// # Safety
/// - `entry` must be a valid ring-3 code address mapped in user page tables
/// - `user_stack` must be 16-byte aligned user stack top
/// - Interrupts must be disabled before calling; RFLAGS.IF=1 is restored by iretq
/// - `sigma_tss_init` must have been called with this process's kernel stack first
/// - Caller must NOT return after this function — it is a one-way transition
#[no_mangle]
pub unsafe extern "C" fn sigma_jump_to_usermode(entry: u64, user_stack: u64) -> ! {
    serial_puts(b"[USERMODE] Entering ring 3, entry=");
    let h = hex64(entry);
    serial_puts(&h);
    serial_puts(b" stack=");
    let hs = hex64(user_stack);
    serial_puts(&hs);
    serial_puts(b"\n");

    core::arch::asm!(
        // Disable interrupts while we set up the stack frame
        "cli",
        // Set up user data segment registers
        "mov ax, {user_data_sel:x}",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        // Build IRETQ frame on stack:
        // push SS, push user_rsp, push RFLAGS (IF=1), push CS, push RIP
        "push {user_data}",     // SS
        "push {user_sp}",       // RSP
        "pushfq",               // RFLAGS (current flags)
        "pop  rax",
        "or   rax, 0x200",      // set IF (enable interrupts after iretq)
        "and  rax, ~0x100",     // clear TF (no single-step)
        "push rax",             // push modified RFLAGS
        "push {user_code}",     // CS
        "push {entry}",         // RIP
        // Restore kernel segment for SS before IRETQ switches it
        "mov ax, {kernel_data_sel:x}",
        "mov ss, ax",
        // Enter ring 3
        "iretq",
        user_data_sel   = const GDT_USER_DATA as u32,
        kernel_data_sel = const GDT_KERNEL_DATA as u32,
        user_data  = in(reg) GDT_USER_DATA  as u64,
        user_code  = in(reg) GDT_USER_CODE  as u64,
        user_sp    = in(reg) user_stack,
        entry      = in(reg) entry,
        options(noreturn)
    );
}

// ── General Protection Fault handler for IRETQ errors ──────────────────────
/// Called by IDT vector 13 (#GP) handler when iretq triggers a fault.
/// Logs diagnostic and halts — usermode entry failed.
#[no_mangle]
pub unsafe extern "C" fn sigma_usermode_gp_handler(error_code: u64, rip: u64) {
    serial_puts(b"[#GP] General Protection Fault during usermode transition\n");
    serial_puts(b"  error_code=");
    let he = hex64(error_code);
    serial_puts(&he);
    serial_puts(b"  rip=");
    let hr = hex64(rip);
    serial_puts(&hr);
    serial_puts(b"\n[HALT] Kernel halted.\n");
    loop {
        core::arch::asm!("cli; hlt", options(nomem, nostack, preserves_flags));
    }
}

// ── Query: get kernel stack for process (for testing) ─────────────────────
#[no_mangle]
pub extern "C" fn sigma_get_process_stack(pid: u32) -> u64 {
    if (pid as usize) < MAX_PROCS {
        PROCESS_STACKS[pid as usize].load(Ordering::Relaxed)
    } else {
        0
    }
}
