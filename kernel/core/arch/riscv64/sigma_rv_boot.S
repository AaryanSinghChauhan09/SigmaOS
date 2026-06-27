// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_rv_boot.S — RISC-V 64-bit (RV64GC) boot sequence for SigmaOS
//
// Silicon sovereignty angle: RISC-V is an open ISA — no licensing fees,
// no vendor lock-in.  This port targets QEMU virt + SiFive Unmatched.
//
// Entry from M-mode (SBI/OpenSBI hands off to S-mode).
// OpenSBI sets:
//   a0 = hartid (hardware thread ID, our CPU id)
//   a1 = pointer to device tree (DTB)
//
// Inspired by:
//   • Linux arch/riscv/kernel/head.S
//   • RISC-V privileged spec v20211203 (§ 3.1 Machine CSRs)
//   • SiFive Freedom SDK boot sequence

.section .text.boot
.global _sigma_rv_start
.global sigma_rv_stack_top

// ── Boot stack ────────────────────────────────────────────────────────────────
.section .bss.boot
.align 16
sigma_rv_boot_stack:
    .space 65536
sigma_rv_stack_top:

// ── Entry point ───────────────────────────────────────────────────────────────
.section .text.boot
_sigma_rv_start:
    // a0 = hartid, a1 = dtb pointer (passed by OpenSBI)

    // Set stack pointer
    la   sp, sigma_rv_stack_top

    // Disable all interrupts: clear SIE bit in sstatus
    csrci sstatus, 0x2   // SIE = bit 1

    // Zero BSS
    la   t0, __bss_start
    la   t1, __bss_end
.Lbss_loop:
    bgeu t0, t1, .Lbss_done
    sd   zero, 0(t0)
    addi t0, t0, 8
    j    .Lbss_loop
.Lbss_done:

    // Set up supervisor trap vector (sigma_rv_trap_vector)
    la   t0, sigma_rv_trap_vector
    csrw stvec, t0

    // Enable S-mode interrupts (after trap vector is ready)
    li   t0, 0x222       // SSIE | STIE | SEIE
    csrw sie, t0

    // Call C entry point (hartid in a0, dtb in a1)
    call sigma_rv_main

.Lhalt_rv:
    wfi
    j .Lhalt_rv

// ── Trap vector (aligned to 4 bytes, direct mode) ─────────────────────────────
.align 4
.global sigma_rv_trap_vector
sigma_rv_trap_vector:
    // Save all registers to per-hart trap frame
    // (full trap frame save not shown — see sigma_rv_trap.cpp)
    call sigma_rv_handle_trap
    sret

// ── Hart ID accessor ──────────────────────────────────────────────────────────
.global sigma_rv_hartid
sigma_rv_hartid:
    csrr a0, mhartid
    ret
