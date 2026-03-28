; =========================================================================
; Σ SIGMAOS: SOVEREIGN LIBC (v19.0 - x86_64 Direct Syscalls)
; =========================================================================
; Mission: Direct Hardware Interface (No C Library).
; Capability: x86_64 Linux/Windows Core Sharding (Target: Linux-Kernel-Zenith).
; OS: Metal-Native Root.
; =========================================================================

[BITS 64]

global sigma_exit
global sigma_write
global sigma_mmap

section .text

; --- sigma_exit (int code) ---
sigma_exit:
    mov rdi, rax    ; RDI = code
    mov rax, 60     ; sys_exit
    syscall
    ret

; --- sigma_write (int fd, const void* buf, size_t count) ---
sigma_write:
    ; RDI, RSI, RDX already in place for syscall 1 (sys_write)
    mov rax, 1      ; sys_write
    syscall
    ret

; --- sigma_mmap (void* addr... etc) ---
sigma_mmap:
    mov r10, rcx    ; R10 = R9 (fourth param for syscall)
    mov rax, 9      ; sys_mmap
    syscall
    ret

; --- String Length (Direct) ---
global sigma_strlen
sigma_strlen:
    xor rax, rax
.loop:
    cmp byte [rdi + rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret
