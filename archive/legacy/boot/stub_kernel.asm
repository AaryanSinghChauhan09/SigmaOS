; SigmaOS Stub Kernel
; Minimal x86_64 kernel loaded via Multiboot2.
; Prints "SigmaOS v0.1 - Sovereign OS Loading..." to VGA text mode then halts.
; GALL'S LAW: Start simple — a working stub is better than a broken complex kernel.

bits 32

; ============================================================
; Multiboot2 header (must be in first 32768 bytes of image)
; ============================================================
section .multiboot2_header
ALIGN 8
mb2_start:
    dd 0xe85250d6                           ; Multiboot2 magic
    dd 0                                    ; architecture: i386 protected mode
    dd mb2_end - mb2_start                  ; header length
    dd -(0xe85250d6 + 0 + (mb2_end - mb2_start))  ; checksum

    ; End tag
    dw 0                                    ; type = 0 (end)
    dw 0                                    ; flags
    dd 8                                    ; size
mb2_end:

; ============================================================
; Kernel entry (32-bit protected mode, Multiboot2 loaded here)
; ============================================================
section .text
global _start
_start:
    ; Disable interrupts for safety
    cli

    ; Set up a minimal stack
    mov esp, stack_top

    ; Clear VGA screen (80x25, 0xB8000)
    call clear_screen

    ; Print the boot message
    mov esi, boot_msg
    mov edi, 0xB8000
    call print_vga

    ; Print a second line with status
    mov esi, status_msg
    mov edi, 0xB8000 + (80 * 2)           ; row 2 (skip row 0 and 1 for centering feel)
    call print_vga

    ; Serial port output (COM1 = 0x3F8) for QEMU -serial stdio
    mov esi, serial_msg
    call print_serial

    ; Hang forever
.halt:
    hlt
    jmp .halt

; ============================================================
; clear_screen: fills VGA buffer with spaces (white on black)
; ============================================================
clear_screen:
    push eax
    push ecx
    push edi
    mov edi, 0xB8000
    mov ecx, 80 * 25
    mov ax, 0x0720                         ; space + light grey on black
    rep stosw
    pop edi
    pop ecx
    pop eax
    ret

; ============================================================
; print_vga: prints null-terminated string at ESI to EDI
; Attribute byte: 0x0A = light green on black (sovereign vibes)
; ============================================================
print_vga:
    push eax
    push esi
    push edi
.loop:
    lodsb                                  ; AL = *ESI++
    test al, al
    jz .done
    mov ah, 0x0A                           ; attribute: light green on black
    stosw                                  ; write char+attr to video memory
    jmp .loop
.done:
    pop edi
    pop esi
    pop eax
    ret

; ============================================================
; print_serial: send null-terminated string at ESI to COM1
; Waits for transmit-hold-empty before each byte (polling)
; ============================================================
print_serial:
    push eax
    push esi
    push edx
.loop:
    lodsb
    test al, al
    jz .done
    ; Wait for transmitter empty (bit 5 of Line Status Register 0x3FD)
.wait:
    mov edx, 0x3FD
    in al, dx
    test al, 0x20
    jz .wait
    ; Send the character
    mov edx, 0x3F8
    lodsb
    dec esi                                ; compensate for lodsb above
    lodsb
    out dx, al
    jmp .loop
.done:
    pop edx
    pop esi
    pop eax
    ret

; ============================================================
; Read-only data
; ============================================================
section .rodata
boot_msg:
    db "  SigmaOS v0.1 - Sovereign OS Loading...                               ", 0
status_msg:
    db "  [OK] Stub kernel alive. Real kernel not yet compiled. Halting safely. ", 0
serial_msg:
    db "SigmaOS v0.1 - Sovereign OS Loading...", 13, 10
    db "[stub] Multiboot2 kernel alive on COM1 serial.", 13, 10
    db "[stub] Real Rust kernel not yet compiled. Halting.", 13, 10, 0

; ============================================================
; BSS: stack
; ============================================================
section .bss
align 16
stack_bottom:
    resb 16384                             ; 16 KiB stub stack
stack_top:
