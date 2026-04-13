; =============================================================================
; Σ SIGMAOS: SOVEREIGN CORE ASSEMBLY LIBRARY (v150.0 - PURITY EDITION)
; =============================================================================
; Mission: Zero-Library, Zero-HLL (High Level Language) Dependency.
; Target: x86_64
; Description: Raw silicon-direct implementation of OS primitives.
; =============================================================================

section .text
    global _sigma_sys_write
    global _sigma_sys_read
    global _sigma_sys_mmap
    global _sigma_sys_munmap
    global _sigma_sys_exit
    global _sigma_strlen
    global _sigma_memcpy
    global _sigma_memset
    global _sigma_streq
    global _sigma_atoi

; --- SYSCALL WRAPPERS (Linux ABI Parity - Aether-WSW Bridge) ---

_sigma_sys_write:
    ; rdi: fd, rsi: buf, rdx: count
    mov rax, 1          ; sys_write
    syscall
    ret

_sigma_sys_read:
    ; rdi: fd, rsi: buf, rdx: count
    mov rax, 0          ; sys_read
    syscall
    ret

_sigma_sys_mmap:
    ; rdi: addr, rsi: len, rdx: prot, r10: flags, r8: fd, r9: off
    mov rax, 9          ; sys_mmap
    syscall
    ret

_sigma_sys_munmap:
    ; rdi: addr, rsi: len
    mov rax, 11         ; sys_munmap
    syscall
    ret

_sigma_sys_exit:
    ; rdi: status
    mov rax, 60         ; sys_exit
    syscall

; --- CORE UTILS (HLL-Replacement) ---

_sigma_strlen:
    ; rdi: s
    xor rax, rax
.loop:
    cmp byte [rdi + rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret

_sigma_memcpy:
    ; rdi: dest, rsi: src, rdx: n
    mov rcx, rdx
    rep movsb
    ret

_sigma_memset:
    ; rdi: s, rsi: c (al), rdx: n
    mov rcx, rdx
    mov rax, rsi
    rep stosb
    ret

_sigma_streq:
    ; rdi: s1, rsi: s2
    xor rax, rax
.loop:
    mov al, [rdi]
    mov cl, [rsi]
    cmp al, cl
    jne .not_equal
    cmp al, 0
    je .equal
    inc rdi
    inc rsi
    jmp .loop
.not_equal:
    xor rax, rax
    ret
.equal:
    mov rax, 1
    ret

_sigma_atoi:
    ; rdi: str
    xor rax, rax        ; result
    xor rcx, rcx        ; current char
.loop:
    movzx rcx, byte [rdi]
    cmp rcx, 0
    je .done
    cmp rcx, '0'
    jb .done
    cmp rcx, '9'
    ja .done
    sub rcx, '0'
    imul rax, 10
    add rax, rcx
    inc rdi
    jmp .loop
.done:
    ret

; =============================================================================
; Σ SIGMAOS: SILICON SOVEREIGNTY. ZERO DEPENDENCY.
; =============================================================================

