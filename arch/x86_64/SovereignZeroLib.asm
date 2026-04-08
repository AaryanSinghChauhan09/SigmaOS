; =============================================================================
; Σ SIGMAOS: SOVEREIGN ZEROLIB (v1.0 - ABSOLUTE RAW SYSCALLS)
; =============================================================================
; Mission: Zero-Standard-Lib. Zero-External-Symbols.
; Capability: Direct x86_64 Syscall Sharding for I/O and Memory.
; =============================================================================

SECTION .text
    GLOBAL _sigma_sys_write
    GLOBAL _sigma_sys_read
    GLOBAL _sigma_sys_open
    GLOBAL _sigma_sys_close
    GLOBAL _sigma_sys_socket
    GLOBAL _sigma_sys_bind
    GLOBAL _sigma_sys_connect
    GLOBAL _sigma_sys_mmap
    GLOBAL _sigma_sys_exit
    GLOBAL _sigma_asm_strlen
    GLOBAL _sigma_asm_memcpy

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

; _sigma_sys_mmap(rdi: addr, rsi: len, rdx: prot, rcx: flags, r8: fd, r9: off)
_sigma_sys_mmap:
    mov r10, rcx        ; System Call uses R10 instead of RCX for 4th arg
    mov rax, 9          ; sys_mmap
    syscall
    ret

; _sigma_sys_open(rdi: filename, rsi: flags, rdx: mode)
_sigma_sys_open:
    mov rax, 2          ; sys_open
    syscall
    ret

; _sigma_sys_close(rdi: fd)
_sigma_sys_close:
    mov rax, 3          ; sys_close
    syscall
    ret

; _sigma_sys_socket(rdi: domain, rsi: type, rdx: protocol)
_sigma_sys_socket:
    mov rax, 41         ; sys_socket
    syscall
    ret

; _sigma_sys_bind(rdi: sockfd, rsi: addr, rdx: addrlen)
_sigma_sys_bind:
    mov rax, 49         ; sys_bind
    syscall
    ret

; _sigma_sys_connect(rdi: sockfd, rsi: addr, rdx: addrlen)
_sigma_sys_connect:
    mov rax, 42         ; sys_connect
    syscall
    ret

; _sigma_asm_strlen(rdi: s) -> rax: len
_sigma_asm_strlen:
    xor rax, rax        ; count = 0
.loop:
    cmp byte [rdi + rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret

; _sigma_asm_memcpy(rdi: dest, rsi: src, rdx: count)
_sigma_asm_memcpy:
    mov rcx, rdx        ; count
    rep movsb           ; copy bytes
    ret

