; Σ SigmaOS Zenith — Stage 1 Bootloader
; Zero-Dependency: No GRUB. 16-bit Real Mode. MBR Boot Sector.
; Compiles with NASM: nasm -f bin boot.asm -o boot.bin

[BITS 16]
[ORG 0x7C00]

jmp short start
nop

; ------------------------------------------------------------------
; BIOS Parameter Block (FAT32) placeholder to appease some BIOSes
; ------------------------------------------------------------------
times 33 db 0

start:
    cli             ; Disable interrupts
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00  ; Stack grows downwards from bootloader
    sti             ; Enable interrupts

    ; Print "SigmaOS Booting..."
    mov si, msg_boot
    call print_string

    ; ------------------------------------------------------------------
    ; Load Stage 2 from disk (LBA 1, loading to 0x1000:0000)
    ; Assuming Stage 2 is 10 sectors long for now.
    ; ------------------------------------------------------------------
    mov ah, 0x42        ; Extended Read Sectors From Drive
    mov dl, 0x80        ; Drive 0 (first hard disk)
    mov si, disk_packet ; Pointer to Disk Address Packet (DAP)
    int 0x13
    jc disk_error

    ; Jump to Stage 2
    jmp 0x1000:0000

disk_error:
    mov si, msg_error
    call print_string
    hlt
    jmp disk_error

print_string:
    mov ah, 0x0E    ; Teletype output
.loop:
    lodsb
    test al, al
    jz .done
    int 0x10
    jmp .loop
.done:
    ret

; Disk Address Packet for INT 13h AH=42h
disk_packet:
    db 0x10         ; Size of packet
    db 0            ; Always 0
    dw 10           ; Number of sectors to read
    dw 0x0000       ; Offset
    dw 0x1000       ; Segment (loads to 0x10000)
    dq 1            ; Starting LBA (Sector 1, right after 0)

msg_boot db "SigmaOS Booting...", 13, 10, 0
msg_error db "Disk Error!", 13, 10, 0

times 510-($-$$) db 0
dw 0xAA55
