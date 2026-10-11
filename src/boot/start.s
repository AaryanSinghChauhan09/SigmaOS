/*
 * SigmaOS Assembly Bootstrap Entry Wrapper (start.s)
 * Universal Multiboot2 & Limine Handoff Entry Point for x86_64 Long Mode
 */

.code32
.section .text
.global start_kernel_entry
.global multiboot2_longmode_entry
.weak _start

_start:
start_kernel_entry:
multiboot2_longmode_entry:
    /* Jumps to main assembly bootstrap sequence in boot.s */
    jmp start
