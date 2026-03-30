; =============================================================================
; Σ SIGMAOS: SOVEREIGN ENTRY (v1.0 - ABSOLUTE STARTUP FINALITY)
; =============================================================================
; Mission: Zero-CRT0. Zero-Main. Pure ASM Entry Point.
; Capability: Bypassing Compiler-Provided Startup Logic.
; =============================================================================

SECTION .text
    GLOBAL _start
    EXTERN sigma_main

_start:
    ; Absolute Scratch Entry: No pre-defined startup.
    xor rbp, rbp        ; Clear RBP (per SysV ABI)
    
    ; Setup Stack for sigma_main
    mov rdi, [rsp]      ; argc
    lea rsi, [rsp + 8]  ; argv
    
    ; Launch Sovereign Logic
    call sigma_main
    
    ; Absolute Exit via Raw Syscall
    mov rdi, rax        ; Use sigma_main return value as exit code
    mov rax, 60         ; sys_exit
    syscall
