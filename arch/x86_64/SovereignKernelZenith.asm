; =========================================================================
; Σ SIGMAOS: SOVEREIGN KERNEL ZENITH (v1.0 - ASSEMBLY METAL)
; =========================================================================
; Mission: Silicon-Direct Execution & Hardware Handshake.
; Principle: Zero-Standard-Bootloader. Pure Hand-Optimized Assembly.
; =========================================================================

[BITS 64]

section .text
global _start
global sigma_hw_enter_ring0
global sigma_hw_context_switch
global sigma_hw_sysexit

; --- KERNEL ENTRY POINT (From Bootloader) ---
_start:
    ; 1. Disable Interrupts
    cli
    
    ; 2. Reload GDT for Sovereign Ring-0
    lgdt [gdt_ptr]
    
    ; 3. Setup Stack
    mov rsp, kernel_stack_top
    
    ; 4. Enable SYSCALL/SYSRET (EFER-SCE)
    mov rcx, 0xC0000080 ; EFER MSR
    rdmsr
    or eax, 1           ; SCE (System Call Enable)
    wrmsr
    
    ; 5. Set Syscall Entry Point (LSTAR)
    mov rcx, 0xC0000082 ; LSTAR MSR
    mov rax, sigma_hw_syscall_entry
    mov rdx, rax
    shr rdx, 32
    wrmsr
    
    ; 6. Call C Kernel Bootstrap
    extern start_kernel_zenith
    call start_kernel_zenith
    
    ; 7. Final Halt (Should never reach)
.halt:
    hlt
    jmp .halt

; --- SOVEREIGN SYSCALL ENTRY (LSTAR) ---
; Ring-3 -> Ring-0 transition via SYSCALL instruction.
; RCX = User RIP, R11 = User RFLAGS
sigma_hw_syscall_entry:
    swapgs ; Switch to kernel thread local storage
    
    ; Save user registers (Simplified)
    push rbp
    push rdi
    push rsi
    push r11
    push rcx
    
    ; Call C Dispatcher
    extern sovereign_syscall_dispatch
    ; RDI, RSI, RDX are already used by caller for syscall args
    call sovereign_syscall_dispatch
    
    ; Restore user registers
    pop rcx
    pop r11
    pop rsi
    pop rdi
    pop rbp
    
    swapgs
    sysretq

; --- CONTEXT SWITCH (The Hot Path) ---
; void sigma_hw_context_switch(void* from_stack, void* to_stack)
sigma_hw_context_switch:
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    
    mov [rdi], rsp ; Save current stack pointer
    mov rsp, rsi   ; Load new stack pointer
    
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    ret

section .data
align 8
gdt_start:
    dq 0x0000000000000000 ; Null Segment
    dq 0x00AF9A000000FFFF ; Kernel Code (Ring 0)
    dq 0x00AF92000000FFFF ; Kernel Data (Ring 0)
    dq 0x00AFFB000000FFFF ; User Code (Ring 3)
    dq 0x00AFF3000000FFFF ; User Data (Ring 3)
gdt_end:

gdt_ptr:
    dw gdt_end - gdt_start - 1
    dq gdt_start

section .bss
align 16
kernel_stack_bottom:
    resb 4096 * 4 ; 16KB Kernel Stack
kernel_stack_top:

