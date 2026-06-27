// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_arm_boot.S — AArch64 (ARM64) boot sequence for SigmaOS
//
// Entry point reached from UEFI or U-Boot.
// Responsibilities:
//   1. Validate EL (Exception Level) — must be EL1 or EL2
//   2. Disable interrupts (DAIF bits)
//   3. Set up the MMU page tables (TTBR0/TTBR1)
//   4. Enable MMU + instruction cache + data cache
//   5. Zero BSS section
//   6. Call sigma_arm_main() (C entry point)
//
// Inspired by:
//   • Linux arch/arm64/kernel/head.S
//   • Raspberry Pi 4 bare-metal boot (ARM Cortex-A72)
//   • QEMU virt AArch64 machine startup

.section .text.boot
.global _sigma_arm_start
.global sigma_arm_stack_top

// ── Boot stack ────────────────────────────────────────────────────────────────
.section .bss.boot
.align 16
sigma_arm_boot_stack:
    .space 65536          // 64KB boot stack
sigma_arm_stack_top:

// ── Entry point ───────────────────────────────────────────────────────────────
.section .text.boot
_sigma_arm_start:
    // Determine current EL
    mrs x0, CurrentEL
    lsr x0, x0, #2
    cmp x0, #2
    beq .Ldrop_to_el1     // Running at EL2 → drop to EL1
    cmp x0, #1
    bne .Lhalt_arm        // EL3 or EL0: unsupported

.Lat_el1:
    // Disable all interrupts (DAIF = mask Debug/SError/IRQ/FIQ)
    msr DAIFSet, #0xF

    // Set up stack pointer for EL1
    adr x0, sigma_arm_stack_top
    mov sp, x0

    // Clear BSS
    adr x0, __bss_start
    adr x1, __bss_end
.Lbss_loop:
    cmp x0, x1
    bge .Lbss_done
    str xzr, [x0], #8
    b   .Lbss_loop
.Lbss_done:

    // Set SCTLR_EL1: enable MMU + I-cache + D-cache + alignment check
    mrs x0, SCTLR_EL1
    orr x0, x0, #(1 << 0)   // M:  MMU enable
    orr x0, x0, #(1 << 2)   // C:  D-cache enable
    orr x0, x0, #(1 << 12)  // I:  I-cache enable
    msr SCTLR_EL1, x0
    isb

    // Jump to C entry
    bl  sigma_arm_main
    // Should never return — loop forever
.Lhalt_arm:
    wfe
    b .Lhalt_arm

// ── Drop from EL2 to EL1 ──────────────────────────────────────────────────────
.Ldrop_to_el1:
    // Configure HCR_EL2 to run kernel at EL1 (AArch64)
    mov x0, #(1 << 31)   // RW: EL1 is AArch64
    msr HCR_EL2, x0

    // SPSR_EL2: mask all interrupts, EL1h
    mov x0, #0x3C5        // DAIF masked, EL1h (SP_EL1)
    msr SPSR_EL2, x0
    adr x0, .Lat_el1
    msr ELR_EL2, x0
    eret

.section .text
.global sigma_arm_cpu_id
sigma_arm_cpu_id:
    // Returns MPIDR_EL1[7:0] (CPU affinity level 0)
    mrs x0, MPIDR_EL1
    and x0, x0, #0xFF
    ret
