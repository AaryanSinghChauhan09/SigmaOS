; Sovereign_Interrupt_Entry.asm
; Atomic Shard: Low-Level Interrupt Entry.
; Version: Phase 61 (High-Workability HAL)

[BITS 64]
global sigma_isr_stub

section .text

sigma_isr_stub:
    push rax
    push rbx
    push rcx
    push rdx
    push rdi
    push rsi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    mov rdi, rsp    ; Pass stack frame as context
    extern sigma_interrupt_dispatch
    call sigma_interrupt_dispatch

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rsi
    pop rdi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    iretq
