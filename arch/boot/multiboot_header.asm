; =============================================================================
; Σ SIGMAOS KERNEL: MULTIBOOT-HEADER (GRUB / Independent Boot)
; =============================================================================
; Principles:
;   - Multiboot 1 & 2 Compatibility.
;   - Allows SigmaOS to boot via GRUB, Syslinux, or Direct.
; =============================================================================

MAGIC       equ 0x1BADB002
FLAGS       equ (1 << 0) | (1 << 1)
CHECKSUM    equ -(MAGIC + FLAGS)

section .multiboot
align 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM

global _multiboot_start
_multiboot_start:
    ; Handover from GRUB
    ; EAX = 0x2BADB002
    ; EBX = pointer to multiboot info
    jmp sovereign_entry_32
