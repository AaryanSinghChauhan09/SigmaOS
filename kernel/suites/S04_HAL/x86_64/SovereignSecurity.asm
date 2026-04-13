; =============================================================================
; Σ SIGMAOS: SOVEREIGN SECURITY (v1.0 - ABSOLUTE SILICON SCRUBBER)
; =============================================================================
; Mission: Zero-Leak. Zero-Trace. Raw Register/Stack Sanitization.
; Capability: Scrubbing Silicon State before Task Switch or Exit.
; =============================================================================

SECTION .text
    GLOBAL sigma_security_scrub_registers
    GLOBAL sigma_security_scrub_stack

; sigma_security_scrub_registers(): Clears all GPRs to prevent info leaks.
sigma_security_scrub_registers:
    xor rax, rax
    xor rbx, rbx
    xor rcx, rcx
    xor rdx, rdx
    xor rsi, rsi
    xor rdi, rdi
    xor r8, r8
    xor r9, r9
    xor r10, r10
    xor r11, r11
    xor r12, r12
    xor r13, r13
    xor r14, r14
    xor r15, r15
    ret

; sigma_security_scrub_stack(uint64_t size): Zero-fills the stack shard.
sigma_security_scrub_stack:
    push rbp
    mov rbp, rsp
    
    mov rcx, rdi        ; size (in bytes)
    shr rcx, 3          ; convert to qwords (size / 8)
    mov rdi, rsp        ; start from current SP
    add rdi, 16         ; skip preserved rbp and return address
    xor rax, rax        ; zero value
    
    rep stosq           ; zero-fill the specified stack range
    
    pop rbp
    ret

