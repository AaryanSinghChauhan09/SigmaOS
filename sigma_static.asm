; -----------------------------------------------------------------------------
; SigmaOS Enterprise Static Shard v1.0 (x86_64 Assembly)
; Inspiration: snacklinux (Tiny, Static binaries).
; USP: Zero-Dependency Silicon-Direct Minimal Zenith.
; Principle: Static Mastery & Performance.
; -----------------------------------------------------------------------------

[SECTION .text]
global _start

_start:
    ; syscall: write(1, message, 38)
    mov rax, 1
    mov rdi, 1
    mov rsi, sigma_msg
    mov rdx, 38
    syscall

    ; syscall: exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall

[SECTION .data]
sigma_msg: db "[STATIC]: Static Zenith Operational.", 10
