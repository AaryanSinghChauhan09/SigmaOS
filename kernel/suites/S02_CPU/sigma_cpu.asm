; =========================================================================
; SIGMA OS: BARE-METAL CPU SUBROUTINES (ASM)
; Low-level Assembly instructions to interface with modern silicon registers.
; =========================================================================

global gdt_flush

section .text
bits 64

; -------------------------------------------------------------------------
; gdt_flush
; Force the CPU to load the custom Global Descriptor Table.
; Argument 1 (RDI): Pointer to the GDT structure
; -------------------------------------------------------------------------
gdt_flush:
    lgdt [rdi]        ; Load the GDT pointer from RDI parameter

    ; Set all data segment registers to our Kernel Data Segment (0x10 is absolute 2nd entry: 2 * 8)
    mov ax, 0x10      ; 0x10 is the offset in the GDT to our data segment
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Far jump to flush instruction pipeline and set CS to Kernel Code (0x08 is 1st entry: 1 * 8)
    ; Push the code segment selector and return address to simulate a far return
    pop rdi           ; Get return address
    push 0x08         ; Push kernel code segment
    push rdi          ; Push return address back
    o64 retf          ; 64-bit far return (forces CS update)
