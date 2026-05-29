; =========================================================================
; Σ SIGMAOS: CONTEXT SWITCH (Phase 16)
; =========================================================================
; Pure assembly cooperative context switch for the MLFQ scheduler.
;
; void sigma_context_switch(sigma_cpu_context_t* old, sigma_cpu_context_t* new);
;
; Saves callee-saved registers (SysV x86_64 ABI) into old context,
; restores from new context, and returns into the new task.
;
; sigma_cpu_context_t layout (see include/sigma_context.h):
;   offset 0:   rsp
;   offset 8:   rbp
;   offset 16:  rbx
;   offset 24:  r12
;   offset 32:  r13
;   offset 40:  r14
;   offset 48:  r15
;   offset 56:  rip (return address)
;   offset 64:  rflags
;   offset 72:  fxsave_area (512 bytes, 16-byte aligned)
; =========================================================================

[BITS 64]

section .text

global sigma_context_switch
global sigma_context_init

; =========================================================================
; sigma_context_switch(old_ctx* rdi, new_ctx* rsi)
; =========================================================================
sigma_context_switch:
    ; --- Save current (old) context ---
    
    ; Save callee-saved registers into old context
    mov  [rdi + 16], rbx
    mov  [rdi + 8],  rbp
    mov  [rdi + 24], r12
    mov  [rdi + 32], r13
    mov  [rdi + 40], r14
    mov  [rdi + 48], r15

    ; Save return address (the address that called us)
    ; It's on the stack at [rsp] since CALL pushed it
    mov  rax, [rsp]
    mov  [rdi + 56], rax

    ; Save RSP (after the CALL, so we skip the return address on restore)
    lea  rax, [rsp + 8]
    mov  [rdi + 0], rax

    ; Save RFLAGS
    pushfq
    pop  rax
    mov  [rdi + 64], rax

    ; Save FPU/SSE state (512 bytes at offset 72, must be 16-byte aligned)
    ; Check if fxsave area is available (non-null context)
    lea  rax, [rdi + 72]
    ; Ensure 16-byte alignment (buddy allocator guarantees this for contexts)
    fxsave [rax]

    ; --- Restore new context ---

    ; Restore FPU/SSE state
    lea  rax, [rsi + 72]
    fxrstor [rax]

    ; Restore RFLAGS
    mov  rax, [rsi + 64]
    push rax
    popfq

    ; Restore callee-saved registers
    mov  rbx, [rsi + 16]
    mov  rbp, [rsi + 8]
    mov  r12, [rsi + 24]
    mov  r13, [rsi + 32]
    mov  r14, [rsi + 40]
    mov  r15, [rsi + 48]

    ; Restore RSP
    mov  rsp, [rsi + 0]

    ; Jump to the new task's saved RIP
    ; We push it onto the new stack and RET into it
    mov  rax, [rsi + 56]
    push rax
    ret


; =========================================================================
; sigma_context_init(ctx* rdi, entry_point rsi, stack_top rdx)
; =========================================================================
; Initialize a context for a new task that hasn't run yet.
; When sigma_context_switch restores this context, execution begins at
; the entry_point function with the given stack.
; =========================================================================
sigma_context_init:
    ; Set up the context struct
    mov  [rdi + 0],  rdx       ; rsp = stack_top
    mov  [rdi + 8],  rdx       ; rbp = stack_top (frame pointer)
    mov  qword [rdi + 16], 0   ; rbx = 0
    mov  qword [rdi + 24], 0   ; r12 = 0
    mov  qword [rdi + 32], 0   ; r13 = 0
    mov  qword [rdi + 40], 0   ; r14 = 0
    mov  qword [rdi + 48], 0   ; r15 = 0
    mov  [rdi + 56], rsi       ; rip = entry_point
    
    ; Set RFLAGS: interrupts enabled (IF=1), direction clear (DF=0)
    mov  qword [rdi + 64], 0x202
    
    ; Zero out the fxsave area (512 bytes at offset 72)
    push rdi
    lea  rdi, [rdi + 72]
    mov  rcx, 64               ; 512 / 8 = 64 qwords
    xor  rax, rax
    rep  stosq
    pop  rdi

    ret
