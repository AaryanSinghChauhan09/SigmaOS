/*
 * =============================================================================
 * Σ SIGMAOS: AARCH64 EXCEPTION VECTORS
 * =============================================================================
 * Low-level ARM64 exception vector table.
 * Catches Synchronous (Page Faults, Illegal Instructions), IRQ, FIQ, and 
 * SError exceptions to prevent silent kernel panics during DMA/NPU offload.
 *
 * Standard: GNU Assembler
 * =============================================================================
 */

.global vectors
.global aarch64_exception_handler

/* 
 * Macro to define an exception vector entry (each must be 0x80 bytes aligned).
 * We save volatile registers, call the C handler, and restore.
 */
.macro vector_entry label
    .align 7
    b \label
.endm

.macro handle_exception type
    /* Save registers x0-x29, lr(x30) to the stack */
    sub sp, sp, #256
    stp x0, x1, [sp, #16 * 0]
    stp x2, x3, [sp, #16 * 1]
    stp x4, x5, [sp, #16 * 2]
    stp x6, x7, [sp, #16 * 3]
    stp x8, x9, [sp, #16 * 4]
    stp x10, x11, [sp, #16 * 5]
    stp x12, x13, [sp, #16 * 6]
    stp x14, x15, [sp, #16 * 7]
    stp x16, x17, [sp, #16 * 8]
    stp x18, x19, [sp, #16 * 9]
    stp x20, x21, [sp, #16 * 10]
    stp x22, x23, [sp, #16 * 11]
    stp x24, x25, [sp, #16 * 12]
    stp x26, x27, [sp, #16 * 13]
    stp x28, x29, [sp, #16 * 14]
    
    /* Pass the exception type as the first argument to the C handler */
    mov x0, \type
    
    /* Call the high-level C exception router */
    bl aarch64_exception_router

    /* Restore registers and return from exception */
    ldp x0, x1, [sp, #16 * 0]
    ldp x2, x3, [sp, #16 * 1]
    ldp x4, x5, [sp, #16 * 2]
    ldp x6, x7, [sp, #16 * 3]
    ldp x8, x9, [sp, #16 * 4]
    ldp x10, x11, [sp, #16 * 5]
    ldp x12, x13, [sp, #16 * 6]
    ldp x14, x15, [sp, #16 * 7]
    ldp x16, x17, [sp, #16 * 8]
    ldp x18, x19, [sp, #16 * 9]
    ldp x20, x21, [sp, #16 * 10]
    ldp x22, x23, [sp, #16 * 11]
    ldp x24, x25, [sp, #16 * 12]
    ldp x26, x27, [sp, #16 * 13]
    ldp x28, x29, [sp, #16 * 14]
    add sp, sp, #256
    eret
.endm

/* =========================================================================
 * Vector Table (VBAR_EL1 points here)
 * ========================================================================= */
.align 11
vectors:
    /* Current EL with SP0 (Never used in SigmaOS typical EL1 execution) */
    vector_entry sync_curr_sp0
    vector_entry irq_curr_sp0
    vector_entry fiq_curr_sp0
    vector_entry serror_curr_sp0

    /* Current EL with SPx (Typical Kernel Mode Exceptions) */
    vector_entry sync_curr_spx
    vector_entry irq_curr_spx
    vector_entry fiq_curr_spx
    vector_entry serror_curr_spx

    /* Lower EL using AArch64 (Typical User Mode Exceptions) */
    vector_entry sync_lower_aarch64
    vector_entry irq_lower_aarch64
    vector_entry fiq_lower_aarch64
    vector_entry serror_lower_aarch64

    /* Lower EL using AArch32 (Not supported) */
    vector_entry sync_lower_aarch32
    vector_entry irq_lower_aarch32
    vector_entry fiq_lower_aarch32
    vector_entry serror_lower_aarch32

/* =========================================================================
 * Handlers
 * ========================================================================= */
sync_curr_sp0:          handle_exception 0
irq_curr_sp0:           handle_exception 1
fiq_curr_sp0:           handle_exception 2
serror_curr_sp0:        handle_exception 3

sync_curr_spx:          handle_exception 4  /* Kernel Sync (e.g. Page Fault) */
irq_curr_spx:           handle_exception 5  /* Kernel IRQ */
fiq_curr_spx:           handle_exception 6
serror_curr_spx:        handle_exception 7

sync_lower_aarch64:     handle_exception 8  /* User Sync (e.g. Syscall) */
irq_lower_aarch64:      handle_exception 9  /* User IRQ */
fiq_lower_aarch64:      handle_exception 10
serror_lower_aarch64:   handle_exception 11

sync_lower_aarch32:     handle_exception 12
irq_lower_aarch32:      handle_exception 13
fiq_lower_aarch32:      handle_exception 14
serror_lower_aarch32:   handle_exception 15
