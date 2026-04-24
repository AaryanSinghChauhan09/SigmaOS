# =============================================================================
# Σ SIGMAOS: RISC-V 64 (RV64) BOOTSTRAP STUB
# =============================================================================
# Minimal entry point for RISC-V 64-bit targets.
# OpenSBI has set up the M-mode environment; we run in S-mode.
# a0 = hart ID, a1 = device tree pointer.
#
# This file is only assembled when SIGMA_ARCH_RISCV64 is defined.
# =============================================================================

.section .text
.global _start
.extern kernel_main

_start:
    # Only hart 0 proceeds; all other harts spin
    bnez    a0, .Lspin

    # Save device tree pointer
    mv      s1, a1

    # Set up kernel stack
    la      sp, _stack_top

    # Zero BSS
    la      t0, __bss_start
    la      t1, __bss_end
.Lzero_bss:
    bge     t0, t1, .Lbss_done
    sd      zero, 0(t0)
    addi    t0, t0, 8
    j       .Lzero_bss
.Lbss_done:

    # Call kernel_main(hart_id=0, dtb_ptr)
    li      a0, 0
    mv      a1, s1
    call    kernel_main

.Lspin:
    wfi
    j       .Lspin

# =============================================================================
# Stack reservation (256KB)
# =============================================================================
.section .bss
.align 4
_stack_bottom:
    .space  262144
_stack_top:
