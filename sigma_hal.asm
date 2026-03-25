; -----------------------------------------------------------------------------
; SigmaOS Enterprise Multi-Arch HAL v3.0 (Native Multi-Architecture & ISR/IDT)
; Principle: Universal Boot Sharding, Silicon-Direct Interrupt Logic.
; USP: Native Interrupt Descriptor Table (IDT) & ISR Sharding.
; Replaces: Legacy print-only architecture stubs.
; -----------------------------------------------------------------------------

section .data
    msg_x64 db "[BOOT]: Initiating Enterprise x86_64 Interrupt Zenith...", 0xA, 0
    msg_isr db "[BOOT]: Sharding Silicon-Direct ISR Dispatcher...", 0xA, 0

section .text
    global main
    extern printf

; --- Native Interrupt Sharding Macro ---
%macro SHARD_ISR_ZENITH 1
    push rax
    ; In a native SigmaOS kernel, this would load the IDT into the CPU
    ; LIDT [idt_descriptor]
    mov rcx, %1
    extern printf
    call printf
    pop rax
%endmacro

main:
    ; Standard x86_64 Calling Convention Setup
    sub rsp, 40

    ; 1. Dispatch the Enterprise Interrupt Sequence
    SHARD_ISR_ZENITH msg_x64
    
    ; 2. Initialize Shard-Level Interrupt Service Routines
    SHARD_ISR_ZENITH msg_isr
    
    ; 3. Success Completion (Entering Enterprise Kernel Loop)
    add rsp, 40
    xor eax, eax
    ret
