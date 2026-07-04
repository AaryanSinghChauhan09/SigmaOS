; SPDX-License-Identifier: MIT
; Copyright (c) 2024-2026 SigmaOS Project
;
; arch/x86_64/context_switch.asm — CPU context switch
;
; sigma_context_switch(from: *TaskContext, to: *TaskContext)
;   RDI = *from  (save current CPU state here)
;   RSI = *to    (restore CPU state from here)
;
; TaskContext layout (matches kernel/core/process_manager.rs):
;   offset  0: rsp
;   offset  8: r15
;   offset 16: r14
;   offset 24: r13
;   offset 32: r12
;   offset 40: rbp
;   offset 48: rbx
;   offset 56: rip   (return address / instruction pointer)
;   offset 64: cr3   (page table base)
;   offset 72: rflags
;   total: 80 bytes

global sigma_context_switch
global sigma_context_restore_first   ; used for very first task launch

section .text
bits 64

; ── sigma_context_switch(from*, to*) ────────────────────────────────────────
sigma_context_switch:
    ; Save callee-saved registers to *from
    ; The return address is already on the stack
    pop  rax                ; pop return address into rax
    mov  [rdi +  0], rsp
    mov  [rdi +  8], r15
    mov  [rdi + 16], r14
    mov  [rdi + 24], r13
    mov  [rdi + 32], r12
    mov  [rdi + 40], rbp
    mov  [rdi + 48], rbx
    mov  [rdi + 56], rax    ; save return address as rip
    mov  rax, cr3
    mov  [rdi + 64], rax    ; save page table
    pushfq
    pop  rax
    mov  [rdi + 72], rax    ; save rflags

    ; Restore from *to
    mov  rsp, [rsi +  0]
    mov  r15, [rsi +  8]
    mov  r14, [rsi + 16]
    mov  r13, [rsi + 24]
    mov  r12, [rsi + 32]
    mov  rbp, [rsi + 40]
    mov  rbx, [rsi + 48]
    ; Restore page table (if different)
    mov  rcx, [rsi + 64]
    mov  rax, cr3
    cmp  rax, rcx
    je   .skip_cr3
    mov  cr3, rcx
.skip_cr3:
    ; Restore rflags
    mov  rax, [rsi + 72]
    push rax
    popfq
    ; Push return address and ret
    push qword [rsi + 56]
    ret

; ── sigma_context_restore_first(to*) — launch first task ──────────────────
; Used when there is no "from" task to save into.
sigma_context_restore_first:
    mov  rsi, rdi           ; to* is in rdi
    ; Restore page table
    mov  rcx, [rsi + 64]
    mov  cr3, rcx
    ; Restore stack and registers
    mov  rsp, [rsi +  0]
    mov  r15, [rsi +  8]
    mov  r14, [rsi + 16]
    mov  r13, [rsi + 24]
    mov  r12, [rsi + 32]
    mov  rbp, [rsi + 40]
    mov  rbx, [rsi + 48]
    mov  rax, [rsi + 72]
    push rax
    popfq
    push qword [rsi + 56]
    ret
