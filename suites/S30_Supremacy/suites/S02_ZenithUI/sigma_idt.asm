; =========================================================================
; SIGMA OS: BARE-METAL INTERRUPT ROUTING (ASM)
; Loads the IDT register and provides safe fallback instruction blocks.
; =========================================================================

global idt_flush
global isr_stub

section .text
bits 64

; -------------------------------------------------------------------------
; idt_flush
; Forces the CPU to update its internal Interrupt Matrix vector.
; Argument 1 (RDI): Pointer to the IDT ptr structure
; -------------------------------------------------------------------------
idt_flush:
    lidt [rdi]        ; Load the Interrupt Descriptor Table using RDI
    sti               ; Re-enable system hardware interrupts now that table is loaded
    ret

; -------------------------------------------------------------------------
; isr_stub
; The universal catch-all for undefined CPU exceptions (Division by Zero, 
; Page Faults, etc) preventing the OS from triple-faulting unexpectedly.
; -------------------------------------------------------------------------
isr_stub:
    cli               ; Block other cascading interrupts
    ; In a production OS this would map back into C error dumps
    ; For now, infinite system halt loop
    hlt
    jmp isr_stub
