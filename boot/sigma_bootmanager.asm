; SigmaOS Native Enterprise Bootloader (x86_16 Real Mode)
; ========================================================
; USP: Direct hardware initialization from scratch, bypassing GRUB.
; Compiles cleanly to a 512-byte MBR sector (0xAA55).

org 0x7C00
bits 16

start:
    ; 1. Segment Initialization (OOP equivalents in logic flow)
    cli                 ; Clear interrupts
    xor ax, ax          ; Zero out AX
    mov ds, ax          ; Data Segment
    mov es, ax          ; Extra Segment
    mov ss, ax          ; Stack Segment
    mov sp, 0x7C00      ; Stack point starts at boot sector
    sti                 ; Restore interrupts

    ; 2. Enterprise Screen Initialization (VGA Mode 0x0E)
    mov ah, 0x00
    mov al, 0x03        ; Text mode 80x25
    int 0x10

    ; 3. Print Banner
    mov si, boot_msg
    call print_string

    ; 4. Kernel Handoff Preparation
    ; (Here, SigmaOS would leap into protected mode and handoff to sigma_kernel_core.c)
    jmp $               ; Infinite loop for now (Halt CPU)

; Pure low-level string execution
print_string:
    mov ah, 0x0E        ; BIOS Teletype Output
.loop:
    lodsb               ; Load string byte
    cmp al, 0
    je .done            ; End string if null terminator
    int 0x10            ; Print char
    jmp .loop
.done:
    ret

boot_msg db 'SigmaOS (Apex v6.7): Enterprise Execution Mode Active. Loading C-Kernel...', 0x0D, 0x0A, 0

; 5. Boot Sector Padding
times 510-($-$$) db 0
dw 0xAA55               ; Boot Signature
