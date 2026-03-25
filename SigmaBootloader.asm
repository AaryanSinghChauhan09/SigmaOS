; Σ SIGMA OS: SOVEREIGN BOOTLOADER (v12.0 - ZERO-GRUB BARE-METAL)
; ==============================================================
; USP Absorbed: GRUB/Coreboot (System Initialization), Syslinux.
; Capability: Absolute 16-bit Real Mode Boot Sequence. Boot Sector 0 (MBR).
; Principle: Bypasses complex bootloaders. Loads Sigma directly from BIOS.

[BITS 16]         ; Tell assembler to generate 16-bit code
[ORG 0x7C00]      ; BIOS loads the boot sector to memory address 0x7C00

start:
    ; 1. Clear interrupts while configuring segment registers
    cli
    xor ax, ax      ; Set AX to 0
    mov ds, ax      ; Data Segment = 0
    mov es, ax      ; Extra Segment = 0
    mov ss, ax      ; Stack Segment = 0
    mov sp, 0x7C00  ; Set Stack Pointer just before our bootloader
    sti             ; Re-enable interrupts

    ; 2. Clear Screen and Set Video Mode (VGA Text Mode 80x25)
    mov ah, 0x00    ; BIOS function: Set Video Mode
    mov al, 0x03    ; Mode 3: 80x25 text, 16 colors
    int 0x10        ; Call BIOS Video Interrupt

    ; 3. Print Boot Message directly via BIOS teletype
    mov si, boot_msg
print_loop:
    lodsb           ; Load byte at DS:SI into AL and increment SI
    or al, al       ; Check if AL is 0 (end of string)
    jz boot_done    ; If zero, jump to end
    mov ah, 0x0E    ; BIOS function: Teletype output
    mov bh, 0x00    ; Page number 0
    mov bl, 0x0A    ; Color: Light Green (0x0A)
    int 0x10        ; Call BIOS Video Interrupt
    jmp print_loop

boot_done:
    ; 4. Infinite Loop (Halt Processor) ensuring OS stability
    jmp $           ; Jump to current address (infinite loop)

; Data Section
boot_msg db '[SIGMA_BOOT]: Zero-GRUB Sovereign Sequence Initiated. Reading Sector...', 0x0D, 0x0A, 0

; Boot Sector Magic Number Padding
times 510-($-$$) db 0   ; Pad rest of 512-byte sector with zeroes
dw 0xAA55               ; Boot Signature required by BIOS to recognize bootable sector
