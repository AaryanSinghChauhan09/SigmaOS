; =============================================================================
; Σ SIGMAOS KERNEL: SILICON CONTEXT SWITCH (v1.0 - x86_64)
; =============================================================================
; void context_switch(SigmaTask* prev, SigmaTask* next)
; Arguments (System V AMD64 ABI):
;   RDI = prev
;   RSI = next
;
; SigmaTask Structure Offsets (x86_64):
;   offset 72: rsp
;   offset 88: cr3
; =============================================================================

[BITS 64]
section .text
global context_switch

context_switch:
    ; 1. Save callee-saved registers of current (prev) task
    push rbp
    push rbx
    push r12
    push r13
    push r14
    push r15
    
    ; 2. Save current stack pointer (RSP) into prev->rsp (offset 72)
    mov [rdi + 72], rsp
    
    ; 3. Load next stack pointer from next->rsp (offset 72)
    mov rsp, [rsi + 72]
    
    ; 4. Address space switch (CR3) if necessary
    mov rax, [rsi + 88] ; next->cr3 (offset 88)
    mov rcx, cr3
    cmp rax, rcx
    je .no_cr3_switch
    mov cr3, rax
.no_cr3_switch:

    ; 5. Restore callee-saved registers of next task
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    
    ; 6. Return into next task's RIP (which was pushed on its kstack)
    ret
