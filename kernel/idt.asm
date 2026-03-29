; =============================================================================
; Σ SIGMAOS KERNEL: INTERRUPT DESCRIPTOR TABLE (v1.0 - x86_64 NASM)
; =============================================================================
; Provides: IDT entry stubs for all 256 vectors (ISRs + IRQs)
; - ISR 0-31:  CPU exceptions (with/without error codes per Intel spec)
; - IRQ 0-15:  Hardware interrupts (PIC-remapped to vectors 32-47)
; - Syscall:   Vector 128 (0x80) — SigmaOS system call gate
; Each stub saves CPU state → calls common C handler sigma_interrupt_handler()
; =============================================================================

[BITS 64]
[GLOBAL idt_load]
[GLOBAL isr_stub_table]
[EXTERN sigma_interrupt_handler]

; =============================================================================
; Common ISR entry: save all registers, call C handler, restore, return
; =============================================================================
%macro ISR_NOERR 1
isr_%1:
    push  qword 0          ; fake error code
    push  qword %1         ; vector number
    jmp   isr_common
%endmacro

%macro ISR_ERR 1
isr_%1:
    push  qword %1         ; vector number (error code already on stack by CPU)
    jmp   isr_common
%endmacro

%macro IRQ 2
irq_%1:
    push  qword 0
    push  qword %2
    jmp   isr_common
%endmacro

; CPU exceptions — Intel SDM §6.3.1
ISR_NOERR  0   ; Divide-by-zero
ISR_NOERR  1   ; Debug
ISR_NOERR  2   ; NMI
ISR_NOERR  3   ; Breakpoint
ISR_NOERR  4   ; Overflow
ISR_NOERR  5   ; Bound Range Exceeded
ISR_NOERR  6   ; Invalid Opcode
ISR_NOERR  7   ; Device Not Available
ISR_ERR    8   ; Double Fault (error code = 0)
ISR_NOERR  9   ; Coprocessor Segment Overrun (legacy)
ISR_ERR    10  ; Invalid TSS
ISR_ERR    11  ; Segment Not Present
ISR_ERR    12  ; Stack-Segment Fault
ISR_ERR    13  ; General Protection Fault
ISR_ERR    14  ; Page Fault
ISR_NOERR  15  ; Reserved
ISR_NOERR  16  ; x87 Floating-Point Exception
ISR_ERR    17  ; Alignment Check
ISR_NOERR  18  ; Machine Check
ISR_NOERR  19  ; SIMD Floating-Point Exception
ISR_NOERR  20  ; Virtualization Exception
ISR_ERR    21  ; Control Protection Exception
ISR_NOERR  22
ISR_NOERR  23
ISR_NOERR  24
ISR_NOERR  25
ISR_NOERR  26
ISR_NOERR  27
ISR_NOERR  28  ; Hypervisor Injection
ISR_ERR    29  ; VMM Communication
ISR_ERR    30  ; Security Exception
ISR_NOERR  31  ; Reserved

; Hardware IRQs (PIC remapped to 32–47)
IRQ  0, 32    ; PIT Timer
IRQ  1, 33    ; Keyboard
IRQ  2, 34    ; Cascade (slave PIC)
IRQ  3, 35    ; COM2
IRQ  4, 36    ; COM1
IRQ  5, 37    ; LPT2
IRQ  6, 38    ; Floppy
IRQ  7, 39    ; LPT1 / spurious
IRQ  8, 40    ; CMOS RTC
IRQ  9, 41    ; Free
IRQ 10, 42    ; Free
IRQ 11, 43    ; Free
IRQ 12, 44    ; PS/2 Mouse
IRQ 13, 45    ; FPU
IRQ 14, 46    ; Primary ATA
IRQ 15, 47    ; Secondary ATA

; Syscall gate — vector 0x80
ISR_NOERR 128

; =============================================================================
; Common ISR handler body
; Structure on entry (System V AMD64 ABI stack frame):
;   [rsp+0]  = vector
;   [rsp+8]  = error code
;   [rsp+16] = rip (pushed by CPU)
;   [rsp+24] = cs
;   [rsp+32] = rflags
;   [rsp+40] = rsp (user)
;   [rsp+48] = ss
; =============================================================================
isr_common:
    ; Save general-purpose registers (System V clobber set)
    push  rax
    push  rbx
    push  rcx
    push  rdx
    push  rsi
    push  rdi
    push  rbp
    push  r8
    push  r9
    push  r10
    push  r11
    push  r12
    push  r13
    push  r14
    push  r15

    ; Pass pointer to register frame as first argument
    mov   rdi, rsp          ; arg1: *SigmaInterruptFrame

    ; Save segment registers
    mov   ax, ds
    push  rax
    mov   ax, es
    push  rax
    mov   ax, 0x10          ; kernel data segment
    mov   ds, ax
    mov   es, ax

    ; Call C handler
    call  sigma_interrupt_handler

    ; Restore segment registers
    pop   rax
    mov   es, ax
    pop   rax
    mov   ds, ax

    ; Restore GPRs
    pop   r15
    pop   r14
    pop   r13
    pop   r12
    pop   r11
    pop   r10
    pop   r9
    pop   r8
    pop   rbp
    pop   rdi
    pop   rsi
    pop   rdx
    pop   rcx
    pop   rbx
    pop   rax

    ; Remove vector + error code pushed by stubs
    add   rsp, 16

    iretq

; =============================================================================
; ISR Stub Pointer Table (accessible from C as isr_stub_table[])
; =============================================================================
section .data
isr_stub_table:
    dq isr_0,  isr_1,  isr_2,  isr_3,  isr_4,  isr_5,  isr_6,  isr_7
    dq isr_8,  isr_9,  isr_10, isr_11, isr_12, isr_13, isr_14, isr_15
    dq isr_16, isr_17, isr_18, isr_19, isr_20, isr_21, isr_22, isr_23
    dq isr_24, isr_25, isr_26, isr_27, isr_28, isr_29, isr_30, isr_31
    dq irq_0,  irq_1,  irq_2,  irq_3,  irq_4,  irq_5,  irq_6,  irq_7
    dq irq_8,  irq_9,  irq_10, irq_11, irq_12, irq_13, irq_14, irq_15
    ; Fill 48→127 with isr_22 (generic reserved handler) as placeholder
    times 80 dq isr_22
    dq isr_128   ; syscall vector 0x80

; =============================================================================
; IDT Load — called from C after IDT is filled in
; =============================================================================
section .text
idt_load:
    ; rdi = pointer to IDTR (limit:u16 + base:u64, 10 bytes)
    lidt  [rdi]
    ret
