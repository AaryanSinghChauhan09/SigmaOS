; Σ SIGMAOS ZENITH: BARE-METAL KERNEL ENTRY (v1500.0)
; Multiboot2 / x86_64 Long Mode Transition

[BITS 32]
section .text

; --- MULTIBOOT2 HEADER (1M alignment) ---
align 8
section .multiboot_header
header_start:
    dd 0xe85250d6                ; Magic number
    dd 0                         ; Architecture: i386 (protected mode)
    dd header_end - header_start ; Header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; Checksum
    
    ; End tag
    dw 0
    dw 0
    dd 8
header_end:

; --- KERNEL ENTRY ---
global start
extern kmain

start:
    ; 1. Stack setup
    mov esp, stack_top

    ; 2. Enable Paging & Long Mode (Simplified)
    ; (Placeholder logic: IDT/GDT setup for 64-bit)

    ; 3. Transition to kmain
    call kmain

    ; 4. Halt on core return
    hlt

section .bss
align 16
stack_bottom:
    resb 16384 ; 16KB Kernel Stack
stack_top:
