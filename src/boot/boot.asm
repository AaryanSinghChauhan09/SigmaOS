; SigmaOS Bare-Metal Assembly Bootstrap Entry
; Inspired by Linux head_64.S, FreeBSD locore.S, OpenBSD locore.s, and Multiboot2 Specification.
;
; This assembly entry point verifies Multiboot2 headers, sets up early 4-level paging (PML4),
; configures 64-bit GDT, transitions the CPU from 32-bit Protected Mode into 64-bit Long Mode,
; and passes control to the Rust kernel main entry point (`start_kernel` / `kernel_main`).

section .multiboot_header
align 8
header_start:
    ; Multiboot2 Magic Number
    dd 0xE85250D6
    ; Architecture: i386 (protected mode entry)
    dd 0
    ; Header Length
    dd header_end - header_start
    ; Checksum
    dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

    ; Optional Framebuffer Tag
    align 8
    dw 5 ; Type: Framebuffer
    dw 1 ; Flags: Optional
    dd 20
    dd 1920 ; Width
    dd 1080 ; Height
    dd 32   ; Depth

    ; End tag
    align 8
    dw 0
    dw 0
    dd 8
header_end:

section .bss
align 4096
pml4_table:
    resb 4096
pdp_table:
    resb 4096
pd_table:
    resb 4096

align 16
stack_bottom:
    resb 65536 ; 64 KB initial kernel stack
stack_top:

section .rodata
align 8
gdt64:
    dq 0 ; Null Descriptor
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; 64-bit Code Segment (Kernel)
.data: equ $ - gdt64
    dq (1<<41) | (1<<44) | (1<<47) ; 64-bit Data Segment (Kernel)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

section .text
bits 32
global _start
extern start_kernel ; Defined in src/kernel/main.rs

_start:
    cli ; Disable interrupts

    ; Set up initial 32-bit stack
    mov esp, stack_top

    ; Preserve Multiboot info registers
    mov edi, ebx ; Multiboot2 info structure pointer
    mov esi, eax ; Multiboot2 magic number (0x36d76289)

    ; 1. Verify Multiboot2 magic number
    cmp eax, 0x36d76289
    jne .no_multiboot

    ; 2. Set up Identity Page Tables (PML4 -> PDP -> PD 1GB identity map)
    mov eax, pdp_table
    or eax, 0b11 ; present + writable
    mov [pml4_table], eax

    mov eax, pd_table
    or eax, 0b11 ; present + writable
    mov [pdp_table], eax

    ; Identity map first 1GB using 2MB huge pages
    mov ecx, 0
.map_pd:
    mov eax, 0x200000 ; 2MB
    mul ecx
    or eax, 0b10000011 ; present + writable + huge page (2MB)
    mov [pd_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .map_pd

    ; 3. Enable PAE (Physical Address Extension) in CR4
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; 4. Load PML4 into CR3
    mov eax, pml4_table
    mov cr3, eax

    ; 5. Enable Long Mode in EFER MSR (0xC0000080)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8 ; LME (Long Mode Enable)
    wrmsr

    ; 6. Enable Paging & Protected Mode in CR0
    mov eax, cr0
    or eax, (1 << 31) | (1 << 0) ; PG + PE
    mov cr0, eax

    ; 7. Load 64-bit Global Descriptor Table
    lgdt [gdt64.pointer]

    ; 8. Far jump into 64-bit Long Mode
    jmp gdt64.code:long_mode_start

.no_multiboot:
    mov al, 'M'
    jmp .error

.error:
    mov dword [0xb8000], 0x4f524f45 ; Red 'ERR:' on VGA
    hlt

bits 64
long_mode_start:
    ; Reset segment selectors
    mov ax, gdt64.data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Reload 64-bit stack
    mov rsp, stack_top

    ; Pass Multiboot arguments to Rust kernel in RDI & RSI
    ; rdi = multiboot_info_ptr
    ; rsi = multiboot_magic
    call start_kernel

.hang:
    cli
    hlt
    jmp .hang
