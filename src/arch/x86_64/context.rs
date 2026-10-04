//! SigmaOS — x86_64 Task Context Switch
//! SPDX-License-Identifier: MIT OR GPL-2.0
//!
//! Provides `TaskContext` (register save area), `switch_context` (naked
//! context-switcher following the System V AMD64 ABI callee-save convention),
//! `init_context` (fresh task setup), and FPU state save/restore via
//! `fxsave`/`fxrstor`.

#![allow(dead_code)]

use core::arch::asm;

// ─── TaskContext ──────────────────────────────────────────────────────────────

/// CPU register state for one kernel task (or a user task while in the kernel).
///
/// Layout matches the order in which `switch_context` pushes/pops registers so
/// that the struct can be used both as a save area and as an initial register
/// snapshot for `init_context`.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TaskContext {
    // General-purpose registers — only callee-saved GPRs need to be preserved
    // across a cooperative context switch (System V AMD64 ABI §3.2.1):
    //   rbx, rbp, r12–r15
    // The full set below is stored so that the struct can also serve as the
    // initial state for a new task and as a debug dump.
    pub rax:    u64,
    pub rbx:    u64,
    pub rcx:    u64,
    pub rdx:    u64,
    pub rsi:    u64,
    pub rdi:    u64,
    pub rbp:    u64,
    pub rsp:    u64,   // saved stack pointer
    pub r8:     u64,
    pub r9:     u64,
    pub r10:    u64,
    pub r11:    u64,
    pub r12:    u64,
    pub r13:    u64,
    pub r14:    u64,
    pub r15:    u64,
    pub rip:    u64,   // instruction pointer to resume at
    pub rflags: u64,   // saved RFLAGS
    pub cs:     u64,   // code segment selector
    pub ss:     u64,   // stack segment selector
}

// ─── Context switch ───────────────────────────────────────────────────────────

/// Switch CPU execution from the `from` task to the `to` task.
///
/// **ABI**: `from` is in `rdi`, `to` is in `rsi` (System V AMD64 1st/2nd arg).
///
/// Only callee-saved registers are pushed/popped (rbx, rbp, r12–r15) plus
/// the return address (which `call` pushes implicitly).  The remaining
/// registers (rax, rcx, rdx, rsi, rdi, r8–r11) are caller-saved and the
/// scheduler is responsible for not relying on them across a switch.
///
/// # Safety
/// - `from` must point to a valid, writable `TaskContext`.
/// - `to` must point to a valid, readable `TaskContext` whose `rsp` points
///   to a valid kernel stack and whose `rip` is a valid code address.
/// - The caller must ensure no data races on the context structs.
#[naked]
pub unsafe extern "C" fn switch_context(
    from: *mut TaskContext,
    to: *const TaskContext,
) {
    // SAFETY: This is a naked function — the compiler emits NO prologue or
    // epilogue.  We manually save all callee-saved registers onto the
    // *current* stack, record the current RSP into `from->rsp`, load the
    // `to` context's RSP, then pop the callee-saved registers and ret.
    //
    // Register layout on entry:
    //   rdi = from: *mut TaskContext
    //   rsi = to:   *const TaskContext
    asm!(
        // ── Save callee-saved GPRs of the outgoing task ──────────────────
        "push rbp",
        "push rbx",
        "push r12",
        "push r13",
        "push r14",
        "push r15",

        // ── Save current RSP into from->rsp (offset 0x38 = field rsp) ───
        // Offset of `rsp` in TaskContext:
        //   rax=0, rbx=8, rcx=16, rdx=24, rsi=32, rdi=40, rbp=48, rsp=56
        //   => byte offset 56 = 0x38
        "mov [rdi + 0x38], rsp",

        // ── Load the incoming task's RSP ─────────────────────────────────
        "mov rsp, [rsi + 0x38]",

        // ── Restore callee-saved GPRs of the incoming task ───────────────
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",

        // ── Return into the incoming task's saved RIP ─────────────────────
        // (the `ret` address was pushed by whoever last called switch_context
        //  on the `to` task, or by init_context which set up a fake frame).
        "ret",
        options(noreturn)
    );
}

// ─── init_context ─────────────────────────────────────────────────────────────

/// Initialise `ctx` so that the first `switch_context` *into* this task will
/// begin executing `entry`.
///
/// The fake stack frame laid down here mirrors what `switch_context` pops:
///   [stack_top - 8*1]  r15 = 0
///   [stack_top - 8*2]  r14 = 0
///   [stack_top - 8*3]  r13 = 0
///   [stack_top - 8*4]  r12 = 0
///   [stack_top - 8*5]  rbx = 0
///   [stack_top - 8*6]  rbp = 0
///   [stack_top - 8*7]  rip (= entry)  ← the `ret` in switch_context pops this
///
/// # Safety
/// - `stack_top` must be 16-byte aligned (after the `ret` address is popped
///   the stack will be 16-byte aligned as required by the ABI).
/// - `stack_top` must point one byte past the end of a valid, writable stack
///   buffer of at least 64 bytes.
pub unsafe fn init_context(ctx: &mut TaskContext, entry: fn(), stack_top: u64) {
    // Clear the context.
    *ctx = TaskContext::default();

    // Build the fake switch frame on the new task's stack.
    // We write 7 × u64 below stack_top (56 bytes used).
    let stack = (stack_top - 56) as *mut u64;

    // Safety: caller guarantees stack_top points to a valid, writable buffer.
    // Pop order in switch_context: r15, r14, r13, r12, rbx, rbp, then ret.
    stack.add(0).write(0);            // r15
    stack.add(1).write(0);            // r14
    stack.add(2).write(0);            // r13
    stack.add(3).write(0);            // r12
    stack.add(4).write(0);            // rbx
    stack.add(5).write(0);            // rbp
    stack.add(6).write(entry as u64); // return address → task entry point

    // RSP points at the bottom of the fake frame.
    ctx.rsp = stack_top - 56;
    ctx.rip = entry as u64;
    // RFLAGS: interrupts enabled (bit 9), reserved bit 1 always set.
    ctx.rflags = 0x0202;
}

// ─── FPU / SSE state ──────────────────────────────────────────────────────────

/// Save the x87 FPU, MMX, and SSE state (512 bytes) using `fxsave`.
///
/// # Safety
/// - `buf` must be 16-byte aligned (required by the `fxsave64` instruction).
/// - This instruction saves the state of the *current* CPU's FPU.
#[inline]
pub unsafe fn save_fpu_state(buf: &mut [u8; 512]) {
    // SAFETY: `buf` is 512 bytes; caller guarantees 16-byte alignment.
    asm!(
        "fxsave64 [{}]",
        in(reg) buf.as_mut_ptr(),
        options(nostack, preserves_flags)
    );
}

/// Restore the x87 FPU, MMX, and SSE state (512 bytes) using `fxrstor`.
///
/// # Safety
/// - `buf` must be 16-byte aligned.
/// - `buf` must contain a valid state image previously written by `fxsave64`.
#[inline]
pub unsafe fn restore_fpu_state(buf: &[u8; 512]) {
    // SAFETY: `buf` is 512 bytes; caller guarantees valid fxsave image.
    asm!(
        "fxrstor64 [{}]",
        in(reg) buf.as_ptr(),
        options(nostack, preserves_flags)
    );
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn task_context_size_and_alignment() {
        // 20 fields × 8 bytes = 160 bytes
        assert_eq!(core::mem::size_of::<TaskContext>(), 160);
        // repr(C) struct must be at least u64-aligned
        assert!(core::mem::align_of::<TaskContext>() >= 8);
    }

    #[test]
    fn task_context_default_zeroed() {
        let ctx = TaskContext::default();
        assert_eq!(ctx.rax, 0);
        assert_eq!(ctx.rsp, 0);
        assert_eq!(ctx.rip, 0);
        assert_eq!(ctx.rflags, 0);
    }

    #[test]
    fn init_context_sets_rip_and_rflags() {
        fn dummy_task() {}
        let mut stack = [0u8; 256];
        let stack_top = stack.as_mut_ptr() as u64 + 256;
        let mut ctx = TaskContext::default();
        unsafe { init_context(&mut ctx, dummy_task, stack_top) };
        assert_eq!(ctx.rip, dummy_task as u64);
        assert_eq!(ctx.rflags, 0x0202);
        // RSP must be below stack_top
        assert!(ctx.rsp < stack_top);
        assert!(ctx.rsp >= stack.as_ptr() as u64);
    }

    #[test]
    fn init_context_places_entry_on_stack() {
        fn my_task() {}
        let mut stack = [0u8; 256];
        let stack_top = stack.as_mut_ptr() as u64 + 256;
        let mut ctx = TaskContext::default();
        unsafe { init_context(&mut ctx, my_task, stack_top) };
        // The entry address should be the last u64 in the fake frame
        let ret_slot = ctx.rsp as *const u64;
        // Advance past: r15, r14, r13, r12, rbx, rbp (6 × 8 = 48 bytes)
        let entry_slot = unsafe { ret_slot.add(6) };
        assert_eq!(unsafe { *entry_slot }, my_task as u64);
    }
}
