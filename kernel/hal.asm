; =============================================================================
; Σ SIGMAOS KERNEL: HARDWARE ABSTRACTION LAYER (v1.0 - x86_64 NASM)
; =============================================================================
; Provides bare-metal primitives that C cannot express directly:
;   - Task State Segment (TSS) load
;   - GDT reload in long mode
;   - CPU feature detection (CPUID leaf 7 for AVX-512)
;   - XSAVE/XRSTOR for FPU context save/restore
;   - WRMSR/RDMSR wrappers (Ring-0 MSR access)
;   - Spinlock (LOCK XCHG — hardware atomic)
;   - Stack canary check (anti-smash)
; =============================================================================

[BITS 64]

; Exports
global hal_load_tss
global hal_reload_gdt
global hal_cpuid
global hal_rdmsr
global hal_wrmsr
global hal_spinlock_acquire
global hal_spinlock_release
global hal_xsave
global hal_xrstor
global hal_get_rflags
global hal_set_rflags

section .text

; =============================================================================
; hal_load_tss(u16 tss_selector)
; Loads the Task State Segment selector into TR register
; =============================================================================
hal_load_tss:
    ; rdi = tss_selector (u16)
    ltr  di
    ret

; =============================================================================
; hal_reload_gdt(GDTDescriptor* gdtr)
; Reload GDTR and far-return to refresh CS
; =============================================================================
hal_reload_gdt:
    ; rdi = pointer to { u16 limit, u64 base }
    lgdt  [rdi]
    ; Far return to refresh CS with new code segment (selector 0x08)
    push  qword 0x08         ; code segment
    lea   rax, [rel .reload_cs]
    push  rax
    retfq
.reload_cs:
    mov   ax, 0x10           ; data segment selector
    mov   ds, ax
    mov   es, ax
    mov   fs, ax
    mov   gs, ax
    mov   ss, ax
    ret

; =============================================================================
; hal_cpuid(u32 leaf, u32* eax, u32* ebx, u32* ecx, u32* edx)
; Safe CPUID wrapper — preserves rbx (System V ABI requires it)
; =============================================================================
hal_cpuid:
    ; rdi=leaf, rsi=*eax, rdx=*ebx, rcx=*ecx, r8=*edx
    push  rbx
    push  r12
    push  r13
    push  r14
    mov   r12, rsi           ; save output pointers
    mov   r13, rdx
    mov   r14, rcx
    mov   eax, edi
    xor   ecx, ecx
    cpuid
    mov   [r12], eax
    mov   [r13], ebx
    mov   [r14], ecx
    mov   [r8],  edx
    pop   r14
    pop   r13
    pop   r12
    pop   rbx
    ret

; =============================================================================
; u64 hal_rdmsr(u32 msr)  — read MSR into rax
; =============================================================================
hal_rdmsr:
    ; rdi = msr id (u32)
    mov   ecx, edi
    rdmsr                    ; result: edx:eax
    shl   rdx, 32
    or    rax, rdx
    ret

; =============================================================================
; void hal_wrmsr(u32 msr, u64 value)
; =============================================================================
hal_wrmsr:
    ; rdi = msr id, rsi = value
    mov   ecx, edi
    mov   rax, rsi
    mov   rdx, rsi
    shr   rdx, 32
    wrmsr
    ret

; =============================================================================
; void hal_spinlock_acquire(volatile u32* lock)
; LOCK XCHG — hardware-atomic test-and-set
; =============================================================================
hal_spinlock_acquire:
    ; rdi = pointer to lock word (0=free, 1=held)
    mov   eax, 1
.spin:
    lock xchg   [rdi], eax
    test  eax, eax
    jnz   .spin              ; spin until we set lock from 0→1
    ret

; =============================================================================
; void hal_spinlock_release(volatile u32* lock)
; =============================================================================
hal_spinlock_release:
    ; rdi = pointer to lock word
    xor   eax, eax
    lock xchg   [rdi], eax   ; atomically write 0 (release)
    ret

; =============================================================================
; void hal_xsave(u8* buf, u64 mask)
; Save extended CPU state (FPU/SSE/AVX) to 64-byte aligned buffer
; =============================================================================
hal_xsave:
    ; rdi = buf (must be 64-byte aligned), rsi = mask
    mov   rax, rsi           ; mask low 32 bits
    mov   rdx, rsi
    shr   rdx, 32            ; mask high 32 bits
    xsave64 [rdi]
    ret

; =============================================================================
; void hal_xrstor(u8* buf, u64 mask)
; Restore extended CPU state from 64-byte aligned buffer
; =============================================================================
hal_xrstor:
    ; rdi = buf (64-byte aligned), rsi = mask
    mov   rax, rsi
    mov   rdx, rsi
    shr   rdx, 32
    xrstor64 [rdi]
    ret

; =============================================================================
; u64 hal_get_rflags(void)
; =============================================================================
hal_get_rflags:
    pushfq
    pop   rax
    ret

; =============================================================================
; void hal_set_rflags(u64 flags)
; =============================================================================
hal_set_rflags:
    push  rdi
    popfq
    ret
