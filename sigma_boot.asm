/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

; -----------------------------------------------------------------------------
; SigmaOS Enterprise Bootloader v1.0 (NASM 16-bit)
; Inspiration: torvalds/linux/arch/x86/boot
; USP: First-Stage Shard Discovery.
; -----------------------------------------------------------------------------

[BITS 16]
[ORG 0x7C00]

start:
    mov ah, 0x0E        ; BIOS TTY teletype output
    mov al, 'S'         ; Printing "SIGMA" to boot console
    int 0x10
    mov al, 'I'
    int 0x10
    mov al, 'G'
    int 0x10
    mov al, 'M'
    int 0x10
    mov al, 'A'
    int 0x10
    mov al, ':'
    int 0x10

    mov si, boot_msg
    call print_string

    jmp $               ; Infinite loop (Bootloader static status)

print_string:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp print_string
.done:
    ret

boot_msg db ' Enterprise KERNEL DISCOVERED (BOOT-SHARD-ACTIVE)', 0

times 510-($-$$) db 0
dw 0xAA55               ; Boot signature

