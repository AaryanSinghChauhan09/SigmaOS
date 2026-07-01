/**
 * SIGMAOS: SOVEREIGN MEMORY PRIMITIVES (x86_64 ASSEMBLY)
 * Purpose: Absolute performance. Zero high-level language dependency.
 * Principle: Direct Silicon Orchestration.
 */

.section .text
.global sigma_memcpy_asm
.global sigma_memset_asm
.global sigma_strlen_asm

# void* sigma_memcpy_asm(void* dest, const void* src, sigma_size_t n)
# RDI = dest, RSI = src, RDX = n
sigma_memcpy_asm:
    mov %rdi, %rax          # Return value is dest
    mov %rdx, %rcx          # RCX = count
    rep movsb               # Copy RCX bytes from RSI to RDI
    ret

# void* sigma_memset_asm(void* s, int c, sigma_size_t n)
# RDI = s, RSI = c, RDX = n
sigma_memset_asm:
    mov %rdi, %rax          # Return value is s
    mov %rsi, %rcx          # RCX = value (c)
    mov %rdx, %r8           # Save n
    mov %rcx, %rax          # AL = c
    mov %r8, %rcx           # RCX = n
    rep stosb               # Fill RCX bytes with AL starting at RDI
    mov %rdi, %rax          # Return s
    ret

# sigma_size_t sigma_strlen_asm(const char* s)
# RDI = s
sigma_strlen_asm:
    xor %rax, %rax          # RAX = 0 (counter)
.L_strlen_loop:
    cmpb $0, (%rdi, %rax)   # Is s[RAX] == 0?
    je .L_strlen_done
    inc %rax
    jmp .L_strlen_loop
.L_strlen_done:
    ret
