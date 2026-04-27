; =============================================================================
; Σ SIGMAOS KERNEL: SOVEREIGN-BOOT (x86_64 Minimal Entry)
; =============================================================================
; Principles:
;   - Minimal dependency on UEFI/BIOS.
;   - Direct transition to Long Mode.
;   - Setup minimal GDT and Page Tables.
; =============================================================================

[BITS 32]
section .boot

global _start
extern kernel_main

_start:
    cli                         ; Clear interrupts
    mov esp, stack_top          ; Setup temporary stack

    ; Check for Long Mode support (simplified for demo)
    ; In real hardware: CPUID check, PAE setup, LME bit in EFER

    ; Transition to Long Mode (Placeholder for sharded transition)
    ; call setup_paging
    ; call setup_gdt
    
    ; Jump to 64-bit kernel entry
    jmp 0x08:long_mode_entry

[BITS 64]
long_mode_entry:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Call the C kernel entry point
    call kernel_main

    ; If kernel returns, halt
.halt:
    hlt
    jmp .halt

section .bss
align 16
stack_bottom:
    resb 16384                  ; 16 KB stack
stack_top:
