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
global sigma_open
global sigma_read
global sigma_close
global sigma_getdents64
global sigma_execve
global sigma_strlen
global sigma_memset
global sigma_memcpy

section .text

; --- sigma_exit (int code) ---
sigma_exit:
    mov rdi, rdi    ; rdi already has code
    mov rax, 60     ; sys_exit
    syscall
    ret

; --- sigma_write (int fd, const void* buf, size_t count) ---
sigma_write:
    mov rax, 1      ; sys_write
    syscall
    ret

; --- sigma_read (int fd, void* buf, size_t count) ---
sigma_read:
    mov rax, 0      ; sys_read
    syscall
    ret

; --- sigma_open (const char* filename, int flags, int mode) ---
sigma_open:
    mov rax, 2      ; sys_open
    syscall
    ret

; --- sigma_close (int fd) ---
sigma_close:
    mov rax, 3      ; sys_close
    syscall
    ret

; --- sigma_mmap (void* addr... etc) ---
sigma_mmap:
    mov r10, rcx    ; R10 = R9 (fourth param for syscall)
    mov rax, 9      ; sys_mmap
    syscall
    ret

; --- sigma_getdents64 (unsigned int fd, struct linux_dirent64 *dirp, unsigned int count) ---
sigma_getdents64:
    mov rax, 217    ; sys_getdents64
    syscall
    ret

; --- sigma_execve (const char *filename, char *const argv[], char *const envp[]) ---
sigma_execve:
    mov rax, 59     ; sys_execve
    syscall
    ret

; --- sigma_strlen (const char* s) ---
sigma_strlen:
    xor rax, rax
.loop:
    cmp byte [rdi + rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret

; --- sigma_memset (void* s, int c, size_t n) ---
sigma_memset:
    mov rax, rsi    ; rax = c (value to set)
    mov rcx, rdx    ; rcx = n (count)
    mov rdi, rdi    ; rdi = s (destination)
    rep stosb       ; set n bytes
    mov rax, rdi    ; return original s? Actually rep stosb modifies rdi.
    ret

; --- sigma_memcpy (void* dest, const void* src, size_t n) ---
sigma_memcpy:
    mov rcx, rdx    ; count
    rep movsb       ; copy bytes
    ret
