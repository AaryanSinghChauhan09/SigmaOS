; =========================================================================
; Cosmos AI-OS: Multiboot2 Bootloader Entry (x86_64)
; =========================================================================
; Mission: Transition from BIOS/UEFI into the Enterprise Environment.
; Bypasses standard host OSes completely. This is bare-metal.

section .multiboot_header
align 8
header_start:
    dd 0xe85250d6                ; Multiboot2 magic number
    dd 0                         ; Architecture 0 (protected mode i386)
    dd header_end - header_start ; Header length
    ; Checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; Framebuffer tag (Requesting native GPU mode from bootloader)
    align 8
    dw 5                         ; Type: Framebuffer
    dw 0                         ; Flags
    dd 20                        ; Size
    dd 1920                      ; Width
    dd 1080                      ; Height
    dd 32                        ; Depth (32-bit RGBA)

    align 8
    dw 0                         ; Type: End tag
    dw 0
    dd 8
header_end:

section .bss
align 16
stack_bottom:
    resb 16384                   ; 16 KB early kernel stack
stack_top:

section .text
global _start
extern cosmos_c_init             ; The C kernel entry function

_start:
    ; 1. Setup Early Stack
    mov esp, stack_top

    ; 2. Push Multiboot Magic and Pointer for C Code
    ; EAX contains Multiboot magic, EBX contains pointer to Multiboot info tree
    push eax
    push ebx

    ; 3. Disable CPU Interrupts (Until IDT is fully established)
    cli

    ; 4. Jump into the Deep-C Core
    ; We are now leaving Assembly and entering the C runtime.
    call cosmos_c_init

    ; 5. Failsafe Halt (if C code ever returns)
.hang:
    cli
    hlt
    jmp .hang
