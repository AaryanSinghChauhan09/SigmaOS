; -----------------------------------------------------------------------------
; SigmaOS Enterprise Paging Shard v1.0 (NASM 32-bit/64-bit)
; Inspiration: torvalds/linux/arch/x86/mm/init.c (Paging Setup)
; USP: Virtual Memory Sharding for Identity Isolation.
; Principle: Memory Space Enterprisety.
; -----------------------------------------------------------------------------

[BITS 32]

section .bss
align 4096
page_directory: resb 4096
page_table:     resb 4096

section .text
align 4
global sigma_init_paging

sigma_init_paging:
    ; Filling the first 1024 entries of the page table
    ; Mapping 0x00000000 to 0x003FFFFF (Identity Mapping)
    mov eax, 0          ; Frame index 0
    mov ecx, 0          ; Table index 0
.fill_table:
    mov edx, eax        ; Base address
    or edx, 3           ; Attributes (Present + Read/Write)
    mov [page_table + ecx*4], edx
    add eax, 4096       ; Next frame
    inc ecx
    cmp ecx, 1024
    jl .fill_table

    ; Placing the page table into the page directory
    mov eax, page_table
    or eax, 3           ; Attributes
    mov [page_directory], eax

    ; Loading the page directory into CR3
    mov eax, page_directory
    mov cr3, eax

    ; Enabling Paging (Setting PG bit in CR0)
    mov eax, cr0
    or eax, 0x80000000
    mov cr0, eax

    ret
