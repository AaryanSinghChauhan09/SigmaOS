; =============================================================================
; Σ SIGMAOS: SOVEREIGN KERNEL BOOT (v1.0 - MULTIBOOT2 + LONG MODE ENTRY)
; =============================================================================
; Standard: NASM x86_64 ELF64
; Compliant: Multiboot2 (spec §3.1.6) — GRUB/UEFI-bootable
; Features:
;   - Multiboot2 magic header
;   - GDT64 bootstrap (flat 4GB code+data rings)
;   - 4-level paging identity map (first 4GB)
;   - Long mode (IA-32e) activation
;   - Stack allocation (256KB sovereign boot stack)
;   - Jump to sigma_kernel_main (C11 kernel entry)
; =============================================================================

[BITS 32]
[GLOBAL _start]
[EXTERN sigma_kernel_main]

; =============================================================================
; MULTIBOOT2 HEADER (must be in first 32KB, 8-byte aligned)
; =============================================================================
section .multiboot2
align 8
mb2_header_start:
    dd  0xE85250D6          ; Multiboot2 magic
    dd  0                   ; Architecture: i386 protected mode
    dd  mb2_header_end - mb2_header_start ; Header length
    dd  -(0xE85250D6 + 0 + (mb2_header_end - mb2_header_start)) ; Checksum
    ; End tag
    dw  0                   ; type = end
    dw  0                   ; flags
    dd  8                   ; size
mb2_header_end:

; =============================================================================
; BOOTSTRAP STACK (256KB — sovereign, below kernel image)
; =============================================================================
section .bss
align 16
stack_bottom:
    resb 256 * 1024         ; 256 KB sovereign boot stack
stack_top:

; =============================================================================
; PAGE TABLES (identity-map first 4 GB — 2MB huge pages)
; =============================================================================
align 4096
pml4_table:     resb 4096
pdp_table:      resb 4096
pd_table:       resb 4096

; =============================================================================
; GDT64 — Flat 64-bit code/data segments (no segmentation overhead)
; =============================================================================
section .data
align 8
gdt64:
    ; Null descriptor
    dq 0x0000000000000000
gdt64_code:
    ; Code segment: base=0, limit=0xFFFFF, L=1 (64-bit), DPL=0, P=1
    dq 0x00AF9A000000FFFF
gdt64_data:
    ; Data segment: base=0, limit=0xFFFFF, P=1, DPL=0, S=1, W=1
    dq 0x00CF92000000FFFF
gdt64_end:

gdt64_descriptor:
    dw gdt64_end - gdt64 - 1   ; limit
    dq gdt64                   ; base

GDT_CODE_SEL equ gdt64_code - gdt64
GDT_DATA_SEL equ gdt64_data - gdt64

; =============================================================================
; 32-BIT PROTECTED MODE ENTRY
; =============================================================================
section .text
_start:
    ; Save multiboot2 info pointer (ebx) before we clobber registers
    mov   edi, ebx          ; arg1 for sigma_kernel_main (multiboot2 info)
    mov   esi, eax          ; arg2: multiboot2 magic (0x36D76289)

    ; Set up stack
    mov   esp, stack_top

    ; Verify CPU supports CPUID
    call  check_cpuid
    ; Verify CPU supports long mode
    call  check_longmode

    ; Set up page tables for identity map (4 GB via 2MB huge pages)
    call  setup_paging

    ; Load GDT64
    lgdt  [gdt64_descriptor]

    ; Enable PAE (Physical Address Extension) — required for long mode
    mov   eax, cr4
    or    eax, (1 << 5)     ; CR4.PAE
    mov   cr4, eax

    ; Load PML4 into CR3
    mov   eax, pml4_table
    mov   cr3, eax

    ; Set IA32_EFER.LME (Long Mode Enable)
    mov   ecx, 0xC0000080   ; IA32_EFER MSR
    rdmsr
    or    eax, (1 << 8)     ; LME bit
    wrmsr

    ; Enable paging + protected mode → activates long mode
    mov   eax, cr0
    or    eax, (1 << 31) | (1 << 0)   ; PG | PE
    mov   cr0, eax

    ; Far jump to 64-bit code segment → enters IA-32e long mode
    jmp   GDT_CODE_SEL:long_mode_entry

; =============================================================================
; CPU FEATURE CHECKS
; =============================================================================
check_cpuid:
    ; Try toggling EFLAGS.ID bit to verify CPUID support
    pushfd
    pop   eax
    mov   ecx, eax
    xor   eax, (1 << 21)
    push  eax
    popfd
    pushfd
    pop   eax
    xor   eax, ecx
    jz    .no_cpuid
    ret
.no_cpuid:
    ; Halt: no CPUID support — cannot boot
    hlt

check_longmode:
    ; Check CPUID extended leaf 0x80000001 for LM bit
    mov   eax, 0x80000000
    cpuid
    cmp   eax, 0x80000001
    jb    .no_longmode
    mov   eax, 0x80000001
    cpuid
    test  edx, (1 << 29)    ; LM bit
    jz    .no_longmode
    ret
.no_longmode:
    hlt

; =============================================================================
; PAGE TABLE SETUP (Identity map 0→4GB with 2MB huge pages)
; =============================================================================
setup_paging:
    ; PML4[0] → PDP table
    mov   eax, pdp_table
    or    eax, 0x3           ; Present + Writable
    mov   [pml4_table], eax

    ; PDP[0→3] → PD tables (4 × 1GB entries → 4 PD tables)
    mov   eax, pd_table
    or    eax, 0x3
    mov   [pdp_table], eax

    ; PD[0→511]: 2MB huge pages covering 0→1GB
    mov   ecx, 0
    mov   eax, 0x83          ; Present + Writable + Huge (PS bit)
.fill_pd:
    mov   [pd_table + ecx * 8], eax
    add   eax, 0x200000      ; +2MB per entry
    inc   ecx
    cmp   ecx, 512
    jne   .fill_pd
    ret

; =============================================================================
; 64-BIT LONG MODE ENTRY
; =============================================================================
[BITS 64]
long_mode_entry:
    ; Reload segment registers with 64-bit data selector
    mov   ax, GDT_DATA_SEL
    mov   ds, ax
    mov   es, ax
    mov   fs, ax
    mov   gs, ax
    mov   ss, ax

    ; Reload stack pointer (now in 64-bit mode)
    mov   rsp, stack_top

    ; Zero BSS
    ; (handled by kernel C entry via sigma_memset)

    ; Call C kernel main: sigma_kernel_main(multiboot2_info*, magic)
    ; edi/esi contain the saved values from 32-bit entry
    call  sigma_kernel_main

    ; Should never return — halt forever
.hang:
    cli
    hlt
    jmp   .hang
