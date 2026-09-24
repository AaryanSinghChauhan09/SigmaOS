; SigmaOS Bare-Metal Bootloader Entry
; Inspired by cfenollosa/os-tutorial and Multiboot standards
; 
; This file provides a Multiboot2 compliant header and the initial assembly 
; entry point to transition from the bootloader (e.g. GRUB) into the Rust kernel.

section .multiboot_header
align 8
header_start:
    ; Multiboot2 Magic Number
    dd 0xE85250D6
    ; Architecture: i386 (protected mode)
    dd 0
    ; Header Length
    dd header_end - header_start
    ; Checksum
    dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

    ; End tag
    align 8
    dw 0
    dw 0
    dd 8
header_end:

section .bss
align 16
stack_bottom:
    resb 16384 ; 16 KB initial stack
stack_top:

section .text
global _start
extern start_kernel ; Defined in src/kernel/main.rs

_start:
    ; The bootloader has loaded us into 32-bit protected mode.
    ; Interrupts are disabled. Paging is disabled.

    ; 1. Set up the stack
    mov esp, stack_top

    ; 2. Push Multiboot info pointer (EBX) and magic number (EAX) as arguments
    push ebx
    push eax

    ; 3. Disable interrupts (just to be absolutely certain)
    cli

    ; 4. Call the Rust kernel entry point
    call start_kernel

    ; 5. Halt the CPU if the kernel ever returns
.hang:
    cli
    hlt
    jmp .hang
