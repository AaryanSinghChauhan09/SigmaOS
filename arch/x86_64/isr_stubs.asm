; SPDX-License-Identifier: MIT
; arch/x86_64/isr_stubs.asm
;
; SigmaOS ISR entry stubs for CPU exception vectors 0–31.
; Each stub pushes a uniform stack frame (error code + vector) and jumps
; to common_isr_stub, which saves all GPRs and calls sigma_isr_dispatch().
;
; Vectors that push an error code automatically:
;   8 (DF), 10 (#TS), 11 (#NP), 12 (#SS), 13 (#GP), 14 (#PF),
;   17 (#AC), 21 (#CP), 30 (#SX)
; All others get a dummy zero pushed first.
;
; Stack layout on entry to common_isr_stub (matches sigma_isr_frame_t):
;   [ss, rsp, rflags, cs, rip]  — pushed by CPU on interrupt
;   [err_code]                  — pushed by CPU or dummy by stub
;   [vector]                    — pushed by stub
;   [rax … r15]                 — saved by common_isr_stub
;
; Exported symbols: sigma_isr_0 … sigma_isr_31
; Also exports: sigma_isr_table (array of 32 function pointers)

[BITS 64]

section .text

extern sigma_isr_dispatch          ; Rust: kernel/core/sigma_irq.rs (or C shim)

; ── Macro: stub without hardware error code ──────────────────────────────────
%macro ISR_NOERRCODE 1
global sigma_isr_%1
sigma_isr_%1:
    push qword 0          ; dummy error code
    push qword %1         ; vector number
    jmp  common_isr_stub
%endmacro

; ── Macro: stub with hardware error code (already on stack) ──────────────────
%macro ISR_ERRCODE 1
global sigma_isr_%1
sigma_isr_%1:
    push qword %1         ; vector number (error code already pushed by CPU)
    jmp  common_isr_stub
%endmacro

; ── Common ISR stub ───────────────────────────────────────────────────────────
; Saves all GPRs, calls sigma_isr_dispatch(frame*), restores, iretq.
common_isr_stub:
    ; Push GPRs in order matching sigma_isr_frame_t
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

    ; RDI = pointer to saved frame (SysV AMD64 first arg)
    mov  rdi, rsp
    call sigma_isr_dispatch

    ; Restore GPRs
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

    add  rsp, 16          ; discard vector + error code
    iretq

; ── Exception stubs: vectors 0–31 ────────────────────────────────────────────
; Vectors with hardware error code: 8, 10, 11, 12, 13, 14, 17, 21, 30
ISR_NOERRCODE  0   ; #DE  Divide Error
ISR_NOERRCODE  1   ; #DB  Debug
ISR_NOERRCODE  2   ;      NMI
ISR_NOERRCODE  3   ; #BP  Breakpoint
ISR_NOERRCODE  4   ; #OF  Overflow
ISR_NOERRCODE  5   ; #BR  Bound Range Exceeded
ISR_NOERRCODE  6   ; #UD  Invalid Opcode
ISR_NOERRCODE  7   ; #NM  Device Not Available
ISR_ERRCODE    8   ; #DF  Double Fault          (error code = 0)
ISR_NOERRCODE  9   ;      Coprocessor Segment Overrun (legacy)
ISR_ERRCODE   10   ; #TS  Invalid TSS
ISR_ERRCODE   11   ; #NP  Segment Not Present
ISR_ERRCODE   12   ; #SS  Stack-Segment Fault
ISR_ERRCODE   13   ; #GP  General Protection Fault
ISR_ERRCODE   14   ; #PF  Page Fault
ISR_NOERRCODE 15   ;      Reserved
ISR_NOERRCODE 16   ; #MF  x87 FPU Floating-Point Error
ISR_ERRCODE   17   ; #AC  Alignment Check
ISR_NOERRCODE 18   ; #MC  Machine Check
ISR_NOERRCODE 19   ; #XF  SIMD Floating-Point Exception
ISR_NOERRCODE 20   ; #VE  Virtualization Exception
ISR_ERRCODE   21   ; #CP  Control Protection Exception
ISR_NOERRCODE 22   ;      Reserved
ISR_NOERRCODE 23   ;      Reserved
ISR_NOERRCODE 24   ;      Reserved
ISR_NOERRCODE 25   ;      Reserved
ISR_NOERRCODE 26   ;      Reserved
ISR_NOERRCODE 27   ;      Reserved
ISR_NOERRCODE 28   ; #HV  Hypervisor Injection Exception
ISR_NOERRCODE 29   ; #VC  VMM Communication Exception
ISR_ERRCODE   30   ; #SX  Security Exception
ISR_NOERRCODE 31   ;      Reserved

; ── ISR function pointer table (32 entries, for sigma_idt_init) ──────────────
section .data
global sigma_isr_table
sigma_isr_table:
%assign v 0
%rep 32
    dq sigma_isr_ %+ v
%assign v v+1
%endrep
