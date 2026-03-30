; ═══════════════════════════════════════════════════════════════
; Σ SIGMAOS ZENITH SUPREME: SOVEREIGN SILICON LOGGING (v94.0)
; ═══════════════════════════════════════════════════════════════
; Mission: Direct hardware-level output without C-library bloat.
; Principle: Zero-Standard-Library, Raw Syscalls.
; ═══════════════════════════════════════════════════════════════

[bits 64]

global sigma_silicon_log
global sigma_silicon_exit

section .text

; -----------------------------------------------------------------------------
; sigma_silicon_log(const char* message, uint64_t length)
; rdi = message, rsi = length
; -----------------------------------------------------------------------------
sigma_silicon_log:
    push rbp
    mov rbp, rsp
    
    mov rax, 1          ; sys_write
    mov rdx, rsi        ; count
    mov rsi, rdi        ; buf
    mov rdi, 1          ; fd (stdout)
    syscall             ; silicon-direct interrupt
    
    pop rbp
    ret

; -----------------------------------------------------------------------------
; sigma_silicon_exit(int code)
; rdi = code
; -----------------------------------------------------------------------------
sigma_silicon_exit:
    mov rax, 60         ; sys_exit
    syscall             ; halt industrial silicon
    ret
