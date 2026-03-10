; =========================================================================
; Cosmos AI-OS: System Call Trampoline (x86_64)
; Mission: Microsecond latency user-to-kernel transitions.
; Security: SwapGS enforcement to prevent Meltdown/Spectre timing attacks.
; =========================================================================

global syscall_trampoline
extern cosmos_c_syscall_router

section .text

syscall_trampoline:
    ; 1. Hardware Boundary Protection
    ; The SYSCALL instruction does NOT save the user stack. We must do it.
    swapgs                  ; Switch to Kernel GS base
    mov [gs:0x10], rsp      ; Save User Space stack pointer
    mov rsp, [gs:0x08]      ; Load Ring-0 Kernel stack pointer

    ; 2. State Preservation (Caller-saved registers + Syscall ABI)
    ; SYSCALL puts the return address in RCX and RFLAGS in R11
    push r11                ; Save RFLAGS
    push rcx                ; Save Return RIP
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15

    ; 3. Argument Passing to C Router
    ; x86_64 Syscall ABI: RAX (ID), RDI, RSI, RDX, R10, R8, R9
    ; C ABI expects 4th arg in RCX, so move R10 to RCX
    mov rcx, r10            

    ; 4. Enter the Sovereign Core
    call cosmos_c_syscall_router

    ; 5. State Restoration
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop rcx                 ; Restore Return RIP
    pop r11                 ; Restore RFLAGS

    ; 6. Return to User Space
    mov rsp, [gs:0x10]      ; Restore User Space stack pointer
    swapgs                  ; Switch back to User GS base
    sysretq                 ; Hardware jump back to Ring-3
