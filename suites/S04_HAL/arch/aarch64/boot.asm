; =============================================================================
; Σ SIGMAOS: ARM64 (AArch64) BOOTSTRAP STUB
; =============================================================================
; Minimal entry point for ARM64 targets. Sets up the stack and jumps to
; kernel_main(). The UEFI bootloader has already set up the MMU and provided
; the framebuffer address in x0.
;
; This file is only assembled when SIGMA_ARCH_AARCH64 is defined.
; =============================================================================

.section .text
.global _start
.extern kernel_main

_start:
    ; Save bootloader-provided framebuffer pointer (x0)
    mov     x19, x0

    ; Set up kernel stack (256KB)
    ldr     x1, =_stack_top
    mov     sp, x1

    ; Zero BSS section
    ldr     x0, =__bss_start
    ldr     x1, =__bss_end
.Lzero_bss:
    cmp     x0, x1
    b.ge    .Lbss_done
    str     xzr, [x0], #8
    b       .Lzero_bss
.Lbss_done:

    ; Pass framebuffer base to kernel_main as first argument
    mov     x0, x19
    bl      kernel_main

    ; If kernel_main returns, halt the CPU
.Lhalt:
    wfi
    b       .Lhalt

; =============================================================================
; Stack reservation (256KB)
; =============================================================================
.section .bss
.align 16
_stack_bottom:
    .space  262144
_stack_top:
