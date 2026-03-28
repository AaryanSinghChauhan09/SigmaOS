/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

; =========================================================================
; Cosmos AI-OS: Enterprise Context Switcher (x86_64)
; =========================================================================
; Mission: Execute lightning-fast task switching. This gives Cosmos 
;          extreme responsiveness by bypassing OS abstractions and managing
;          the CPU state directly at Ring-0.

global cosmos_switch_tasks
extern current_task_stack_ptr

section .text

; void cosmos_switch_tasks(uint64_t* next_stack_ptr)
; RDI = Pointer to the stack of the next task

cosmos_switch_tasks:
    ; 1. Save current task's execution state onto its own stack
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    ; Save Flags
    pushf

    ; 2. Store the current stack pointer into the current process control block
    mov rax, [current_task_stack_ptr]
    mov [rax], rsp

    ; 3. Perform the actual context switch: Change the Stack Pointer!
    mov rsp, rdi

    ; 4. Restore the incoming task's execution state
    popf

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    
    ; 5. Jump into the new task
    ret

