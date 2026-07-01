; =========================================================================
; SIGMA OS: BOOTLOADER ENTRY PROTOCOL (v2.0 — Hardened)
; Multiboot 1 Compliant Header. Bypasses GRUB into Bare-Metal Kernel.
;
; Changes vs v1.0:
;  - x87 FPU + SSE2 explicitly initialized before kmain.
;  - Stack aligned to 16 bytes (required by System V ABI / SSE).
;  - EFLAGS registers cleared for deterministic startup.
;  - CPUID guard for SSE2; halts gracefully if unavailable.
; =========================================================================

MAGIC    equ 0x1BADB002
FLAGS    equ 0x03
CHECKSUM equ -(MAGIC + FLAGS)
[BITS 32]

section .multiboot
align 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM

section .bss
align 16
stack_bottom:
    resb 32768          ; 32 KB kernel stack (was 16 KB)
stack_top:

section .text
global _start
extern kmain

_start:
    ; ── 1. Establish the 16-byte-aligned kernel stack ─────────────
    mov esp, stack_top
    and esp, 0xFFFFFFF0     ; Align to 16-byte boundary (ABI requirement)

    ; ── 2. Clear EFLAGS to a deterministic state ───────────────────
    push 0x00000002         ; Reserved bit 1 must be set
    popf

    ; ── 3. Initialize x87 FPU ──────────────────────────────────────
    fninit

    ; ── 4. Check for SSE2 via CPUID ────────────────────────────────
    mov eax, 1
    cpuid
    test edx, (1 << 26)     ; EDX bit 26 = SSE2
    jz  .no_sse2

    ; ── 5. Enable SSE2: CR0 & CR4 bits ────────────────────────────
    mov eax, cr0
    and eax, ~(1 << 2)      ; Clear EM (x87 emulation)
    or  eax, (1 << 1)       ; Set MP (monitor coprocessor)
    mov cr0, eax

    mov eax, cr4
    or  eax, (3 << 9)       ; Set OSFXSR (bit 9) + OSXMMEXCPT (bit 10)
    mov cr4, eax

    ; ── 6. Transfer to C Sovereign Execution Matrix ────────────────
    push ebx                ; Multiboot info structure
    push eax                ; Multiboot magic number
    call kmain

    ; ── 7. Total Halt Loop ─────────────────────────────────────────
    cli
.hang:
    hlt
    jmp .hang

.no_sse2:
    ; Minimal error display before halt (requires VGA text mode)
    mov byte [0xB8000], 'E'
    mov byte [0xB8001], 0x4F   ; White on Red attribute
    cli
    hlt
