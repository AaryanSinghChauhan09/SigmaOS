; Σ SIGMAOS ZENITH: SOVEREIGN BOOT SHARD (v3000.0)
; Mission: Multiboot2 Handoff & 64-bit Long Mode Transition.

section .multiboot
align 8
multiboot_header_start:
    dd 0xe85250d6                ; Magic number (multiboot 2)
    dd 0                         ; Architecture 0 (protected mode i386)
    dd multiboot_header_end - multiboot_header_start ; Header length
    ; Checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (multiboot_header_end - multiboot_header_start))
    
    ; Tags (End Tag)
    dw 0
    dw 0
    dd 8
multiboot_header_end:

section .text
bits 32
global sigma_kernel_entry
extern kmain

sigma_kernel_entry:
    ; 1. Setup Stack
    mov esp, sigma_stack_top

    ; 2. (Simulated) Long Mode Transition (Jump to C)
    ; In a full implementation, we set up CR3 (Paging), CR4, and EFER here.
    call kmain

    ; 3. Mission Halt
    cli
.halt:
    hlt
    jmp .halt

section .bss
align 4096
stack_bottom:
    dd 0xDEADC0DE ; Σ SOVEREIGN STACK CANARY (B6)
    resb 16380    ; 16KB Stack (minus 4 bytes for canary)
sigma_stack_top:
