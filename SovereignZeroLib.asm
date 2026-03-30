; =============================================================================
; Σ SIGMAOS: SOVEREIGN ZEROLIB (v1.0 - ABSOLUTE RAW SYSCALLS)
; =============================================================================
; Mission: Zero-Standard-Lib. Zero-External-Symbols.
; Capability: Direct x86_64 Syscall Sharding for I/O and Memory.
; =============================================================================

SECTION .text
    GLOBAL _sigma_sys_write
    GLOBAL _sigma_sys_read
    GLOBAL _sigma_sys_mmap
    GLOBAL _sigma_sys_exit

; _sigma_sys_write(rdi: fd, rsi: buf, rdx: count)
_sigma_sys_write:
    mov rax, 1          ; sys_write
    syscall
    ret

; _sigma_sys_read(rdi: fd, rsi: buf, rdx: count)
_sigma_sys_read:
    mov rax, 0          ; sys_read
    syscall
    ret

; _sigma_sys_exit(rdi: code)
_sigma_sys_exit:
    mov rax, 60         ; sys_exit
    syscall
    ret

; _sigma_sys_mmap(rdi: addr, rsi: len, rdx: prot, r10: flags, r8: fd, r9: off)
_sigma_sys_mmap:
    mov rax, 9          ; sys_mmap
    syscall
    ret
