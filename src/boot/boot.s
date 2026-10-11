/*
 * SigmaOS Bare-Metal x86_64 Assembly Bootstrap Entry (boot.s)
 * Multiboot2 Compliant Header & Protected Mode (32-bit) -> Long Mode (64-bit) Transition
 *
 * Inspired by Linux boot/header.S, xv6-x86_64 boot, and Redox OS bootloader.
 */

.code32
.section .multiboot_header, "a"
.align 8
header_start:
    /* Magic number (Multiboot2 specification) */
    .long 0xE85250D6
    /* Architecture: 0 = i386 (protected mode entry point) */
    .long 0
    /* Header length */
    .long header_end - header_start
    /* Checksum */
    .long 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

    /* Multiboot2 Tag: Framebuffer Request */
    .align 8
    .word 5         /* Type: Framebuffer request */
    .word 0         /* Flags: Optional */
    .long 20        /* Size: 20 bytes */
    .long 1920      /* Preferred Width */
    .long 1080      /* Preferred Height */
    .long 32        /* Preferred Depth (bpp) */

    /* Multiboot2 Tag: Console Flags */
    .align 8
    .word 4         /* Type: Console flags */
    .word 0         /* Flags */
    .long 12        /* Size: 12 bytes */
    .long 3         /* Console flags: EGA text + Graphical console */

    /* Multiboot2 Tag: End Tag */
    .align 8
    .word 0         /* Type: End */
    .word 0         /* Flags */
    .long 8         /* Size: 8 bytes */
header_end:

/* BSS Section for Stack & Initial Page Tables */
.section .bss
.align 4096
.global stack_bottom
.global stack_top
stack_bottom:
    .skip 16384     /* 16 KiB initial bootstrap stack */
stack_top:

/* 4-Level Paging Structures (Identity & Higher-Half Direct Map) */
.align 4096
pml4_table:
    .skip 4096
pdpt_table:
    .skip 4096
pd_table:
    .skip 4096

.section .text
.global _start
.global start
.type _start, @function

_start:
start:
    /* Disable interrupts during low-level setup */
    cli

    /* Initialize stack pointer */
    mov $stack_top, %esp

    /* Save Multiboot2 bootloader parameters passed in registers according to System V ABI:
     * %eax = Multiboot2 Magic (0x36d76289) -> %esi (2nd arg)
     * %ebx = Multiboot2 Information Structure Pointer (physical) -> %edi (1st arg)
     */
    mov %ebx, %edi   /* %edi = 1st arg: Multiboot Info Struct Address */
    mov %eax, %esi   /* %esi = 2nd arg: Multiboot Magic Number */

    /* Step 1: Verify CPU support for x86_64 Long Mode via CPUID */
    call check_cpuid
    call check_long_mode

    /* Step 2: Set up 4-Level Paging (PML4 -> PDPT -> PD) */
    call setup_page_tables

    /* Step 3: Enable PAE (Physical Address Extension) in CR4 */
    mov %cr4, %eax
    or $(1 << 5), %eax    /* Set PAE bit (bit 5) */
    mov %eax, %cr4

    /* Step 4: Enable Long Mode in EFER MSR (0xC0000080) */
    mov $0xC0000080, %ecx
    rdmsr
    or $(1 << 8), %eax    /* Set LME bit (bit 8) */
    wrmsr

    /* Step 5: Enable Paging (PG) and Protection (PE) in CR0 */
    mov $pml4_table, %eax
    mov %eax, %cr3       /* Load PML4 base into CR3 */

    mov %cr0, %eax
    or $(1 << 31 | 1 << 0), %eax  /* Set PG (bit 31) & PE (bit 0) */
    mov %eax, %cr0

    /* Step 6: Load 64-bit Global Descriptor Table (GDT) */
    lgdt gdt64_pointer

    /* Step 7: Far jump to transition into 64-bit Long Mode code segment */
    ljmp $0x08, $long_mode_entry

.code64
long_mode_entry:
    /* Step 8: Update 64-bit Segment Selector Registers */
    mov $0x10, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs
    mov %ax, %ss

    /* %rdi already holds Multiboot Info Struct Address (zero-extended from %edi)
     * %rsi already holds Multiboot Magic (zero-extended from %esi)
     * Arguments match System V ABI: kernel_main(multiboot_info: u64, magic: u64)
     */

    /* Step 9: Call Rust Kernel Main Entry Point */
    extern kernel_main
    call kernel_main

    /* Step 10: Infinite Halt Loop if kernel_main ever returns */
.hang:
    cli
    hlt
    jmp .hang

/* Helper Assembly Subroutines */
.code32
check_cpuid:
    /* Check if CPUID flag (bit 21 in EFLAGS) can be toggled */
    pushfl
    pop %eax
    mov %eax, %ecx
    xor $(1 << 21), %eax
    push %eax
    popfl
    pushfl
    pop %eax
    push %ecx
    popfl
    cmp %ecx, %eax
    je .no_cpuid
    ret
.no_cpuid:
    mov $0x45525230, %eax /* "ERR0": No CPUID */
    jmp .boot_error

check_long_mode:
    /* Query extended CPUID functions (0x80000000) */
    mov $0x80000000, %eax
    cpuid
    cmp $0x80000001, %eax
    jb .no_long_mode

    /* Query extended feature flags (0x80000001) */
    mov $0x80000001, %eax
    cpuid
    test $(1 << 29), %edx /* Check LM bit (bit 29) in EDX */
    jz .no_long_mode
    ret
.no_long_mode:
    mov $0x45525231, %eax /* "ERR1": No 64-bit Long Mode */
    jmp .boot_error

setup_page_tables:
    /* Map first entry of PML4 to PDPT */
    mov $pdpt_table, %eax
    or $0x3, %eax        /* Present + Writable (flags = 0b011) */
    mov %eax, pml4_table
    movl $0, pml4_table + 4

    /* Also map higher-half entry (entry 256 & 511) to PDPT for higher-half kernel */
    mov %eax, pml4_table + (256 * 8)
    movl $0, pml4_table + (256 * 8) + 4
    mov %eax, pml4_table + (511 * 8)
    movl $0, pml4_table + (511 * 8) + 4

    /* Map first entry of PDPT to PD */
    mov $pd_table, %eax
    or $0x3, %eax        /* Present + Writable */
    mov %eax, pdpt_table
    movl $0, pdpt_table + 4

    /* Identity map 1 GB (512 x 2 MB Huge Pages) in PD Table */
    mov $0, %ecx         /* Counter index: 0..512 */
.map_pd_loop:
    mov $0x200000, %eax  /* 2 MB = 0x200000 */
    mul %ecx             /* %eax = 2MB * ecx */
    or $0x83, %eax       /* Present + Writable + Huge Page (bit 7 = 1) */
    mov %eax, pd_table(,%ecx,8)
    movl $0, pd_table+4(,%ecx,8)

    inc %ecx
    cmp $512, %ecx
    jne .map_pd_loop
    ret

.boot_error:
    /* Output error code to serial port 0x3F8 for debugging */
    mov $0x3F8, %dx
    out %al, %dx
.error_hang:
    cli
    hlt
    jmp .error_hang

/* 64-bit Global Descriptor Table (GDT) */
.section .rodata
.align 8
gdt64:
    /* Null Descriptor */
    .quad 0
    /* Kernel Code 64 Descriptor (Selector 0x08): Executable, Readable, Conforming, Long Mode bit */
    .quad (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
    /* Kernel Data 64 Descriptor (Selector 0x10): Writable, Present */
    .quad (1 << 41) | (1 << 44) | (1 << 47)
gdt64_end:

gdt64_pointer:
    .word gdt64_end - gdt64 - 1
    .quad gdt64
