; =========================================================================
; SigmaOS Sovereign x86_64 Long-Mode Setup & GDT Initialization
; =========================================================================
; Purpose: After Multiboot2 hands control to the 32-bit stub (boot.asm),
;          this code enables PAE, sets up page tables for identity-mapping,
;          transitions the CPU to 64-bit Long Mode, and jumps to the
;          Rust/C kernel entry point.
;
; IP Compliance: 100% original. Based on Intel SDM public specification.
; References: Intel® 64 and IA-32 Architectures Software Developer Manuals
;             (publicly available at intel.com — no copyrighted code copied).
; =========================================================================

[BITS 32]
global long_mode_start
global _setup_long_mode
extern sigma64_entry            ; 64-bit C/Rust kernel entry

; ── CONSTANTS ──
PAGE_PRESENT   equ 0x1
PAGE_WRITABLE  equ 0x2
PAGE_HUGE      equ 0x80
CR0_PE         equ 0x1
CR0_PG         equ 0x80000000
CR4_PAE        equ 0x20
EFER_MSR       equ 0xC0000080
EFER_LME       equ 0x100        ; Long Mode Enable bit

; ── BSS: Page Table Storage (aligned to 4KB page boundary) ──
section .bss
align 4096
pml4_table:  resb 4096          ; Page Map Level 4
pdpt_table:  resb 4096          ; Page Directory Pointer Table
pd_table:    resb 4096          ; Page Directory (2MB huge pages)

; ── 64-bit GDT ──
section .data
align 8
gdt64:
    ; Null descriptor
    dq 0x0000000000000000
    ; 64-bit Code Segment: DPL=0, Present, L=1 (64-bit), Execute/Read
    dq 0x00AF9A000000FFFF
    ; 64-bit Data Segment: DPL=0, Present, Read/Write
    dq 0x00CF92000000FFFF
gdt64_end:

gdt64_descriptor:
    dw gdt64_end - gdt64 - 1   ; Limit
    dd gdt64                    ; Base (32-bit for now)

CODE_SEG equ 0x08
DATA_SEG equ 0x10

; ── Stack for 64-bit kernel ──
section .bss
align 16
kernel64_stack_bottom:
    resb 65536                  ; 64 KB kernel stack
kernel64_stack_top:

; ── TEXT SECTION ──
section .text
[BITS 32]

_setup_long_mode:
    ; ── Step 1: Zero the page tables ──
    mov edi, pml4_table
    mov ecx, (3 * 4096) / 4    ; 3 tables × 4096 bytes / 4-byte words
    xor eax, eax
    rep stosd

    ; ── Step 2: Wire PML4 → PDPT → PD ──
    mov eax, pdpt_table
    or  eax, (PAGE_PRESENT | PAGE_WRITABLE)
    mov [pml4_table], eax

    mov eax, pd_table
    or  eax, (PAGE_PRESENT | PAGE_WRITABLE)
    mov [pdpt_table], eax

    ; ── Step 3: Map first 1GB with 2MB huge pages ──
    ; Each entry covers 2MB; 512 entries = 1 GB identity-mapped
    mov ecx, 0                  ; Page index counter
.map_huge_pages:
    mov eax, 0x200000           ; 2MB
    mul ecx
    or  eax, (PAGE_PRESENT | PAGE_WRITABLE | PAGE_HUGE)
    mov [pd_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jl  .map_huge_pages

    ; ── Step 4: Load PML4 into CR3 ──
    mov eax, pml4_table
    mov cr3, eax

    ; ── Step 5: Enable PAE (Physical Address Extension) ──
    mov eax, cr4
    or  eax, CR4_PAE
    mov cr4, eax

    ; ── Step 6: Set EFER.LME (Long Mode Enable via MSR) ──
    mov ecx, EFER_MSR
    rdmsr
    or  eax, EFER_LME
    wrmsr

    ; ── Step 7: Enable Paging + Protected Mode → triggers Long Mode ──
    mov eax, cr0
    or  eax, (CR0_PE | CR0_PG)
    mov cr0, eax

    ; ── Step 8: Far jump to 64-bit code segment to flush pipeline ──
    lgdt [gdt64_descriptor]
    jmp  CODE_SEG:long_mode_start

; ════════════════════════════════════════════════
; 64-bit Long Mode Entry Point
; ════════════════════════════════════════════════
[BITS 64]
long_mode_start:
    ; Reload data segment registers with 64-bit descriptor
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Set up the full 64-bit kernel stack
    mov rsp, kernel64_stack_top

    ; Clear the interrupt flag until IDT is armed
    cli

    ; Jump into the C/Rust kernel. RSP is aligned. We are in Long Mode.
    call sigma64_entry

    ; Should never reach here — halt the processor
.halt:
    cli
    hlt
    jmp .halt
