; =========================================================================
; SIGMA OS: BOOTLOADER ENTRY PROTOCOL (Step 3)
; Multiboot 1 Compliant Header. Bypasses GRUB into Bare-Metal Kernel.
; =========================================================================

MAGIC equ 0x1BADB002
FLAGS equ 0x03
CHECKSUM equ -(MAGIC + FLAGS)

section .multiboot
align 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM

section .bss
align 16
stack_bottom:
    resb 16384          ; Secure 16 KB isolated kernel stack
stack_top:

section .text
global _start
extern kmain            ; Declare external C entry protocol

; -------------------------------------------------------------------------
; _start : The absolute first instruction executed by the CPU.
; -------------------------------------------------------------------------
_start:
    ; 1. Establish hardware stack to allow C code execution safely
    mov esp, stack_top
    
    ; 2. (Optional FPU/SSE init can happen here)
    
    ; 3. Transfer control into high-level C Sovereign Execution Matrix
    push ebx            ; Pass Multiboot info structure
    push eax            ; Pass Multiboot magic number
    call kmain

    ; 4. Total Halt Loop (If kmain somehow returns)
    cli
.hang:
    hlt
    jmp .hang
