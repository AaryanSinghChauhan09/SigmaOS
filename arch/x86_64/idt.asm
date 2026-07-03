; SPDX-License-Identifier: MIT
; arch/x86_64/idt.asm — IDT (Interrupt Descriptor Table) + IRQ stubs

global sigma_idt_init
global sigma_idt_set_handler
global sigma_idt_ptr

extern sigma_exception_handler     ; Rust: kernel/core/sigma_irq.rs
extern sigma_irq_dispatch           ; Rust: kernel/core/sigma_irq.rs

section .data
align 8

; ── IDT (256 entries × 16 bytes each) ────────────────────────────────────────
sigma_idt:
    times 256 dq 0
    times 256 dq 0         ; 256 × 16-byte entries = 4096 bytes

sigma_idt_ptr:
    dw 256 * 16 - 1        ; limit
    dq sigma_idt           ; base

; ── Exception names (for debug output) ───────────────────────────────────────
section .rodata
exc_names:
    dq exc0, exc1, exc2, exc3, exc4, exc5, exc6, exc7
    dq exc8, exc9, exc10, exc11, exc12, exc13, exc14, exc15
    dq exc16, exc17, exc18, exc19, exc20, exc21, exc22, exc23
    dq exc24, exc25, exc26, exc27, exc28, exc29, exc30, exc31
exc0:  db "Divide Error",0        ; #DE
exc1:  db "Debug",0               ; #DB
exc2:  db "NMI",0                 ; NMI
exc3:  db "Breakpoint",0          ; #BP
exc4:  db "Overflow",0            ; #OF
exc5:  db "Bound Range",0         ; #BR
exc6:  db "Invalid Opcode",0      ; #UD
exc7:  db "Device Not Avail",0    ; #NM
exc8:  db "Double Fault",0        ; #DF
exc9:  db "Coproc Seg Overrun",0
exc10: db "Invalid TSS",0         ; #TS
exc11: db "Segment Not Present",0 ; #NP
exc12: db "Stack Fault",0         ; #SS
exc13: db "General Protection",0  ; #GP
exc14: db "Page Fault",0          ; #PF
exc15: db "Reserved",0
exc16: db "x87 FP Error",0        ; #MF
exc17: db "Alignment Check",0     ; #AC
exc18: db "Machine Check",0       ; #MC
exc19: db "SIMD FP Error",0       ; #XF
exc20: db "Virtualization",0
exc21: db "Control Protection",0
exc22: db "Reserved",0
exc23: db "Reserved",0
exc24: db "Reserved",0
exc25: db "Reserved",0
exc26: db "Reserved",0
exc27: db "Reserved",0
exc28: db "Hypervisor Injection",0
exc29: db "VMM Communication",0
exc30: db "Security Exception",0
exc31: db "Reserved",0

section .text
bits 64

; ── IDT entry format ─────────────────────────────────────────────────────────
; Offset[15:0]  | Selector | IST | Type | Offset[31:16]
; Offset[63:32] | Reserved

; sigma_idt_set_handler(vector: u8, handler: *fn, ist: u8, dpl: u8)
; RDI=vector, RSI=handler, RDX=ist, RCX=dpl
sigma_idt_set_handler:
    movzx rax, dil                ; vector
    lea rbx, [sigma_idt]
    shl rax, 4                    ; each entry = 16 bytes
    add rbx, rax

    ; Build IDT entry
    ; Word 0: handler[15:0]
    mov rax, rsi
    and rax, 0xFFFF
    ; Word 1: CS selector = 0x08
    or  rax, (0x08 << 16)
    ; Byte 4: IST (bits 2:0)
    movzx r8, dl
    and r8, 7
    shl r8, 32
    or  rax, r8
    ; Byte 5: type+dpl+present
    ;   type = 0xE (64-bit interrupt gate) or 0xF (trap gate)
    ;   dpl = 0 (kernel) or 3 (user)
    movzx r8, cl
    and r8, 3
    shl r8, 5
    or  r8, 0x8E                  ; present=1, 64-bit interrupt gate
    shl r8, 40
    or  rax, r8
    ; Word 3: handler[31:16]
    mov r8, rsi
    shr r8, 16
    and r8, 0xFFFF
    shl r8, 48
    or  rax, r8
    mov [rbx], rax                ; low 8 bytes

    ; High 8 bytes: handler[63:32]
    mov rax, rsi
    shr rax, 32
    mov [rbx + 8], rax

    ret

; sigma_idt_init() — set up all 256 IDT entries and load IDT
sigma_idt_init:
    ; Install exception stubs (vectors 0..31)
%assign i 0
%rep 32
    mov rdi, i
    lea rsi, [exc_stub_ %+ i]
    xor rdx, rdx                  ; IST=0
    xor rcx, rcx                  ; DPL=0
    call sigma_idt_set_handler
%assign i i+1
%endrep

    ; Install IRQ stubs (vectors 32..47, mapped from PIC IRQ0..IRQ15)
%assign i 32
%rep 16
    mov rdi, i
    lea rsi, [irq_stub_ %+ i]
    xor rdx, rdx
    xor rcx, rcx
    call sigma_idt_set_handler
%assign i i+1
%endrep

    ; Install sigma-bus syscall gate (vector 0x80)
    mov rdi, 0x80
    lea rsi, [syscall_stub]
    xor rdx, rdx
    mov rcx, 3                    ; DPL=3, callable from ring 3
    call sigma_idt_set_handler

    ; Load IDT
    lidt [sigma_idt_ptr]
    ret

; ── Exception stubs ───────────────────────────────────────────────────────────
; Push vector number, call common handler
%macro exc_stub_noerr 1
exc_stub_ %+ %1:
    push qword 0      ; dummy error code
    push qword %1     ; vector
    jmp exc_common
%endmacro

%macro exc_stub_err 1
exc_stub_ %+ %1:
    push qword %1     ; vector (error code already on stack)
    jmp exc_common
%endmacro

; Exceptions without error code
exc_stub_noerr 0
exc_stub_noerr 1
exc_stub_noerr 2
exc_stub_noerr 3
exc_stub_noerr 4
exc_stub_noerr 5
exc_stub_noerr 6
exc_stub_noerr 7
exc_stub_err   8
exc_stub_noerr 9
exc_stub_err   10
exc_stub_err   11
exc_stub_err   12
exc_stub_err   13
exc_stub_err   14
exc_stub_noerr 15
exc_stub_noerr 16
exc_stub_err   17
exc_stub_noerr 18
exc_stub_noerr 19
exc_stub_noerr 20
exc_stub_noerr 21
%assign i 22
%rep 10
exc_stub_noerr i
%assign i i+1
%endrep

exc_common:
    ; Stack: error_code, vector, RIP, CS, RFLAGS, RSP, SS
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

    mov rdi, rsp          ; arg: pointer to saved regs + error info
    call sigma_exception_handler

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    add rsp, 16           ; pop vector + error code
    iretq

; ── IRQ stubs ─────────────────────────────────────────────────────────────────
%macro irq_stub 1
irq_stub_ %+ %1:
    push qword 0          ; dummy error code
    push qword %1         ; IRQ number
    jmp irq_common
%endmacro

%assign i 32
%rep 16
irq_stub i
%assign i i+1
%endrep

irq_common:
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11

    mov rdi, [rsp + 11*8]  ; IRQ number
    call sigma_irq_dispatch

    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    add rsp, 16            ; pop irq number + dummy error code
    iretq

; ── Syscall gate (int 0x80) ───────────────────────────────────────────────────
syscall_stub:
    push qword 0
    push qword 0x80
    ; save caller regs
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11

    ; Call sigma_syscall_dispatch(nr, a1..a6)
    ; Linux ABI: RAX=nr, RDI=a1, RSI=a2, RDX=a3, R10=a4, R8=a5, R9=a6
    extern sigma_syscall_dispatch
    mov rdi, rax          ; nr
    ; a1..a6 already in rdi,rsi,rdx,r10,r8,r9 (shifted by mov rdi,rax above)
    ; re-load from saved stack
    mov rdi, rax           ; syscall number
    mov rsi, [rsp + 6*8]   ; original rdi = a1
    mov rdx, [rsp + 4*8]   ; original rsi = a2 -- simplified mapping
    xor rcx, rcx
    xor r8,  r8
    xor r9,  r9
    call sigma_syscall_dispatch
    mov [rsp + 9*8], rax   ; store return value in saved RAX slot

    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    add rsp, 16
    iretq
