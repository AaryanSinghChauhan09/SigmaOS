; SPDX-License-Identifier: MIT
; Copyright (c) 2024-2026 SigmaOS Project
;
; arch/x86_64/head64.asm — x86_64 kernel entry point
;
; Called by sigma-boot.efi after ExitBootServices.
; Receives a pointer to BootInfo struct in RCX (Windows/UEFI calling conv).
;
; Responsibilities:
;   1. Set up a valid GDT with kernel CS/DS/SS
;   2. Set up minimal 4-level page tables (identity map first 4 GB)
;   3. Enable NX (no-execute) bit in EFER
;   4. Switch to long mode (already in if UEFI booted 64-bit)
;   5. Set up kernel stack
;   6. Call sigma_kernel_main(boot_info*)

global _start
global sigma_kernel_entry
extern sigma_kernel_main

; ── GDT ─────────────────────────────────────────────────────────────────────
section .data
align 8

gdt64:
    dq 0                          ; 0x00 null
    dq 0x00AF9A000000FFFF         ; 0x08 kernel code  (64-bit, ring 0)
    dq 0x00CF92000000FFFF         ; 0x10 kernel data  (32/64-bit, ring 0)
    dq 0x00AFFA000000FFFF         ; 0x18 user code    (64-bit, ring 3)
    dq 0x00CFF2000000FFFF         ; 0x20 user data    (ring 3)

gdt64_ptr:
    dw $ - gdt64 - 1              ; limit
    dq gdt64                      ; base

; ── Page tables (identity-map first 4 GB) ───────────────────────────────────
section .bss
align 4096

pml4_table: resb 4096
pdp_table:  resb 4096
pd_tables:  resb 4096 * 4        ; 4 PDPT entries → 4 page directories

; ── Kernel stack ─────────────────────────────────────────────────────────────
align 16
sigma_kernel_stack_bottom:
    resb 65536                    ; 64 KB initial kernel stack
sigma_kernel_stack_top:

; ── Code ─────────────────────────────────────────────────────────────────────
section .text
bits 64

sigma_kernel_entry:
_start:
    ; Save boot_info pointer (passed in RCX by UEFI ABI)
    mov rbx, rcx

    ; Set up our own GDT (UEFI may have a volatile one)
    lgdt [gdt64_ptr]

    ; Far jump to flush segment registers with new CS
    push 0x08
    lea rax, [rel .reload_cs]
    push rax
    retfq

.reload_cs:
    mov ax, 0x10          ; kernel data segment
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Set up 4-level page tables (identity map 0..4GB using 2MB huge pages)
    call setup_page_tables
    mov cr3, rax          ; load PML4

    ; Enable NXE (no-execute) in EFER MSR
    mov ecx, 0xC0000080   ; EFER MSR
    rdmsr
    or eax, (1 << 11)     ; NXE bit
    wrmsr

    ; Enable PAE (already set by UEFI, but set it explicitly)
    mov rax, cr4
    or  rax, (1 << 5)     ; PAE
    or  rax, (1 << 7)     ; PGE (global pages)
    or  rax, (1 << 9)     ; OSFXSR (SSE)
    or  rax, (1 << 10)    ; OSXMMEXCPT
    mov cr4, rax

    ; Set up kernel stack
    lea rsp, [sigma_kernel_stack_top]
    and rsp, ~0xF          ; 16-byte align

    ; Call sigma_kernel_main(boot_info*)
    mov rdi, rbx           ; 1st arg (System V ABI) = boot_info
    xor rbp, rbp           ; clear frame pointer
    call sigma_kernel_main

    ; If kernel returns (should not happen), halt
.halt:
    cli
    hlt
    jmp .halt

; ── Setup 4-level page tables ────────────────────────────────────────────────
; Identity-maps physical 0x0 → 0x1_0000_0000 (4 GB) using 2MB pages
; Returns PML4 physical address in RAX
setup_page_tables:
    ; Clear all tables
    lea rdi, [pml4_table]
    xor eax, eax
    mov ecx, (4096 * 6) / 4
    rep stosd

    ; PML4[0] → pdp_table (present + write)
    lea rax, [pdp_table]
    or  rax, 3                    ; present + writable
    lea rbx, [pml4_table]
    mov [rbx], rax

    ; PDP[0..3] → pd_tables[0..3] (present + write)
    lea rdi, [pd_tables]
    lea rbx, [pdp_table]
    mov rcx, 4
.pdp_loop:
    lea rax, [rdi]
    or  rax, 3
    mov [rbx], rax
    add rbx, 8
    add rdi, 4096
    dec rcx
    jnz .pdp_loop

    ; Fill each PD with 512 huge (2MB) page entries
    lea rdi, [pd_tables]
    xor rdx, rdx               ; physical address counter (starts at 0)
    mov rcx, 4 * 512           ; 4 PDs × 512 entries each
.pd_loop:
    mov rax, rdx
    or  rax, (1 << 7) | 3     ; huge page + present + writable
    mov [rdi], rax
    add rdi, 8
    add rdx, 0x200000          ; advance by 2 MB
    dec rcx
    jnz .pd_loop

    lea rax, [pml4_table]
    ret
