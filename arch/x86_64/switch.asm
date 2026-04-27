[BITS 64]
global switch_to_task

; switch_to_task(void** old_esp, void* new_esp)
switch_to_task:
    ; 1. Save current context
    push rbp
    push rbx
    push r12
    push r13
    push r14
    push r15

    ; 2. Swap stacks
    mov [rdi], rsp      ; Save current stack pointer to *old_esp
    mov rsp, rsi        ; Load new stack pointer from new_esp

    ; 3. Restore new context
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp

    ret
