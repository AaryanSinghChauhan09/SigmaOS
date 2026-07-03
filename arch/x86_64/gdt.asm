; SPDX-License-Identifier: MIT
; arch/x86_64/gdt.asm — Global Descriptor Table + TSS setup

global sigma_gdt_init
global sigma_gdt_ptr
global sigma_tss

section .data
align 8

; ── Segment selectors ────────────────────────────────────────────────────────
GDT_KERNEL_CODE equ 0x08
GDT_KERNEL_DATA equ 0x10
GDT_USER_CODE   equ 0x18 | 3   ; ring 3
GDT_USER_DATA   equ 0x20 | 3   ; ring 3
GDT_TSS         equ 0x28

; ── GDT entries ──────────────────────────────────────────────────────────────
sigma_gdt_table:
.null:    dq 0                             ; 0x00 null
.kcode:   dq 0x00AF9A000000FFFF           ; 0x08 64-bit kernel code
.kdata:   dq 0x00CF92000000FFFF           ; 0x10 kernel data
.ucode:   dq 0x00AFFA000000FFFF           ; 0x18 64-bit user code
.udata:   dq 0x00CFF2000000FFFF           ; 0x20 user data
.tss_lo:  dq 0                            ; 0x28 TSS low  (filled at runtime)
.tss_hi:  dq 0                            ; 0x30 TSS high (filled at runtime)
.end:

sigma_gdt_ptr:
    dw sigma_gdt_table.end - sigma_gdt_table - 1
    dq sigma_gdt_table

; ── Task State Segment ────────────────────────────────────────────────────────
section .bss
align 16
sigma_tss:
    dd 0              ; reserved
    dq 0              ; RSP0 (kernel stack when entering ring 0)
    dq 0              ; RSP1
    dq 0              ; RSP2
    dq 0              ; reserved
    times 7 dq 0      ; IST1..IST7
    dq 0              ; reserved
    dw 0              ; reserved
    dw $ - sigma_tss  ; IO permission bitmap base

; ── GDT + TSS load ────────────────────────────────────────────────────────────
section .text
bits 64

; sigma_gdt_init(rsp0: u64) — install GDT + TSS, set RSP0 for syscall entry
sigma_gdt_init:
    ; Arg: RDI = kernel stack pointer (RSP0 for ring-0 entry)

    ; Fill TSS descriptor in GDT
    lea rax, [sigma_tss]
    lea rbx, [sigma_gdt_table + 0x28]  ; TSS lo entry

    ; TSS base = RAX, limit = sizeof TSS - 1
    movzx rcx, ax
    shl rcx, 16
    or  rcx, (sigma_tss.end - sigma_tss - 1)  ; limit[15:0]
    or  rcx, (0x89 << 40)                      ; type=0x9 (available TSS), P=1
    ; insert base[23:16]
    mov rdx, rax
    shr rdx, 16
    and rdx, 0xFF
    shl rdx, 32
    or  rcx, rdx
    ; insert base[31:24]
    mov rdx, rax
    shr rdx, 24
    and rdx, 0xFF
    shl rdx, 56
    or  rcx, rdx
    mov [rbx], rcx

    ; TSS hi: base[63:32]
    mov rdx, rax
    shr rdx, 32
    mov [rbx + 8], rdx

    ; Set RSP0 in TSS (kernel stack pointer for ring-0 entry)
    lea rax, [sigma_tss]
    mov [rax + 4], rdi    ; TSS.RSP0 = kernel stack

    ; Load GDT
    lgdt [sigma_gdt_ptr]

    ; Reload CS via far return
    push GDT_KERNEL_CODE
    lea rax, [rel .reload]
    push rax
    retfq
.reload:
    mov ax, GDT_KERNEL_DATA
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Load TSS
    mov ax, GDT_TSS
    ltr ax

    ret

sigma_tss.end:
