; =========================================================================
; Σ SIGMAOS: INTERRUPT SERVICE ROUTINE STUBS (Phase 16)
; =========================================================================
; Pure NASM x86_64 interrupt entry points.
; Each ISR stub pushes a uniform stack frame and calls into the C handler
; sigma_isr_dispatch(sigma_isr_frame_t*).
;
; Design:
;   - ISRs 0-7:    CPU exceptions (some push error code, some don't)
;   - ISRs 8-31:   CPU exceptions (double fault, GPF, page fault, etc.)
;   - ISRs 32-47:  PIC IRQs (timer, keyboard, etc.)
;   - ISRs 48-255: Software interrupts / future use
;
; The C handler signature:
;   extern void sigma_isr_dispatch(sigma_isr_frame_t* frame);
;
; Stack frame layout (matches sigma_isr_frame_t in sigma_context.h):
;   [rsp+0]   r15
;   [rsp+8]   r14
;   [rsp+16]  r13
;   [rsp+24]  r12
;   [rsp+32]  r11
;   [rsp+40]  r10
;   [rsp+48]  r9
;   [rsp+56]  r8
;   [rsp+64]  rbp
;   [rsp+72]  rdi
;   [rsp+80]  rsi
;   [rsp+88]  rdx
;   [rsp+96]  rcx
;   [rsp+104] rbx
;   [rsp+112] rax
;   [rsp+120] int_no         (pushed by stub)
;   [rsp+128] err_code       (pushed by CPU or stub)
;   [rsp+136] rip            (pushed by CPU on interrupt)
;   [rsp+144] cs
;   [rsp+152] rflags
;   [rsp+160] rsp            (user RSP, if privilege change)
;   [rsp+168] ss
; =========================================================================

[BITS 64]

section .text

extern sigma_isr_dispatch

; =========================================================================
; MACRO: ISR stub that pushes a dummy error code (for exceptions that
; don't push one automatically)
; =========================================================================
%macro ISR_NOERRCODE 1
global sigma_isr_%1
sigma_isr_%1:
    push qword 0          ; Dummy error code
    push qword %1         ; Interrupt number
    jmp  isr_common_stub
%endmacro

; =========================================================================
; MACRO: ISR stub for exceptions that push a real error code
; (Double Fault, Invalid TSS, Segment Not Present, Stack Fault, GPF,
;  Page Fault, Alignment Check, Security Exception)
; =========================================================================
%macro ISR_ERRCODE 1
global sigma_isr_%1
sigma_isr_%1:
    push qword %1         ; Interrupt number (error code already on stack)
    jmp  isr_common_stub
%endmacro

; =========================================================================
; Common ISR handler — saves all registers, calls C, restores, iretq
; =========================================================================
isr_common_stub:
    ; Save all general-purpose registers (callee + caller saved)
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    ; Pass pointer to the stack frame as argument (RDI = first arg, SysV ABI)
    mov  rdi, rsp
    
    ; Ensure 16-byte stack alignment before call (required by SysV ABI)
    ; We've pushed 15 registers (15*8=120) + int_no (8) + err_code (8) = 136
    ; Plus CPU-pushed RIP+CS+RFLAGS+RSP+SS = 40. Total = 176 = 16*11 → aligned.
    
    call sigma_isr_dispatch

    ; Restore all registers
    pop  r15
    pop  r14
    pop  r13
    pop  r12
    pop  r11
    pop  r10
    pop  r9
    pop  r8
    pop  rbp
    pop  rdi
    pop  rsi
    pop  rdx
    pop  rcx
    pop  rbx
    pop  rax

    ; Remove int_no and err_code from stack
    add  rsp, 16

    ; Return from interrupt
    iretq

; =========================================================================
; ISR entries 0–31: CPU Exceptions
; =========================================================================
ISR_NOERRCODE 0    ; Division Error
ISR_NOERRCODE 1    ; Debug
ISR_NOERRCODE 2    ; Non-Maskable Interrupt
ISR_NOERRCODE 3    ; Breakpoint
ISR_NOERRCODE 4    ; Overflow
ISR_NOERRCODE 5    ; Bound Range Exceeded
ISR_NOERRCODE 6    ; Invalid Opcode
ISR_NOERRCODE 7    ; Device Not Available (FPU)
ISR_ERRCODE   8    ; Double Fault
ISR_NOERRCODE 9    ; Coprocessor Segment Overrun (legacy)
ISR_ERRCODE   10   ; Invalid TSS
ISR_ERRCODE   11   ; Segment Not Present
ISR_ERRCODE   12   ; Stack-Segment Fault
ISR_ERRCODE   13   ; General Protection Fault
ISR_ERRCODE   14   ; Page Fault
ISR_NOERRCODE 15   ; Reserved
ISR_NOERRCODE 16   ; x87 FPU Error
ISR_ERRCODE   17   ; Alignment Check
ISR_NOERRCODE 18   ; Machine Check
ISR_NOERRCODE 19   ; SIMD Floating-Point Exception
ISR_NOERRCODE 20   ; Virtualization Exception
ISR_ERRCODE   21   ; Control Protection Exception
ISR_NOERRCODE 22   ; Reserved
ISR_NOERRCODE 23   ; Reserved
ISR_NOERRCODE 24   ; Reserved
ISR_NOERRCODE 25   ; Reserved
ISR_NOERRCODE 26   ; Reserved
ISR_NOERRCODE 27   ; Reserved
ISR_NOERRCODE 28   ; Reserved
ISR_NOERRCODE 29   ; Reserved
ISR_ERRCODE   30   ; Security Exception
ISR_NOERRCODE 31   ; Reserved

; =========================================================================
; ISR entries 32–47: PIC Hardware IRQs (remapped from 32)
; =========================================================================
%assign i 32
%rep 16
ISR_NOERRCODE i
%assign i i+1
%endrep

; =========================================================================
; ISR entries 48–255: Software Interrupts / Syscalls / Future Use
; =========================================================================
%assign i 48
%rep 208
ISR_NOERRCODE i
%assign i i+1
%endrep

; =========================================================================
; ISR Table: Array of 256 function pointers for IDT setup from C
; =========================================================================
section .data
global sigma_isr_table
sigma_isr_table:
%assign i 0
%rep 256
    dq sigma_isr_%+i
%assign i i+1
%endrep
