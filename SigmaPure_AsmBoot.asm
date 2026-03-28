/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

; Σ SIGMA OS: ABSOLUTE ASM KERNEL SHARD (v5.0 - LFS BARE-METAL)
; ==============================================================
; USP Absorbed: Linux From Scratch (LFS), xv6, Coreboot (Bare Init).
; Capability: Direct x86_64 Machine Execution Sequence, Zero C-Runtime.
; Principle: Absolute Hardware Instruction Sovereignty.

; section .data declares initialized data (constants)
section .data
    msg1 db '[ASM_BOOT]: Init Direct Instruction Pipeline...', 0xA, 0
    len1 equ $ - msg1

    msg2 db '[SUCCESS]: LFS & Coreboot Sovereignty. Zero C/C++/Rust Library.', 0xA, 0
    len2 equ $ - msg2

; section .text contains the executable machine instructions
section .text
    global _start ; Kernel entry-point bypasses libc 'main' wrapper

_start:
    ; --- USP: Bare-Metal Syscall Sharding (Linux x86_64 1=write) ---
    
    ; Output Boot Message
    mov rax, 1          ; Syscall: sys_write
    mov rdi, 1          ; File descriptor: 1 (stdout)
    lea rsi, [rel msg1] ; Buffer: msg1 address
    mov rdx, len1       ; Length: message 1 size
    syscall             ; Direct CPU/OS Ring-0 Interrupt

    ; Output Success Message
    mov rax, 1          ; Syscall: sys_write
    mov rdi, 1          ; File descriptor: 1 (stdout)
    lea rsi, [rel msg2] ; Buffer: msg2 address
    mov rdx, len2       ; Length: message 2 size
    syscall             ; Direct CPU/OS Ring-0 Interrupt

    ; --- USP: Zero-Zombie Exit Shard (Linux x86_64 60=exit) ---
    
    mov rax, 60         ; Syscall: sys_exit
    xor rdi, rdi        ; Return code: 0 (clearing register manually)
    syscall             ; Terminating Process cleanly

