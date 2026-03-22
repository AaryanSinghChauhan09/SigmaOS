; SigmaOS Custom Native Fast-Ring Syscalls (x86_64 Assembly)
; ==========================================================
; Provides pure machine-language access to Linux/Windows Kernel without libc!

section .text
    global sigma_fast_syscall_linux
    global sigma_fast_syscall_windows
    global sigma_mem_copy_xmm

; -----------------------------------------------------------------------------
; sigma_fast_syscall_linux
; Direct int 0x80 / syscall wrapper for extremely low latency.
; Replaces libc's syscall() footprint.
; rdi = syscall number, rsi = arg1, rdx = arg2, rcx = arg3, r8 = arg4, r9 = arg5
sigma_fast_syscall_linux:
    ; System V AMD64 ABI mapping to Linux syscall ABI
    mov rax, rdi        ; syscall number
    mov rdi, rsi        ; arg1
    mov rsi, rdx        ; arg2
    mov rdx, rcx        ; arg3
    mov r10, r8         ; arg4
    mov r8, r9          ; arg5
    ; syscall!
    syscall
    ret

; -----------------------------------------------------------------------------
; sigma_fast_syscall_windows
; Replaces ntdll wrappers by calling ring-0 using sysenter/syscall.
; Note: Windows syscall indices vary by build, this acts as a stub wrapper 
; for direct NtQuery/System calls.
sigma_fast_syscall_windows:
    mov r10, rcx        ; fastcall convention mapping
    mov eax, ecx        ; syscall index
    syscall
    ret

; -----------------------------------------------------------------------------
; sigma_mem_copy_xmm (Custom Machine Language memcpy)
; Extremely fast 128-bit aligned memory copy for automation routing strings
; RDI = dest, RSI = src, RDX = size
sigma_mem_copy_xmm:
    test rdx, rdx
    jz .done
    
    ; If size >= 16 bytes, use SSE instructions
    cmp rdx, 16
    jl .byte_copy

.sse_loop:
    movups xmm0, [rsi]
    movups [rdi], xmm0
    add rsi, 16
    add rdi, 16
    sub rdx, 16
    cmp rdx, 16
    jge .sse_loop

.byte_copy:
    test rdx, rdx
    jz .done

.byte_loop:
    mov al, [rsi]
    mov [rdi], al
    inc rsi
    inc rdi
    dec rdx
    jnz .byte_loop

.done:
    ret
